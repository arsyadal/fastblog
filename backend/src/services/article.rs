use chrono::Utc;
use sqlx::{PgPool, Row};
use uuid::Uuid;
use std::error::Error;
use ammonia::clean;
use regex::Regex;
use lazy_static::lazy_static;

use crate::models::{
    Article, ArticleStatus, CreateArticleRequest, UpdateArticleRequest,
    ArticleResponse, ArticleListResponse, ArticleQueryParams, ArticleSortBy
};

lazy_static! {
    static ref SLUG_REGEX: Regex = Regex::new(r"[^a-zA-Z0-9\-]").unwrap();
    static ref HTML_TAG_REGEX: Regex = Regex::new(r"<[^>]*>").unwrap();
}

pub struct ArticleService {
    db: PgPool,
}

impl ArticleService {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    // Generate URL-friendly slug from title
    fn generate_slug(&self, title: &str) -> String {
        let slug = title
            .to_lowercase()
            .replace(' ', "-")
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-')
            .collect::<String>();
        
        // Remove multiple consecutive dashes
        let slug = Regex::new(r"-+").unwrap().replace_all(&slug, "-");
        
        // Remove leading/trailing dashes
        slug.trim_matches('-').to_string()
    }

    // Generate reading time estimate (average 200 words per minute)
    fn calculate_reading_time(&self, content: &str) -> i32 {
        let text = HTML_TAG_REGEX.replace_all(content, " ");
        let word_count = text.split_whitespace().count();
        std::cmp::max(1, (word_count / 200) as i32)
    }

    // Generate excerpt from content
    fn generate_excerpt(&self, content: &str, max_length: usize) -> String {
        let text = HTML_TAG_REGEX.replace_all(content, " ");
        let text = text.trim();
        
        if text.len() <= max_length {
            text.to_string()
        } else {
            let truncated = &text[..max_length];
            if let Some(last_space) = truncated.rfind(' ') {
                format!("{}...", &truncated[..last_space])
            } else {
                format!("{}...", truncated)
            }
        }
    }

    // Sanitize HTML content
    fn sanitize_html(&self, content: &str) -> String {
        clean(content)
    }

    pub async fn create_article(
        &self,
        author_id: Uuid,
        request: CreateArticleRequest,
    ) -> Result<ArticleResponse, Box<dyn Error + Send + Sync>> {
        let article_id = Uuid::new_v4();
        let slug = self.generate_slug(&request.title);
        let content_html = self.sanitize_html(&request.content);
        let reading_time = self.calculate_reading_time(&content_html);
        let excerpt = request.excerpt.unwrap_or_else(|| self.generate_excerpt(&content_html, 200));
        let now = Utc::now();

        // Ensure unique slug
        let unique_slug = self.ensure_unique_slug(&slug, None).await?;

        let article = sqlx::query_as!(
            Article,
            r#"
            INSERT INTO articles (
                id, title, subtitle, content, content_html, excerpt,
                featured_image_url, author_id, publication_id, status,
                is_member_only, paywall_position, slug, tags,
                reading_time_minutes, created_at, updated_at
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10::article_status,
                $11, $12, $13, $14, $15, $16, $17
            ) RETURNING 
                id, title, subtitle, content, content_html, excerpt,
                featured_image_url, author_id, publication_id,
                status as "status: ArticleStatus", is_member_only,
                paywall_position, slug, tags, reading_time_minutes,
                claps_count, comments_count, bookmarks_count, views_count,
                published_at, created_at, updated_at
            "#,
            article_id,
            request.title,
            request.subtitle,
            request.content,
            content_html,
            excerpt,
            request.featured_image_url,
            author_id,
            request.publication_id,
            ArticleStatus::Published as ArticleStatus,
            request.is_member_only.unwrap_or(false),
            request.paywall_position,
            unique_slug,
            &request.tags.unwrap_or_default(),
            reading_time,
            now,
            now
        )
        .fetch_one(&self.db)
        .await?;

        self.get_article_response(&article, Some(author_id)).await
    }

