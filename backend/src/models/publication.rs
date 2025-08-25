use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Publication {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub logo_url: Option<String>,
    pub banner_url: Option<String>,
    pub website_url: Option<String>,
    pub custom_domain: Option<String>,
    pub owner_id: Uuid,
    pub slug: String,
    pub is_verified: bool,
    pub is_accepting_submissions: bool,
    pub followers_count: i32,
    pub articles_count: i32,
    pub writers_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PublicationMember {
    pub id: Uuid,
    pub publication_id: Uuid,
    pub user_id: Uuid,
    pub role: PublicationRole,
    pub is_active: bool,
    pub joined_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "publication_role", rename_all = "lowercase")]
pub enum PublicationRole {
    Owner,
    Editor,
    Writer,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreatePublicationRequest {
    #[validate(length(min = 1, max = 100, message = "Name must be between 1 and 100 characters"))]
    pub name: String,
    
    #[validate(length(max = 500, message = "Description cannot exceed 500 characters"))]
    pub description: Option<String>,
    
    #[validate(url(message = "Invalid logo URL"))]
    pub logo_url: Option<String>,
    
    #[validate(url(message = "Invalid banner URL"))]
    pub banner_url: Option<String>,
    
    #[validate(url(message = "Invalid website URL"))]
    pub website_url: Option<String>,
    
    #[validate(length(min = 3, max = 50, message = "Slug must be between 3 and 50 characters"))]
    #[validate(regex(path = *SLUG_REGEX, message = "Slug can only contain letters, numbers, and hyphens"))]
    pub slug: String,
    
    pub is_accepting_submissions: Option<bool>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdatePublicationRequest {
    #[validate(length(min = 1, max = 100, message = "Name must be between 1 and 100 characters"))]
    pub name: Option<String>,
    
    #[validate(length(max = 500, message = "Description cannot exceed 500 characters"))]
    pub description: Option<String>,
    
    #[validate(url(message = "Invalid logo URL"))]
    pub logo_url: Option<String>,
    
    #[validate(url(message = "Invalid banner URL"))]
    pub banner_url: Option<String>,
    
    #[validate(url(message = "Invalid website URL"))]
    pub website_url: Option<String>,
    
    pub is_accepting_submissions: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct PublicationResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub logo_url: Option<String>,
    pub banner_url: Option<String>,
    pub website_url: Option<String>,
    pub custom_domain: Option<String>,
    pub owner: PublicationOwner,
    pub slug: String,
    pub is_verified: bool,
    pub is_accepting_submissions: bool,
    pub followers_count: i32,
    pub articles_count: i32,
    pub writers_count: i32,
    pub created_at: DateTime<Utc>,
    pub user_interactions: Option<PublicationInteractions>,
}

#[derive(Debug, Serialize)]
pub struct PublicationOwner {
    pub id: Uuid,
    pub username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PublicationInteractions {
    pub is_following: bool,
    pub is_member: bool,
    pub member_role: Option<PublicationRole>,
}

#[derive(Debug, Serialize)]
pub struct PublicationMemberResponse {
    pub id: Uuid,
    pub user: PublicationMemberUser,
    pub role: PublicationRole,
    pub is_active: bool,
    pub joined_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct PublicationMemberUser {
    pub id: Uuid,
    pub username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub articles_count: i32,
    pub followers_count: i32,
}

#[derive(Debug, Deserialize)]
pub struct InviteMemberRequest {
    pub email: String,
    pub role: PublicationRole,
    pub message: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMemberRoleRequest {
    pub role: PublicationRole,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PublicationFollow {
    pub user_id: Uuid,
    pub publication_id: Uuid,
    pub created_at: DateTime<Utc>,
}

// Publication submission (for writers to submit to publications)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PublicationSubmission {
    pub id: Uuid,
    pub publication_id: Uuid,
    pub article_id: Uuid,
    pub writer_id: Uuid,
    pub status: SubmissionStatus,
    pub editor_notes: Option<String>,
    pub reviewed_by: Option<Uuid>,
    pub submitted_at: DateTime<Utc>,
    pub reviewed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "submission_status", rename_all = "lowercase")]
pub enum SubmissionStatus {
    Pending,
    Approved,
    Rejected,
    RevisionRequested,
}

#[derive(Debug, Deserialize)]
pub struct SubmitArticleRequest {
    pub article_id: Uuid,
    pub message: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ReviewSubmissionRequest {
    pub status: SubmissionStatus,
    pub editor_notes: Option<String>,
}

// Regex for slug validation
lazy_static::lazy_static! {
    static ref SLUG_REGEX: regex::Regex = regex::Regex::new(r"^[a-z0-9-]+$").unwrap();
}
