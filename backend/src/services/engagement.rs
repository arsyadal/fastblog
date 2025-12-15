use sqlx::PgPool;
use uuid::Uuid;
use std::error::Error;
use chrono::Utc;
use ammonia::clean;

use crate::models::engagement::{
    ClapRequest, CreateCommentRequest, Comment, CommentResponse, CommentAuthor, CommentInteractions
};

pub struct EngagementService {
    db: PgPool,
}

impl EngagementService {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    // Clap/unclap an article (toggle behavior - max 1 clap per user)
    pub async fn clap_article(
        &self,
        article_id: Uuid,
        user_id: Uuid,
        request: ClapRequest,
    ) -> Result<(i64, i32, bool), Box<dyn Error + Send + Sync>> {
        // Check if user already clapped
        let existing = sqlx::query!(
            "SELECT clap_count FROM claps WHERE user_id = $1 AND article_id = $2",
            user_id,
            article_id
        )
        .fetch_optional(&self.db)
        .await?;

        let (new_clap_count, is_clapped) = if let Some(_existing) = existing {
            // User already clapped - remove clap (unclap)
            sqlx::query!(
                "DELETE FROM claps WHERE user_id = $1 AND article_id = $2",
                user_id,
                article_id
            )
            .execute(&self.db)
            .await?;
            (0, false)
        } else {
            // User hasn't clapped - add clap
            sqlx::query!(
                "INSERT INTO claps (user_id, article_id, clap_count, created_at, updated_at) VALUES ($1, $2, 1, NOW(), NOW())",
                user_id,
                article_id
            )
            .execute(&self.db)
            .await?;
            (1, true)
        };

        // Update article claps_count (count distinct users, not sum of clap_count)
        let total_claps: i64 = sqlx::query_scalar!(
            "SELECT COUNT(*)::BIGINT FROM claps WHERE article_id = $1",
            article_id
        )
        .fetch_one(&self.db)
        .await?
        .unwrap_or(0);

        sqlx::query!(
            "UPDATE articles SET claps_count = $1 WHERE id = $2",
            total_claps,
            article_id
        )
        .execute(&self.db)
        .await?;

        Ok((total_claps, new_clap_count, is_clapped))
    }

    // Get user's clap status for an article (returns 1 if clapped, 0 if not)
    pub async fn get_user_clap_count(
        &self,
        article_id: Uuid,
        user_id: Uuid,
    ) -> Result<i32, Box<dyn Error + Send + Sync>> {
        let result = sqlx::query!(
            "SELECT clap_count FROM claps WHERE user_id = $1 AND article_id = $2",
            user_id,
            article_id
        )
        .fetch_optional(&self.db)
        .await?;

        Ok(if result.is_some() { 1 } else { 0 })
    }

    // Get comments for an article
    pub async fn get_comments(
        &self,
        article_id: Uuid,
        user_id: Option<Uuid>,
    ) -> Result<Vec<CommentResponse>, Box<dyn Error + Send + Sync>> {
        let comments = sqlx::query_as!(
            Comment,
            r#"
            SELECT 
                id, article_id, user_id, parent_id, content, content_html,
                claps_count, replies_count, is_author_reply, created_at, updated_at
            FROM comments
            WHERE article_id = $1 AND parent_id IS NULL
            ORDER BY created_at ASC
            "#,
            article_id
        )
        .fetch_all(&self.db)
        .await?;

        let mut comment_responses = Vec::new();
        for comment in comments {
            // Get author info
            let author = sqlx::query!(
                "SELECT id, username, display_name, avatar_url, is_verified FROM users WHERE id = $1",
                comment.user_id
            )
            .fetch_optional(&self.db)
            .await?
            .ok_or_else(|| format!("Author not found for comment {}", comment.id))?;

            // Get user interactions if logged in
            let user_interactions = if let Some(uid) = user_id {
                let has_clapped = sqlx::query_scalar!(
                    "SELECT EXISTS(SELECT 1 FROM claps WHERE user_id = $1 AND article_id = $2)",
                    uid,
                    article_id
                )
                .fetch_one(&self.db)
                .await?
                .unwrap_or(false);

                Some(CommentInteractions {
                    has_clapped,
                    clap_count: 0, // Could be implemented if we track comment claps
                })
            } else {
                None
            };

            // Get replies
            let replies = self.get_comment_replies(comment.id, user_id).await?;

            comment_responses.push(CommentResponse {
                id: comment.id,
                content: comment.content,
                content_html: comment.content_html,
                author: CommentAuthor {
                    id: author.id,
                    username: author.username,
                    display_name: author.display_name,
                    avatar_url: author.avatar_url,
                    is_verified: author.is_verified,
                },
                parent_id: comment.parent_id,
                claps_count: comment.claps_count,
                replies_count: comment.replies_count,
                is_author_reply: comment.is_author_reply,
                created_at: comment.created_at,
                updated_at: comment.updated_at,
                user_interactions,
                replies: if replies.is_empty() { None } else { Some(replies) },
            });
        }

        Ok(comment_responses)
    }

