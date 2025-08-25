use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

// Claps (Medium's signature feature)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Clap {
    pub id: Uuid,
    pub user_id: Uuid,
    pub article_id: Uuid,
    pub clap_count: i32, // Max 50 per user per article
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ClapRequest {
    #[validate(range(min = 1, max = 50, message = "Clap count must be between 1 and 50"))]
    pub clap_count: i32,
}

// Comments
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Comment {
    pub id: Uuid,
    pub article_id: Uuid,
    pub user_id: Uuid,
    pub parent_id: Option<Uuid>, // For nested comments
    pub content: String,
    pub content_html: String,
    pub claps_count: i32,
    pub replies_count: i32,
    pub is_author_reply: bool, // If article author replied
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateCommentRequest {
    #[validate(length(min = 1, max = 2000, message = "Comment must be between 1 and 2000 characters"))]
    pub content: String,
    pub parent_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateCommentRequest {
    #[validate(length(min = 1, max = 2000, message = "Comment must be between 1 and 2000 characters"))]
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct CommentResponse {
    pub id: Uuid,
    pub content: String,
    pub content_html: String,
    pub author: CommentAuthor,
    pub parent_id: Option<Uuid>,
    pub claps_count: i32,
    pub replies_count: i32,
    pub is_author_reply: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub user_interactions: Option<CommentInteractions>,
    pub replies: Option<Vec<CommentResponse>>, // For nested replies
}

#[derive(Debug, Serialize)]
pub struct CommentAuthor {
    pub id: Uuid,
    pub username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub is_verified: bool,
}

#[derive(Debug, Serialize)]
pub struct CommentInteractions {
    pub has_clapped: bool,
    pub clap_count: i32,
}

// Bookmarks
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Bookmark {
    pub id: Uuid,
    pub user_id: Uuid,
    pub article_id: Uuid,
    pub created_at: DateTime<Utc>,
}

// Highlights (text selections with notes)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Highlight {
    pub id: Uuid,
    pub user_id: Uuid,
    pub article_id: Uuid,
    pub selected_text: String,
    pub note: Option<String>,
    pub start_position: i32,
    pub end_position: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateHighlightRequest {
    #[validate(length(min = 1, max = 1000, message = "Selected text must be between 1 and 1000 characters"))]
    pub selected_text: String,
    
    #[validate(length(max = 500, message = "Note cannot exceed 500 characters"))]
    pub note: Option<String>,
    
    #[validate(range(min = 0, message = "Start position must be non-negative"))]
    pub start_position: i32,
    
    #[validate(range(min = 0, message = "End position must be non-negative"))]
    pub end_position: i32,
}

#[derive(Debug, Serialize)]
pub struct HighlightResponse {
    pub id: Uuid,
    pub selected_text: String,
    pub note: Option<String>,
    pub start_position: i32,
    pub end_position: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Article Views (for analytics)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ArticleView {
    pub id: Uuid,
    pub article_id: Uuid,
    pub user_id: Option<Uuid>, // Can be anonymous
    pub ip_address: String,
    pub user_agent: Option<String>,
    pub reading_time_seconds: Option<i32>,
    pub scroll_percentage: Option<f32>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct RecordViewRequest {
    pub reading_time_seconds: Option<i32>,
    pub scroll_percentage: Option<f32>,
}

// Reading Lists (collections of bookmarked articles)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ReadingList {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub is_public: bool,
    pub articles_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateReadingListRequest {
    #[validate(length(min = 1, max = 100, message = "Name must be between 1 and 100 characters"))]
    pub name: String,
    
    #[validate(length(max = 500, message = "Description cannot exceed 500 characters"))]
    pub description: Option<String>,
    
    pub is_public: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ReadingListArticle {
    pub reading_list_id: Uuid,
    pub article_id: Uuid,
    pub added_at: DateTime<Utc>,
}

// Engagement Analytics
#[derive(Debug, Serialize)]
pub struct EngagementStats {
    pub total_claps: i64,
    pub total_comments: i64,
    pub total_bookmarks: i64,
    pub total_highlights: i64,
    pub total_views: i64,
    pub average_reading_time: f32,
    pub engagement_rate: f32, // (claps + comments + bookmarks) / views
}