    pub async fn get_article_by_id(
        &self,
        article_id: Uuid,
        user_id: Option<Uuid>,
    ) -> Result<ArticleResponse, Box<dyn Error + Send + Sync>> {
        let article = sqlx::query_as!(
            Article,
            r#"
            SELECT 
                id, title, subtitle, content, content_html, excerpt,
                featured_image_url, author_id, publication_id,
                status as "status: ArticleStatus", is_member_only,
                paywall_position, slug, tags, reading_time_minutes,
                claps_count, comments_count, bookmarks_count, views_count,
                published_at, created_at, updated_at
            FROM articles 
            WHERE id = $1 AND (status = 'published' OR author_id = $2)
            "#,
            article_id,
            user_id
        )
        .fetch_optional(&self.db)
        .await?;

        match article {
            Some(article) => self.get_article_response(&article, user_id).await,
            None => Err("Article not found".into()),
        }
    }

    pub async fn get_article_by_slug(
        &self,
        slug: &str,
        user_id: Option<Uuid>,
    ) -> Result<ArticleResponse, Box<dyn Error + Send + Sync>> {
        let article = sqlx::query_as!(
            Article,
            r#"
            SELECT 
                id, title, subtitle, content, content_html, excerpt,
                featured_image_url, author_id, publication_id,
                status as "status: ArticleStatus", is_member_only,
                paywall_position, slug, tags, reading_time_minutes,
                claps_count, comments_count, bookmarks_count, views_count,
                published_at, created_at, updated_at
            FROM articles 
            WHERE slug = $1 AND (status = 'published' OR author_id = $2)
            "#,
            slug,
            user_id
        )
        .fetch_optional(&self.db)
        .await?;

        match article {
            Some(article) => self.get_article_response(&article, user_id).await,
            None => Err("Article not found".into()),
        }
    }

    pub async fn update_article(
        &self,
        article_id: Uuid,
        author_id: Uuid,
        request: UpdateArticleRequest,
    ) -> Result<ArticleResponse, Box<dyn Error + Send + Sync>> {
        // Check ownership
        let existing = sqlx::query!("SELECT author_id FROM articles WHERE id = $1", article_id)
            .fetch_optional(&self.db)
            .await?;

        match existing {
            Some(row) if row.author_id == author_id => {},
            Some(_) => return Err("Unauthorized to update this article".into()),
            None => return Err("Article not found".into()),
        }

        // Simple update for now - in production, use a proper query builder
        if let Some(title) = &request.title {
            sqlx::query!(
                "UPDATE articles SET title = $1, updated_at = NOW() WHERE id = $2 AND author_id = $3",
                title, article_id, author_id
            ).execute(&self.db).await?;
        }

        if let Some(content) = &request.content {
            let content_html = self.sanitize_html(content);
            let reading_time = self.calculate_reading_time(&content_html);
            
            sqlx::query!(
                "UPDATE articles SET content = $1, content_html = $2, reading_time_minutes = $3, updated_at = NOW() WHERE id = $4 AND author_id = $5",
                content, content_html, reading_time, article_id, author_id
            ).execute(&self.db).await?;
        }

        self.get_article_by_id(article_id, Some(author_id)).await
    }

    pub async fn delete_article(
        &self,
        article_id: Uuid,
        author_id: Uuid,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let result = sqlx::query!(
            "DELETE FROM articles WHERE id = $1 AND author_id = $2",
            article_id,
            author_id
        )
        .execute(&self.db)
        .await?;

        if result.rows_affected() == 0 {
            return Err("Article not found or unauthorized".into());
        }

        Ok(())
    }

    pub async fn publish_article(
        &self,
        article_id: Uuid,
        author_id: Uuid,
    ) -> Result<ArticleResponse, Box<dyn Error + Send + Sync>> {
        let result = sqlx::query!(
            r#"
            UPDATE articles 
            SET status = 'published', published_at = NOW(), updated_at = NOW()
            WHERE id = $1 AND author_id = $2 AND status = 'draft'
            "#,
            article_id,
            author_id
        )
        .execute(&self.db)
        .await?;

        if result.rows_affected() == 0 {
            return Err("Article not found, unauthorized, or already published".into());
        }

        self.get_article_by_id(article_id, Some(author_id)).await
    }