    // Get replies to a comment
    async fn get_comment_replies(
        &self,
        parent_id: Uuid,
        user_id: Option<Uuid>,
    ) -> Result<Vec<CommentResponse>, Box<dyn Error + Send + Sync>> {
        let replies = sqlx::query_as!(
            Comment,
            r#"
            SELECT 
                id, article_id, user_id, parent_id, content, content_html,
                claps_count, replies_count, is_author_reply, created_at, updated_at
            FROM comments
            WHERE parent_id = $1
            ORDER BY created_at ASC
            "#,
            parent_id
        )
        .fetch_all(&self.db)
        .await?;

        let mut reply_responses = Vec::new();
        for reply in replies {
            let author = sqlx::query!(
                "SELECT id, username, display_name, avatar_url, is_verified FROM users WHERE id = $1",
                reply.user_id
            )
            .fetch_optional(&self.db)
            .await?
            .ok_or_else(|| format!("Author not found for reply {}", reply.id))?;

            let user_interactions = if let Some(uid) = user_id {
                Some(CommentInteractions {
                    has_clapped: false,
                    clap_count: 0,
                })
            } else {
                None
            };

            reply_responses.push(CommentResponse {
                id: reply.id,
                content: reply.content,
                content_html: reply.content_html,
                author: CommentAuthor {
                    id: author.id,
                    username: author.username,
                    display_name: author.display_name,
                    avatar_url: author.avatar_url,
                    is_verified: author.is_verified,
                },
                parent_id: reply.parent_id,
                claps_count: reply.claps_count,
                replies_count: reply.replies_count,
                is_author_reply: reply.is_author_reply,
                created_at: reply.created_at,
                updated_at: reply.updated_at,
                user_interactions,
                replies: None, // Don't nest deeper than one level
            });
        }

        Ok(reply_responses)
    }

    // Create a comment
    pub async fn create_comment(
        &self,
        article_id: Uuid,
        user_id: Uuid,
        request: CreateCommentRequest,
    ) -> Result<CommentResponse, Box<dyn Error + Send + Sync>> {
        // Check if article exists and get author_id
        let article = sqlx::query!(
            "SELECT author_id FROM articles WHERE id = $1",
            article_id
        )
        .fetch_optional(&self.db)
        .await?
        .ok_or_else(|| "Article not found".to_string())?;

        // Sanitize HTML
        let content_html = clean(&request.content);

        let comment_id = Uuid::new_v4();
        let now = Utc::now();

        // Check if this is a reply to author
        let is_author_reply = if let Some(parent_id) = request.parent_id {
            // Check if parent comment is by article author
            let parent_comment = sqlx::query!(
                "SELECT user_id FROM comments WHERE id = $1",
                parent_id
            )
            .fetch_optional(&self.db)
            .await?;

            if let Some(parent) = parent_comment {
                parent.user_id == article.author_id
            } else {
                false
            }
        } else {
            false
        };

        // Insert comment
        sqlx::query!(
            r#"
            INSERT INTO comments (id, article_id, user_id, parent_id, content, content_html, is_author_reply, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
            comment_id,
            article_id,
            user_id,
            request.parent_id,
            request.content,
            content_html,
            is_author_reply,
            now,
            now
        )
        .execute(&self.db)
        .await?;

        // Update parent comment's replies_count if this is a reply
        if let Some(parent_id) = request.parent_id {
            sqlx::query!(
                "UPDATE comments SET replies_count = replies_count + 1 WHERE id = $1",
                parent_id
            )
            .execute(&self.db)
            .await?;
        }

        // Update article comments_count
        sqlx::query!(
            "UPDATE articles SET comments_count = comments_count + 1 WHERE id = $1",
            article_id
        )
        .execute(&self.db)
        .await?;

        // Get author info
        let author = sqlx::query!(
            "SELECT id, username, display_name, avatar_url, is_verified FROM users WHERE id = $1",
            user_id
        )
        .fetch_optional(&self.db)
        .await?
        .ok_or_else(|| "User not found".to_string())?;

        Ok(CommentResponse {
            id: comment_id,
            content: request.content,
            content_html,
            author: CommentAuthor {
                id: author.id,
                username: author.username,
                display_name: author.display_name,
                avatar_url: author.avatar_url,
                is_verified: author.is_verified,
            },
            parent_id: request.parent_id,
            claps_count: 0,
            replies_count: 0,
            is_author_reply,
            created_at: now,
            updated_at: now,
            user_interactions: None,
            replies: None,
        })
    }
}
