use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;
use std::error::Error;
use ammonia::clean;
use regex::Regex;
use lazy_static::lazy_static;

use crate::models::{
    Article, ArticleStatus, CreateArticleRequest, UpdateArticleRequest,
    ArticleResponse, ArticleListResponse, ArticleQueryParams
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

        let tags: Vec<String> = request.tags.unwrap_or_default();
        let categories: Vec<String> = request.categories.unwrap_or_default();
        
        // Use status from request, or default to Draft
        let status = request.status.unwrap_or(ArticleStatus::Draft);
        
        // Set published_at only if status is Published
        let published_at = if matches!(status, ArticleStatus::Published) {
            Some(now)
        } else {
            None
        };

        // Insert article
        sqlx::query!(
            r#"
            INSERT INTO articles (
                id, title, subtitle, content, content_html, excerpt,
                featured_image_url, author_id, publication_id, status,
                is_member_only, paywall_position, slug, tags, categories,
                reading_time_minutes, created_at, updated_at, published_at, is_featured, reads_count, views_count, claps_count, comments_count, bookmarks_count, last_auto_save, auto_save_version
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10::article_status,
                $11, $12, $13, $14, $15, $16, $17, $18, $19, FALSE, 0, 0, 0, 0, 0, $17, 1
            )
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
            status as ArticleStatus,
            request.is_member_only.unwrap_or(false),
            request.paywall_position,
            unique_slug,
            &tags[..],
            &categories[..],
            reading_time,
            now,
            now,
            published_at
        )
        .execute(&self.db)
        .await?;

        // Fetch and return the created article
        self.get_article_by_id(article_id, Some(author_id)).await
    }

    pub async fn get_article_by_id(
        &self,
        article_id: Uuid,
        user_id: Option<Uuid>,
    ) -> Result<ArticleResponse, Box<dyn Error + Send + Sync>> {
        let article = sqlx::query_as::<_, Article>(
            r#"
            SELECT 
                id, title, subtitle, content, content_html, excerpt,
                featured_image_url, author_id, publication_id,
                status, is_member_only, is_featured,
                paywall_position, slug, 
                COALESCE(tags, ARRAY[]::TEXT[])::TEXT[] as tags, 
                COALESCE(categories, ARRAY[]::TEXT[])::TEXT[] as categories, 
                reading_time_minutes,
                claps_count, comments_count, bookmarks_count, views_count, reads_count,
                published_at, created_at, updated_at, last_auto_save, auto_save_version
            FROM articles 
            WHERE id = $1 AND (status = 'published' OR author_id = $2)
            "#,
        )
        .bind(article_id)
        .bind(user_id)
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
        let article = sqlx::query_as::<_, Article>(
            r#"
            SELECT 
                id, title, subtitle, content, content_html, excerpt,
                featured_image_url, author_id, publication_id,
                status, is_member_only, is_featured,
                paywall_position, slug, 
                COALESCE(tags, ARRAY[]::TEXT[])::TEXT[] as tags, 
                COALESCE(categories, ARRAY[]::TEXT[])::TEXT[] as categories, 
                reading_time_minutes,
                claps_count, comments_count, bookmarks_count, views_count, reads_count,
                published_at, created_at, updated_at, last_auto_save, auto_save_version
            FROM articles 
            WHERE slug = $1 AND (status = 'published' OR author_id = $2)
            "#,
        )
        .bind(slug)
        .bind(user_id)
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
        
        // Build the query based on parameters
        let (articles, total) = if let Some(author) = &params.author {
            // If author is specified, filter by author_id
            if let Ok(author_uuid) = Uuid::parse_str(author) {
                let articles = sqlx::query_as::<_, Article>(
                    r#"
                    SELECT 
                        id, title, subtitle, content, content_html, excerpt, featured_image_url, 
                        author_id, publication_id, status, 
                        is_member_only, is_featured, paywall_position, slug, 
                        COALESCE(tags, ARRAY[]::TEXT[])::TEXT[] as tags, 
                        COALESCE(categories, ARRAY[]::TEXT[])::TEXT[] as categories, 
                        reading_time_minutes, 
                        claps_count, comments_count, bookmarks_count, views_count, reads_count,
                        published_at, created_at, updated_at, last_auto_save, auto_save_version
                    FROM articles 
                    WHERE status = 'published' AND author_id = $1
                    ORDER BY created_at DESC
                    LIMIT $2 OFFSET $3
                    "#,
                )
                .bind(author_uuid)
                .bind(limit)
                .bind(offset)
                .fetch_all(&self.db)
                .await?;

                let total = sqlx::query_scalar!(
                    "SELECT COUNT(*) FROM articles WHERE status = 'published' AND author_id = $1",
                    author_uuid
                )
                .fetch_one(&self.db)
                .await?
                .unwrap_or(0);

                (articles, total)
            } else {
                // Invalid UUID, return empty result
                (vec![], 0)
            }
        } else if let Some(category) = &params.category {
            // Filter by category
            let articles = sqlx::query_as::<_, Article>(
                r#"
                SELECT 
                    id, title, subtitle, content, content_html, excerpt, featured_image_url, 
                    author_id, publication_id, status, 
                    is_member_only, is_featured, paywall_position, slug, 
                    COALESCE(tags, ARRAY[]::TEXT[])::TEXT[] as tags, 
                    COALESCE(categories, ARRAY[]::TEXT[])::TEXT[] as categories, 
                    reading_time_minutes, 
                    claps_count, comments_count, bookmarks_count, views_count, reads_count,
                    published_at, created_at, updated_at, last_auto_save, auto_save_version
                FROM articles 
                WHERE status = 'published' AND $1 = ANY(categories)
                ORDER BY created_at DESC
                LIMIT $2 OFFSET $3
                "#,
            )
            .bind(category)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.db)
            .await?;

            let total = sqlx::query_scalar!(
                "SELECT COUNT(*) FROM articles WHERE status = 'published' AND $1 = ANY(categories)",
                category
            )
            .fetch_one(&self.db)
            .await?
            .unwrap_or(0);

            (articles, total)
        } else if let Some(tag) = &params.tag {
            // Filter by tag
            let articles = sqlx::query_as::<_, Article>(
                r#"
                SELECT 
                    id, title, subtitle, content, content_html, excerpt, featured_image_url, 
                    author_id, publication_id, status, 
                    is_member_only, is_featured, paywall_position, slug, 
                    COALESCE(tags, ARRAY[]::TEXT[])::TEXT[] as tags, 
                    COALESCE(categories, ARRAY[]::TEXT[])::TEXT[] as categories, 
                    reading_time_minutes, 
                    claps_count, comments_count, bookmarks_count, views_count, reads_count,
                    published_at, created_at, updated_at, last_auto_save, auto_save_version
                FROM articles 
                WHERE status = 'published' AND $1 = ANY(tags)
                ORDER BY created_at DESC
                LIMIT $2 OFFSET $3
                "#,
            )
            .bind(tag)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.db)
            .await?;

            let total = sqlx::query_scalar!(
                "SELECT COUNT(*) FROM articles WHERE status = 'published' AND $1 = ANY(tags)",
                tag
            )
            .fetch_one(&self.db)
            .await?
            .unwrap_or(0);

            (articles, total)
        } else {
            // No author filter, get all published articles
            let articles = sqlx::query_as::<_, Article>(
                r#"
                SELECT 
                    id, title, subtitle, content, content_html, excerpt, featured_image_url, 
                    author_id, publication_id, status, 
                    is_member_only, is_featured, paywall_position, slug, 
                    COALESCE(tags, ARRAY[]::TEXT[])::TEXT[] as tags, 
                    COALESCE(categories, ARRAY[]::TEXT[])::TEXT[] as categories, 
                    reading_time_minutes, 
                    claps_count, comments_count, bookmarks_count, views_count, reads_count,
                    published_at, created_at, updated_at, last_auto_save, auto_save_version
                FROM articles 
                WHERE status = 'published'
                ORDER BY created_at DESC
                LIMIT $1 OFFSET $2
                "#,
            )
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.db)
            .await?;

            let total = sqlx::query_scalar!("SELECT COUNT(*) FROM articles WHERE status = 'published'")
                .fetch_one(&self.db)
                .await?
                .unwrap_or(0);

            (articles, total)
        };

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

    pub async fn get_article_response(
        &self,
        article: &Article,
        user_id: Option<Uuid>,
    ) -> Result<ArticleResponse, Box<dyn Error + Send + Sync>> {
        // Get author info
        let author = sqlx::query!(
            "SELECT username, display_name, avatar_url, bio, followers_count, is_verified FROM users WHERE id = $1",
            article.author_id
        )
        .fetch_optional(&self.db)
        .await?
        .ok_or_else(|| format!("Author not found for article {}", article.id))?;

        // Get user interactions if user is logged in
        let user_interactions = if let Some(uid) = user_id {
            use crate::services::engagement::EngagementService;
            let engagement_service = EngagementService::new(self.db.clone());
            
            let user_clap_count = engagement_service.get_user_clap_count(article.id, uid).await.unwrap_or(0);
            let has_clapped = user_clap_count > 0;
            
            // Check if user is following author
            let is_following_author = sqlx::query_scalar!(
                "SELECT EXISTS(SELECT 1 FROM user_follows WHERE follower_id = $1 AND following_id = $2)",
                uid,
                article.author_id
            )
            .fetch_one(&self.db)
            .await?
            .unwrap_or(false);
            
            // Check if bookmarked
            let has_bookmarked = sqlx::query_scalar!(
                "SELECT EXISTS(SELECT 1 FROM bookmarks WHERE user_id = $1 AND article_id = $2)",
                uid,
                article.id
            )
            .fetch_one(&self.db)
            .await?
            .unwrap_or(false);
            
            Some(crate::models::UserInteractions {
                has_clapped,
                clap_count: user_clap_count,
                has_bookmarked,
                is_following_author,
            })
        } else {
            None
        };

        // Generate share URL and metadata
        let frontend_url = std::env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:3003".to_string());
        let share_url = format!("{}/article/{}", frontend_url, article.slug);
        let share_title = article.title.clone();
        let share_description = article.excerpt.clone()
            .or_else(|| article.subtitle.clone())
            .unwrap_or_else(|| {
                // Generate from content if no excerpt
                let text = HTML_TAG_REGEX.replace_all(&article.content_html, " ");
                let text = text.trim();
                if text.len() > 160 {
                    format!("{}...", &text[..157])
                } else {
                    text.to_string()
                }
            });

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
            share_url,
            share_title,
            share_description,
        })
    }

    pub async fn auto_save_draft(
        &self,
        author_id: Uuid,
        request: &crate::models::AutoSaveDraftRequest,
    ) -> Result<Uuid, Box<dyn Error + Send + Sync>> {
        let now = chrono::Utc::now();
        
        if let Some(article_id) = request.article_id {
            // Update existing draft
            sqlx::query!(
                r#"
                UPDATE articles 
                SET 
                    title = COALESCE($1, title),
                    subtitle = $2,
                    content = $3,
                    content_html = $3,
                    excerpt = $4,
                    featured_image_url = $5,
                    tags = COALESCE($6, tags),
                    categories = COALESCE($7, categories),
                    is_member_only = COALESCE($8, is_member_only),
                    paywall_position = $9,
                    updated_at = $10,
                    last_auto_save = $10,
                    auto_save_version = auto_save_version + 1
                WHERE id = $11 AND author_id = $12 AND status = 'draft'
                "#,
                request.title,
                request.subtitle,
                request.content,
                request.excerpt,
                request.featured_image_url,
                request.tags.as_ref().map(|v| &**v),
                request.categories.as_ref().map(|v| &**v),
                request.is_member_only,
                request.paywall_position,
                now,
                article_id,
                author_id
            )
            .execute(&self.db)
            .await?;
            
            Ok(article_id)
        } else {
            // Create new draft
            let article_id = Uuid::new_v4();
            let slug = self.ensure_unique_slug("draft", None).await?;
            
            sqlx::query!(
                r#"
                INSERT INTO articles (
                    id, title, subtitle, content, content_html, excerpt, featured_image_url,
                    author_id, status, tags, categories, is_member_only, paywall_position,
                    slug, reading_time_minutes, created_at, updated_at, last_auto_save, auto_save_version, is_featured, reads_count, views_count, claps_count, comments_count, bookmarks_count
                ) VALUES (
                    $1, $2, $3, $4, $4, $5, $6, $7, 'draft', $8, $9, $10, $11, $12, $13, $14, $14, $14, 1, FALSE, 0, 0, 0, 0, 0
                )
                "#,
                article_id,
                request.title,
                request.subtitle,
                request.content,
                request.excerpt,
                request.featured_image_url,
                author_id,
                request.tags.as_ref().map(|v| &**v),
                request.categories.as_ref().map(|v| &**v),
                request.is_member_only,
                request.paywall_position,
                slug,
                self.calculate_reading_time(&request.content),
                now
            )
            .execute(&self.db)
            .await?;
            
            Ok(article_id)
        }
    }

    pub async fn get_article_stats(
        &self,
        article_id: Uuid,
    ) -> Result<crate::models::ArticleStats, Box<dyn Error + Send + Sync>> {
        let article = sqlx::query!(
            r#"
            SELECT 
                id, title, views_count, reads_count, claps_count, comments_count, 
                bookmarks_count, reading_time_minutes, published_at
            FROM articles 
            WHERE id = $1
            "#,
            article_id
        )
        .fetch_one(&self.db)
        .await?;
        
        let engagement_rate = if article.views_count > 0 {
            (article.reads_count as f64 / article.views_count as f64) * 100.0
        } else {
            0.0
        };
        
        Ok(crate::models::ArticleStats {
            article_id: article.id,
            title: article.title,
            views_count: article.views_count,
            reads_count: article.reads_count,
            claps_count: article.claps_count,
            comments_count: article.comments_count,
            bookmarks_count: article.bookmarks_count,
            reading_time_minutes: article.reading_time_minutes,
            published_at: article.published_at,
            engagement_rate,
        })
    }

    pub async fn get_author_stats(
        &self,
        author_id: Uuid,
    ) -> Result<crate::models::AuthorStats, Box<dyn Error + Send + Sync>> {
        // Get basic stats
        let stats = sqlx::query!(
            r#"
            SELECT 
                COUNT(*) as total_articles,
                COALESCE(SUM(views_count), 0) as total_views,
                COALESCE(SUM(reads_count), 0) as total_reads,
                COALESCE(SUM(claps_count), 0) as total_claps,
                COALESCE(SUM(comments_count), 0) as total_comments,
                COALESCE(SUM(bookmarks_count), 0) as total_bookmarks,
                COALESCE(AVG(reading_time_minutes), 0.0) as avg_reading_time
            FROM articles 
            WHERE author_id = $1 AND status = 'published'
            "#,
            author_id
        )
        .fetch_one(&self.db)
        .await?;
        
        // Get top articles by views
        let top_articles = sqlx::query!(
            r#"
            SELECT 
                id, title, views_count, reads_count, claps_count, comments_count, 
                bookmarks_count, reading_time_minutes, published_at
            FROM articles 
            WHERE author_id = $1 AND status = 'published'
            ORDER BY views_count DESC
            LIMIT 5
            "#,
            author_id
        )
        .fetch_all(&self.db)
        .await?;
        
        let top_articles_stats: Vec<crate::models::ArticleStats> = top_articles
            .into_iter()
            .map(|article| {
                let engagement_rate = if article.views_count > 0 {
                    (article.reads_count as f64 / article.views_count as f64) * 100.0
                } else {
                    0.0
                };
                
                crate::models::ArticleStats {
                    article_id: article.id,
                    title: article.title,
                    views_count: article.views_count,
                    reads_count: article.reads_count,
                    claps_count: article.claps_count,
                    comments_count: article.comments_count,
                    bookmarks_count: article.bookmarks_count,
                    reading_time_minutes: article.reading_time_minutes,
                    published_at: article.published_at,
                    engagement_rate,
                }
            })
            .collect();
        
        // Convert BigDecimal to i64/f64
        
        let total_views = stats.total_views
            .as_ref()
            .and_then(|v| v.to_string().parse::<i64>().ok())
            .unwrap_or(0i64);
        let total_reads = stats.total_reads
            .as_ref()
            .and_then(|v| v.to_string().parse::<i64>().ok())
            .unwrap_or(0i64);
        let total_claps = stats.total_claps
            .as_ref()
            .and_then(|v| v.to_string().parse::<i64>().ok())
            .unwrap_or(0i64);
        let total_comments = stats.total_comments
            .as_ref()
            .and_then(|v| v.to_string().parse::<i32>().ok())
            .unwrap_or(0i32);
        let total_bookmarks = stats.total_bookmarks
            .as_ref()
            .and_then(|v| v.to_string().parse::<i32>().ok())
            .unwrap_or(0i32);
        let avg_reading_time = stats.avg_reading_time
            .as_ref()
            .and_then(|v| v.to_string().parse::<f64>().ok())
            .unwrap_or(0.0);
        let total_articles = stats.total_articles
            .as_ref()
            .and_then(|v| v.to_string().parse::<i32>().ok())
            .unwrap_or(0i32);
        
        let average_engagement_rate = if total_views > 0 {
            (total_reads as f64 / total_views as f64) * 100.0
        } else {
            0.0
        };
        
        Ok(crate::models::AuthorStats {
            total_articles,
            total_views,
            total_reads,
            total_claps,
            total_comments,
            total_bookmarks,
            average_reading_time: avg_reading_time,
            average_engagement_rate,
            top_articles: top_articles_stats,
        })
    }

    pub async fn record_view(&self, article_id: Uuid, _user_id: Option<Uuid>) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Increment view count
        sqlx::query!(
            "UPDATE articles SET views_count = views_count + 1 WHERE id = $1",
            article_id
        )
        .execute(&self.db)
        .await?;
        
        // TODO: Record detailed view analytics (user_id, timestamp, etc.)
        
        Ok(())
    }

    pub async fn record_read(&self, article_id: Uuid, _user_id: Option<Uuid>) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Increment read count
        sqlx::query!(
            "UPDATE articles SET reads_count = reads_count + 1 WHERE id = $1",
            article_id
        )
        .execute(&self.db)
        .await?;
        
        // TODO: Record detailed read analytics (user_id, timestamp, reading duration, etc.)
        
        Ok(())
    }

    pub async fn toggle_featured(&self, article_id: Uuid, _user_id: Option<Uuid>) -> Result<bool, Box<dyn Error + Send + Sync>> {
        // Get current featured status
        let current_status: bool = sqlx::query_scalar!(
            "SELECT is_featured FROM articles WHERE id = $1",
            article_id
        )
        .fetch_one(&self.db)
        .await?;

        let new_status = !current_status;
        
        // Update featured status
        sqlx::query!(
            "UPDATE articles SET is_featured = $1, updated_at = $2 WHERE id = $3",
            new_status,
            chrono::Utc::now(),
            article_id
        )
        .execute(&self.db)
        .await?;

        Ok(new_status)
    }

    // Get user feed - articles from users that the current user follows
    pub async fn get_user_feed(
        &self,
        user_id: Uuid,
        page: Option<i64>,
        limit: Option<i64>,
    ) -> Result<ArticleListResponse, Box<dyn Error + Send + Sync>> {
        let limit = limit.unwrap_or(20).min(100);
        let page = page.unwrap_or(1).max(1);
        let offset = (page - 1) * limit;

        // Get articles from users that this user follows
        let limit_i64 = limit as i64;
        let offset_i64 = offset as i64;
        let articles = sqlx::query_as::<_, Article>(
            r#"
            SELECT 
                a.id, a.title, a.subtitle, a.content, a.content_html, a.excerpt, 
                a.featured_image_url, a.author_id, a.publication_id, 
                a.status, 
                a.is_member_only, a.is_featured, a.paywall_position, a.slug, 
                COALESCE(a.tags, ARRAY[]::TEXT[])::TEXT[] as tags, 
                COALESCE(a.categories, ARRAY[]::TEXT[])::TEXT[] as categories, 
                a.reading_time_minutes, 
                a.claps_count, a.comments_count, a.bookmarks_count, a.views_count, a.reads_count,
                a.published_at, a.created_at, a.updated_at, a.last_auto_save, a.auto_save_version
            FROM articles a
            INNER JOIN user_follows uf ON a.author_id = uf.following_id
            WHERE uf.follower_id = $1 
                AND a.status = 'published'
                AND a.published_at IS NOT NULL
            ORDER BY a.published_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(user_id)
        .bind(limit_i64)
        .bind(offset_i64)
        .fetch_all(&self.db)
        .await?;

        let total = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*)
            FROM articles a
            INNER JOIN user_follows uf ON a.author_id = uf.following_id
            WHERE uf.follower_id = $1 
                AND a.status = 'published'
                AND a.published_at IS NOT NULL
            "#,
            user_id
        )
        .fetch_one(&self.db)
        .await?
        .unwrap_or(0) as u64;

        let mut article_responses = Vec::new();
        for article in articles {
            match self.get_article_response(&article, Some(user_id)).await {
                Ok(response) => article_responses.push(response),
                Err(e) => {
                    tracing::warn!("Failed to get article response for feed: {}", e);
                }
            }
        }

        Ok(ArticleListResponse {
            articles: article_responses,
            total,
            limit: limit as i64,
            offset: offset as i64,
        })
    }

    // Get trending articles based on engagement (claps, comments, views, reads)
    pub async fn get_trending_articles(
        &self,
        user_id: Option<Uuid>,
        page: Option<i64>,
        limit: Option<i64>,
        time_window_hours: Option<i32>,
    ) -> Result<ArticleListResponse, Box<dyn Error + Send + Sync>> {
        let limit = limit.unwrap_or(20).min(100);
        let page = page.unwrap_or(1).max(1);
        let offset = (page - 1) * limit;
        let time_window = time_window_hours.unwrap_or(168); // Default: 7 days

        // Calculate engagement score: weighted combination of claps, comments, views, and reads
        // More recent articles get a boost
        let limit_i64 = limit as i64;
        let offset_i64 = offset as i64;
        let time_window_i32 = time_window as i32;
        let articles = sqlx::query_as::<_, Article>(
            r#"
            SELECT 
                a.id, a.title, a.subtitle, a.content, a.content_html, a.excerpt, 
                a.featured_image_url, a.author_id, a.publication_id, 
                a.status, 
                a.is_member_only, a.is_featured, a.paywall_position, a.slug, 
                COALESCE(a.tags, ARRAY[]::TEXT[])::TEXT[] as tags, 
                COALESCE(a.categories, ARRAY[]::TEXT[])::TEXT[] as categories, 
                a.reading_time_minutes, 
                a.claps_count, a.comments_count, a.bookmarks_count, a.views_count, a.reads_count,
                a.published_at, a.created_at, a.updated_at, a.last_auto_save, a.auto_save_version
            FROM articles a
            WHERE a.status = 'published'
                AND a.published_at IS NOT NULL
                AND a.published_at > NOW() - INTERVAL '1 hour' * $1
            ORDER BY 
                (
                    (a.claps_count * 3) + 
                    (a.comments_count * 2) + 
                    (a.reads_count * 1) + 
                    (a.views_count * 0.1) +
                    (EXTRACT(EPOCH FROM (NOW() - a.published_at)) / 3600 < 24)::int * 10
                ) DESC,
                a.published_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(time_window_i32)
        .bind(limit_i64)
        .bind(offset_i64)
        .fetch_all(&self.db)
        .await?;

        let total: Option<i64> = sqlx::query_scalar(
            r#"
            SELECT COUNT(*)
            FROM articles
            WHERE status = 'published'
                AND published_at IS NOT NULL
                AND published_at > NOW() - INTERVAL '1 hour' * $1
            "#,
        )
        .bind(time_window_i32)
        .fetch_one(&self.db)
        .await?;

        let total = total.unwrap_or(0) as u64;

        let mut article_responses = Vec::new();
        for article in articles {
            match self.get_article_response(&article, user_id).await {
                Ok(response) => article_responses.push(response),
                Err(e) => {
                    tracing::warn!("Failed to get article response for trending: {}", e);
                }
            }
        }

        Ok(ArticleListResponse {
            articles: article_responses,
            total,
            limit: limit as i64,
            offset: offset as i64,
        })
    }
}