    pub async fn get_articles(
        &self,
        params: ArticleQueryParams,
        user_id: Option<Uuid>,
    ) -> Result<ArticleListResponse, Box<dyn Error + Send + Sync>> {
        let limit = params.limit.unwrap_or(20).min(100);
        let page = params.page.unwrap_or(1).max(1);
        let offset = (page - 1) * limit;
        
        // Simple query for testing
        let articles = sqlx::query_as!(
            Article,
            r#"
            SELECT 
                id, title, subtitle, content, content_html, excerpt, featured_image_url, 
                author_id, publication_id, status as "status: ArticleStatus", 
                is_member_only, paywall_position, slug, tags, reading_time_minutes, 
                claps_count, comments_count, bookmarks_count, views_count, 
                published_at, created_at, updated_at 
            FROM articles 
            WHERE status = 'published'
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
            limit,
            offset
        )
        .fetch_all(&self.db)
        .await?;

        let total = sqlx::query_scalar!("SELECT COUNT(*) FROM articles WHERE status = 'published'")
            .fetch_one(&self.db)
            .await?
            .unwrap_or(0);

        Ok(ArticleListResponse {
            articles: futures::future::try_join_all(
                articles.into_iter().map(|article| async move {
                    self.get_article_response(&article, user_id).await
                })
            ).await?,
            total: total as u64,
            limit,
            offset,
        })
    }

    async fn ensure_unique_slug(
        &self,
        base_slug: &str,
        exclude_id: Option<Uuid>,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        let mut slug = base_slug.to_string();
        let mut counter = 1;

        loop {
            let count = if let Some(id) = exclude_id {
                sqlx::query_scalar!(
                    "SELECT COUNT(*) FROM articles WHERE slug = $1 AND id != $2",
                    slug, id
                ).fetch_one(&self.db).await?.unwrap_or(0)
            } else {
                sqlx::query_scalar!(
                    "SELECT COUNT(*) FROM articles WHERE slug = $1",
                    slug
                ).fetch_one(&self.db).await?.unwrap_or(0)
            };

            if count == 0 {
                break;
            }

            slug = format!("{}-{}", base_slug, counter);
            counter += 1;
        }

        Ok(slug)
    }

    async fn get_article_response(
        &self,
        article: &Article,
        user_id: Option<Uuid>,
    ) -> Result<ArticleResponse, Box<dyn Error + Send + Sync>> {
        // Get author info
        let author = sqlx::query!(
            "SELECT username, display_name, avatar_url, bio, followers_count, is_verified FROM users WHERE id = $1",
            article.author_id
        )
        .fetch_one(&self.db)
        .await?;

        // Get user interactions if user is logged in
        let user_interactions = if let Some(_uid) = user_id {
            // TODO: Implement actual queries when tables are ready
            Some(crate::models::UserInteractions {
                has_clapped: false,
                clap_count: 0,
                has_bookmarked: false,
                is_following_author: false,
            })
        } else {
            None
        };

        Ok(ArticleResponse {
            id: article.id,
            title: article.title.clone(),
            subtitle: article.subtitle.clone(),
            content: article.content.clone(),
            content_html: article.content_html.clone(),
            excerpt: article.excerpt.clone(),
            featured_image_url: article.featured_image_url.clone(),
            author: crate::models::ArticleAuthor {
                id: article.author_id,
                username: author.username,
                display_name: author.display_name,
                avatar_url: author.avatar_url,
                bio: author.bio,
                followers_count: author.followers_count,
                is_verified: author.is_verified,
            },
            publication: None, // TODO: Implement publications
            status: article.status.clone(),
            is_member_only: article.is_member_only,
            paywall_position: article.paywall_position,
            slug: article.slug.clone(),
            tags: article.tags.clone(),
            reading_time_minutes: article.reading_time_minutes,
            claps_count: article.claps_count,
            comments_count: article.comments_count,
            bookmarks_count: article.bookmarks_count,
            views_count: article.views_count,
            published_at: article.published_at,
            created_at: article.created_at,
            updated_at: article.updated_at,
            user_interactions,
        })
    }
}
