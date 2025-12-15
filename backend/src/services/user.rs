use sqlx::PgPool;
use uuid::Uuid;
use anyhow::Result;

use crate::models::user::{User, UserResponse, UserType};

#[derive(Clone)]
pub struct UserService {
    db: PgPool,
}

impl UserService {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    pub async fn get_user_by_id(&self, user_id: &Uuid) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT 
                id as user_id,
                username,
                email,
                display_name,
                bio,
                avatar_url,
                user_type as "user_type: UserType",
                is_verified,
                followers_count,
                following_count,
                created_at,
                updated_at
            FROM users 
            WHERE id = $1
            "#,
            user_id
        )
        .fetch_optional(&self.db)
        .await?;

        Ok(user)
    }

    pub async fn get_user_by_username(&self, username: &str) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT 
                id as user_id,
                username,
                email,
                display_name,
                bio,
                avatar_url,
                user_type as "user_type: UserType",
                is_verified,
                followers_count,
                following_count,
                created_at,
                updated_at
            FROM users 
            WHERE username = $1
            "#,
            username
        )
        .fetch_optional(&self.db)
        .await?;

        Ok(user)
    }

    pub async fn update_avatar(&self, user_id: &Uuid, avatar_url: &str) -> Result<()> {
        let avatar_url = if avatar_url.is_empty() { None } else { Some(avatar_url) };
        
        sqlx::query!(
            "UPDATE users SET avatar_url = $1, updated_at = NOW() WHERE id = $2",
            avatar_url,
            user_id
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }

    pub async fn update_profile(
        &self,
        user_id: &Uuid,
        display_name: Option<&str>,
        bio: Option<&str>,
    ) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE users 
            SET 
                display_name = COALESCE($1, display_name),
                bio = COALESCE($2, bio),
                updated_at = NOW() 
            WHERE id = $3
            "#,
            display_name,
            bio,
            user_id
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }

    pub async fn get_user_profile(&self, username: &str) -> Result<Option<UserResponse>> {
        #[derive(sqlx::FromRow)]
        struct UserProfileRow {
            id: Uuid,
            username: String,
            display_name: Option<String>,
            bio: Option<String>,
            avatar_url: Option<String>,
            is_verified: bool,
            followers_count: i32,
            following_count: i32,
            created_at: chrono::DateTime<chrono::Utc>,
            user_type: UserType,
        }

        let user = sqlx::query_as!(
            UserProfileRow,
            r#"
            SELECT 
                id,
                username,
                display_name,
                bio,
                avatar_url,
                is_verified,
                followers_count,
                following_count,
                created_at,
                user_type as "user_type: _"
            FROM users 
            WHERE username = $1
            "#,
            username
        )
        .fetch_optional(&self.db)
        .await?;

        if let Some(user) = user {
            // Get articles count for this user
            let articles_count = sqlx::query_scalar!(
                "SELECT COUNT(*) FROM articles WHERE author_id = $1 AND status = 'published'",
                user.id
            )
            .fetch_one(&self.db)
            .await?
            .unwrap_or(0);

            // Get total claps received (simplified for now)
            let total_claps_received = 0i64;

            // Check if user is a member
            let is_member = user.user_type == UserType::Member || user.user_type == UserType::Writer;

            // Handle avatar URL - if it's a relative path, make it absolute
            let avatar_url = if let Some(url) = &user.avatar_url {
                if url.starts_with("uploads/") {
                    Some(format!("http://localhost:3001/{}", url))
                } else {
                    Some(url.clone())
                }
            } else {
                None
            };

            Ok(Some(UserResponse {
                id: user.id,
                email: "".to_string(), // We don't expose email in public profile
                username: user.username,
                display_name: user.display_name,
                bio: user.bio,
                avatar_url,
                user_type: user.user_type,
                is_verified: user.is_verified,
                is_member,
                followers_count: user.followers_count,
                following_count: user.following_count,
                articles_count: articles_count as i32,
                total_claps_received,
                created_at: user.created_at,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn follow_user(&self, follower_id: &Uuid, following_id: &Uuid) -> Result<()> {
        // Insert follow relationship
        sqlx::query!(
            r#"
            INSERT INTO user_follows (follower_id, following_id, created_at)
            VALUES ($1, $2, NOW())
            ON CONFLICT (follower_id, following_id) DO NOTHING
            "#,
            follower_id,
            following_id
        )
        .execute(&self.db)
        .await?;

        // Update follower counts
        sqlx::query!(
            "UPDATE users SET following_count = following_count + 1 WHERE id = $1",
            follower_id
        )
        .execute(&self.db)
        .await?;

        sqlx::query!(
            "UPDATE users SET followers_count = followers_count + 1 WHERE id = $1",
            following_id
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }

    pub async fn unfollow_user(&self, follower_id: &Uuid, following_id: &Uuid) -> Result<()> {
        // Delete follow relationship
        let result = sqlx::query!(
            "DELETE FROM user_follows WHERE follower_id = $1 AND following_id = $2",
            follower_id,
            following_id
        )
        .execute(&self.db)
        .await?;

        // Only update counts if a row was actually deleted
        if result.rows_affected() > 0 {
            sqlx::query!(
                "UPDATE users SET following_count = GREATEST(following_count - 1, 0) WHERE id = $1",
                follower_id
            )
            .execute(&self.db)
            .await?;

            sqlx::query!(
                "UPDATE users SET followers_count = GREATEST(followers_count - 1, 0) WHERE id = $1",
                following_id
            )
            .execute(&self.db)
            .await?;
        }

        Ok(())
    }

    pub async fn is_following(&self, follower_id: &Uuid, following_id: &Uuid) -> Result<bool> {
        let exists = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM user_follows WHERE follower_id = $1 AND following_id = $2)",
            follower_id,
            following_id
        )
        .fetch_one(&self.db)
        .await?;

        Ok(exists.unwrap_or(false))
    }

    // Get user recommendations - suggest users to follow
    // Based on: users with most followers, users with most articles, users followed by people you follow
    pub async fn get_user_recommendations(
        &self,
        user_id: &Uuid,
        limit: Option<i64>,
    ) -> Result<Vec<UserResponse>> {
        let limit = limit.unwrap_or(10).min(50) as i64;

        // Get users that:
        // 1. Are not the current user
        // 2. Are not already followed by the current user
        // 3. Have published articles
        // 4. Ordered by followers count and articles count
        let users = sqlx::query!(
            r#"
            SELECT 
                u.id,
                u.username,
                u.display_name,
                u.bio,
                u.avatar_url,
                u.user_type as "user_type: UserType",
                u.is_verified,
                u.followers_count,
                u.following_count,
                u.created_at,
                COUNT(DISTINCT a.id) as articles_count
            FROM users u
            LEFT JOIN articles a ON u.id = a.author_id AND a.status = 'published'
            WHERE u.id != $1
                AND NOT EXISTS (
                    SELECT 1 FROM user_follows uf 
                    WHERE uf.follower_id = $1 AND uf.following_id = u.id
                )
            GROUP BY u.id, u.username, u.display_name, u.bio, u.avatar_url, 
                     u.user_type, u.is_verified, u.followers_count, u.following_count, u.created_at
            HAVING COUNT(DISTINCT a.id) > 0
            ORDER BY u.followers_count DESC, articles_count DESC, u.created_at DESC
            LIMIT $2
            "#,
            user_id,
            limit
        )
        .fetch_all(&self.db)
        .await?;

        let mut recommendations = Vec::new();
        for user in users {
            let is_member = user.user_type == UserType::Member || user.user_type == UserType::Writer;
            
            // Handle avatar URL
            let avatar_url = if let Some(url) = &user.avatar_url {
                if url.starts_with("uploads/") {
                    Some(format!("http://localhost:3001/{}", url))
                } else {
                    Some(url.clone())
                }
            } else {
                None
            };

            recommendations.push(UserResponse {
                id: user.id,
                email: "".to_string(), // Don't expose email
                username: user.username,
                display_name: user.display_name,
                bio: user.bio,
                avatar_url,
                user_type: user.user_type,
                is_verified: user.is_verified,
                is_member,
                followers_count: user.followers_count,
                following_count: user.following_count,
                articles_count: user.articles_count.unwrap_or(0) as i32,
                total_claps_received: 0, // Could be calculated if needed
                created_at: user.created_at,
            });
        }

        Ok(recommendations)
    }
}
