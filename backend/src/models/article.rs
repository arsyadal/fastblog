use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Article {
    pub id: Uuid,
    pub title: String,
    pub subtitle: Option<String>,
    pub content: String,
    pub content_html: String,
    pub excerpt: Option<String>,
    pub featured_image_url: Option<String>,
    pub author_id: Uuid,
    pub publication_id: Option<Uuid>,
    pub status: ArticleStatus,
    pub is_member_only: bool,
    pub paywall_position: Option<i32>,
    pub slug: String,
    pub tags: Vec<String>,
    pub reading_time_minutes: i32,
    pub claps_count: i64,
    pub comments_count: i32,
    pub bookmarks_count: i32,
    pub views_count: i64,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "article_status", rename_all = "lowercase")]
pub enum ArticleStatus {
    Draft,
    Published,
    Unlisted,
    Archived,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateArticleRequest {
    #[validate(length(min = 1, max = 200, message = "Title must be between 1 and 200 characters"))]
    pub title: String,
    
    #[validate(length(max = 300, message = "Subtitle cannot exceed 300 characters"))]
    pub subtitle: Option<String>,
    
    #[validate(length(min = 1, message = "Content is required"))]
    pub content: String,
    
    #[validate(length(max = 500, message = "Excerpt cannot exceed 500 characters"))]
    pub excerpt: Option<String>,
    
    #[validate(url(message = "Invalid featured image URL"))]
    pub featured_image_url: Option<String>,
    
    pub publication_id: Option<Uuid>,
    pub is_member_only: Option<bool>,
    pub paywall_position: Option<i32>,
    
    #[validate(length(max = 5, message = "Maximum 5 tags allowed"))]
    pub tags: Option<Vec<String>>,
    
    pub status: Option<ArticleStatus>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateArticleRequest {
    #[validate(length(min = 1, max = 200, message = "Title must be between 1 and 200 characters"))]
    pub title: Option<String>,
    
    #[validate(length(max = 300, message = "Subtitle cannot exceed 300 characters"))]
    pub subtitle: Option<String>,
    
    pub content: Option<String>,
    
    #[validate(url(message = "Invalid featured image URL"))]
    pub featured_image_url: Option<String>,
    
    pub publication_id: Option<Uuid>,
    pub is_member_only: Option<bool>,
    pub paywall_position: Option<i32>,
    
    #[validate(length(max = 5, message = "Maximum 5 tags allowed"))]
    pub tags: Option<Vec<String>>,
    
    pub status: Option<ArticleStatus>,
}

#[derive(Debug, Serialize)]
pub struct ArticleResponse {
    pub id: Uuid,
    pub title: String,
    pub subtitle: Option<String>,
    pub content: String,
    pub content_html: String,
    pub excerpt: Option<String>,
    pub featured_image_url: Option<String>,
    pub author: ArticleAuthor,
    pub publication: Option<ArticlePublication>,
    pub status: ArticleStatus,
    pub is_member_only: bool,
    pub paywall_position: Option<i32>,
    pub slug: String,
    pub tags: Vec<String>,
    pub reading_time_minutes: i32,
    pub claps_count: i64,
    pub comments_count: i32,
    pub bookmarks_count: i32,
    pub views_count: i64,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub user_interactions: Option<UserInteractions>,
}

#[derive(Debug, Serialize)]
pub struct ArticleListResponse {
    pub articles: Vec<ArticleResponse>,
    pub total: u64,
    pub limit: i64,
    pub offset: i64,
}

#[derive(Debug, Serialize)]
pub struct ArticleAuthor {
    pub id: Uuid,
    pub username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub followers_count: i32,
    pub is_verified: bool,
}

#[derive(Debug, Serialize)]
pub struct ArticlePublication {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub logo_url: Option<String>,
    pub followers_count: i32,
}

#[derive(Debug, Serialize)]
pub struct UserInteractions {
    pub has_clapped: bool,
    pub clap_count: i32,
    pub has_bookmarked: bool,
    pub is_following_author: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ArticleQueryParams {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub tag: Option<String>,
    pub author: Option<String>,
    pub publication: Option<String>,
    pub status: Option<ArticleStatus>,
    pub sort: Option<ArticleSortBy>,
    pub search: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ArticleSortBy {
    #[serde(rename = "recent")]
    Recent,
    #[serde(rename = "popular")]
    Popular,
    #[serde(rename = "trending")]
    Trending,
    #[serde(rename = "claps")]
    Claps,
    #[serde(rename = "views")]
    Views,
}

impl Default for ArticleQueryParams {
    fn default() -> Self {
        Self {
            page: Some(1),
            limit: Some(20),
            tag: None,
            author: None,
            publication: None,
            status: None,
            sort: Some(ArticleSortBy::Recent),
            search: None,
        }
    }
}
