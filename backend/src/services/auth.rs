use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use sqlx::Row;
use uuid::Uuid;

use crate::{
    config::Config,
    database::Database,
    models::{AuthResponse, CreateUserRequest, LoginRequest, User, UserType},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // User ID
    pub username: String,
    pub user_type: UserType,
    pub exp: usize, // Expiration time
    pub iat: usize, // Issued at
}

pub struct AuthService<'a> {
    db: &'a Database,
    config: &'a Config,
}

impl<'a> AuthService<'a> {
    pub fn new(db: &'a Database, config: &'a Config) -> Self {
        Self { db, config }
    }

    pub async fn register(&self, request: CreateUserRequest) -> Result<AuthResponse, Box<dyn std::error::Error + Send + Sync>> {
        // Check if user already exists
        let existing_user = sqlx::query("SELECT id FROM users WHERE email = $1 OR username = $2")
            .bind(&request.email)
            .bind(&request.username)
            .fetch_optional(&self.db.pool)
            .await?;

        if existing_user.is_some() {
            return Err("User with this email or username already exists".into());
        }

        // Hash password
        let password_hash = self.hash_password(&request.password)?;

        // Create user
        let user_id = Uuid::new_v4();
        let now = Utc::now();

        // First insert the user
        sqlx::query(
            r#"
            INSERT INTO users (
                id, email, username, display_name, password_hash, user_type,
                is_verified, followers_count, following_count, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            "#,
        )
        .bind(user_id)
        .bind(&request.email)
        .bind(&request.username)
        .bind(&request.display_name)
        .bind(password_hash)
        .bind(UserType::Free)
        .bind(false)
        .bind(0i32)
        .bind(0i32)
        .bind(now)
        .bind(now)
        .execute(&self.db.pool)
        .await?;

        // Then fetch the user
        let user = sqlx::query_as!(
            User,
            r#"SELECT id as user_id, email, username, display_name, bio, avatar_url, user_type as "user_type: UserType", 
                      is_verified, followers_count, following_count, created_at, updated_at
               FROM users WHERE id = $1"#,
            user_id
        )
        .fetch_one(&self.db.pool)
        .await?;

        // Generate JWT token
        let (token, expires_at) = self.generate_token(&user)?;

        Ok(AuthResponse {
            user: user.into(),
            token,
            expires_at,
        })
    }

    pub async fn login(&self, request: LoginRequest) -> Result<AuthResponse, Box<dyn std::error::Error + Send + Sync>> {
        // Find user by email or username
        // Check if input looks like an email (contains @)
        let is_email = request.email.contains('@');
        
        let user = if is_email {
            // Search by email
            sqlx::query_as!(
                User,
                r#"SELECT id as user_id, email, username, display_name, bio, avatar_url, user_type as "user_type: UserType", 
                          is_verified, followers_count, following_count, created_at, updated_at
                   FROM users WHERE email = $1"#,
                &request.email
            )
            .fetch_optional(&self.db.pool)
            .await?
        } else {
            // Search by username
            sqlx::query_as!(
                User,
                r#"SELECT id as user_id, email, username, display_name, bio, avatar_url, user_type as "user_type: UserType", 
                          is_verified, followers_count, following_count, created_at, updated_at
                   FROM users WHERE username = $1"#,
                &request.email
            )
            .fetch_optional(&self.db.pool)
            .await?
        };

        let user = user.ok_or("Invalid email/username or password")?;

        // Get password hash
        let password_hash_row = sqlx::query("SELECT password_hash FROM users WHERE id = $1")
            .bind(user.user_id)
            .fetch_one(&self.db.pool)
            .await?;
        
        let password_hash: String = password_hash_row.get("password_hash");

        // Verify password
        if !self.verify_password(&request.password, &password_hash)? {
            return Err("Invalid email/username or password".into());
        }

        // Update last login
        sqlx::query("UPDATE users SET updated_at = $1 WHERE id = $2")
            .bind(Utc::now())
            .bind(user.user_id)
            .execute(&self.db.pool)
            .await?;

        // Generate JWT token with appropriate duration based on remember_me
        let remember_me = request.remember_me.unwrap_or(false);
        let (token, expires_at) = self.generate_token_with_duration(&user, remember_me)?;

        Ok(AuthResponse {
            user: user.into(),
            token,
            expires_at,
        })
    }

    pub async fn verify_token(&self, token: &str) -> Result<Claims, Box<dyn std::error::Error + Send + Sync>> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.config.jwt_secret.as_ref()),
            &Validation::default(),
        )?;

        Ok(token_data.claims)
    }

    pub async fn get_user_by_id(&self, user_id: &str) -> Result<User, Box<dyn std::error::Error + Send + Sync>> {
        let user_uuid = Uuid::parse_str(user_id)?;
        
        let user = sqlx::query_as!(
            User,
            r#"SELECT id as user_id, email, username, display_name, bio, avatar_url, user_type as "user_type: UserType", 
                      is_verified, followers_count, following_count, created_at, updated_at
               FROM users WHERE id = $1"#,
            user_uuid
        )
        .fetch_one(&self.db.pool)
        .await?;

        Ok(user)
    }

    fn hash_password(&self, password: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| format!("Password hashing failed: {}", e))?
            .to_string();
        Ok(password_hash)
    }

    fn verify_password(&self, password: &str, hash: &str) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| format!("Invalid password hash: {}", e))?;
        let argon2 = Argon2::default();
        Ok(argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok())
    }

    fn generate_token(&self, user: &User) -> Result<(String, chrono::DateTime<Utc>), Box<dyn std::error::Error + Send + Sync>> {
        self.generate_token_with_duration(user, false)
    }

    fn generate_token_with_duration(&self, user: &User, remember_me: bool) -> Result<(String, chrono::DateTime<Utc>), Box<dyn std::error::Error + Send + Sync>> {
        let now = Utc::now();
        // If remember_me is true, token expires in 30 days, otherwise 24 hours
        let expires_at = if remember_me {
            now + Duration::days(30)
        } else {
            now + Duration::hours(24)
        };

        let claims = Claims {
            sub: user.user_id.to_string(),
            username: user.username.clone(),
            user_type: user.user_type.clone(),
            exp: expires_at.timestamp() as usize,
            iat: now.timestamp() as usize,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.config.jwt_secret.as_ref()),
        )?;

        Ok((token, expires_at))
    }
}
