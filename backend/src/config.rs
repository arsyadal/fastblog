use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub port: u16,
    pub environment: Environment,
    pub cors_origins: Vec<String>,
    pub max_file_size: usize,
    pub redis_url: Option<String>,
    pub smtp_config: Option<SmtpConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub enum Environment {
    Development,
    Production,
    Testing,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SmtpConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub from_email: String,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        dotenvy::dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://localhost/fastblog".to_string());

        let jwt_secret = env::var("JWT_SECRET")
            .unwrap_or_else(|_| "your-super-secret-jwt-key-change-this-in-production".to_string());

        let port = env::var("PORT")
            .unwrap_or_else(|_| "3001".to_string())
            .parse::<u16>()?;

        let environment = match env::var("ENVIRONMENT")
            .unwrap_or_else(|_| "development".to_string())
            .to_lowercase()
            .as_str()
        {
            "production" => Environment::Production,
            "testing" => Environment::Testing,
            _ => Environment::Development,
        };

        let cors_origins = env::var("CORS_ORIGINS")
            .unwrap_or_else(|_| "http://localhost:3000,http://localhost:3001".to_string())
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        let max_file_size = env::var("MAX_FILE_SIZE")
            .unwrap_or_else(|_| "10485760".to_string()) // 10MB default
            .parse::<usize>()?;

        let redis_url = env::var("REDIS_URL").ok();

        let smtp_config = if let (Ok(host), Ok(port), Ok(username), Ok(password), Ok(from_email)) = (
            env::var("SMTP_HOST"),
            env::var("SMTP_PORT"),
            env::var("SMTP_USERNAME"),
            env::var("SMTP_PASSWORD"),
            env::var("SMTP_FROM_EMAIL"),
        ) {
            Some(SmtpConfig {
                host,
                port: port.parse()?,
                username,
                password,
                from_email,
            })
        } else {
            None
        };

        Ok(Config {
            database_url,
            jwt_secret,
            port,
            environment,
            cors_origins,
            max_file_size,
            redis_url,
            smtp_config,
        })
    }

    pub fn is_development(&self) -> bool {
        matches!(self.environment, Environment::Development)
    }

    pub fn is_production(&self) -> bool {
        matches!(self.environment, Environment::Production)
    }
}
