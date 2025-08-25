use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub user_id: Uuid, // Alias for id in queries
    pub email: String,
    pub username: String,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub user_type: UserType,
    pub is_verified: bool,
    pub followers_count: i32,
    pub following_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "user_type", rename_all = "lowercase")]
pub enum UserType {
    Free,
    Member,
    Writer,
    Publication,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    
    #[validate(length(min = 3, max = 30, message = "Username must be between 3 and 30 characters"))]
    #[validate(regex(path = *USERNAME_REGEX, message = "Username can only contain letters, numbers, and underscores"))]
    pub username: String,
    
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,
    
    #[validate(length(max = 100, message = "Display name cannot exceed 100 characters"))]
    pub display_name: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserRequest {
    #[validate(length(max = 100, message = "Display name cannot exceed 100 characters"))]
    pub display_name: Option<String>,
    
    #[validate(length(max = 500, message = "Bio cannot exceed 500 characters"))]
    pub bio: Option<String>,
    
    #[validate(url(message = "Invalid avatar URL"))]
    pub avatar_url: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    
    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub user_type: UserType,
    pub is_verified: bool,
    pub is_member: bool,
    pub followers_count: i32,
    pub following_count: i32,
    pub articles_count: i32,
    pub total_claps_received: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub user: UserResponse,
    pub token: String,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserFollow {
    pub follower_id: Uuid,
    pub following_id: Uuid,
    pub created_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.user_id,
            email: user.email,
            username: user.username,
            display_name: user.display_name,
            bio: user.bio,
            avatar_url: user.avatar_url,
            user_type: user.user_type,
            is_verified: user.is_verified,
            is_member: false, // Default, we'll need to query this properly
            followers_count: user.followers_count,
            following_count: user.following_count,
            articles_count: 0, // Default, we'll need to query this properly  
            total_claps_received: 0, // Default, we'll need to query this properly
            created_at: user.created_at,
        }
    }
}

// Regex for username validation
lazy_static::lazy_static! {
    static ref USERNAME_REGEX: regex::Regex = regex::Regex::new(r"^[a-zA-Z0-9_]+$").unwrap();
}
