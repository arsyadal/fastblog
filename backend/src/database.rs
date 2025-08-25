use sqlx::{postgres::PgPoolOptions, PgPool, Row};
use std::time::Duration;

#[derive(Clone)]
pub struct Database {
    pub pool: PgPool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(50)
            .min_connections(5)
            .acquire_timeout(Duration::from_secs(10))
            .idle_timeout(Duration::from_secs(300))
            .max_lifetime(Duration::from_secs(3600))
            .connect(database_url)
            .await?;

        Ok(Database { pool })
    }

    pub async fn migrate(&self) -> Result<(), sqlx::Error> {
        sqlx::migrate!("./migrations").run(&self.pool).await
            .map_err(|e| sqlx::Error::Migrate(Box::new(e)))
    }

    pub async fn health_check(&self) -> Result<bool, sqlx::Error> {
        let row = sqlx::query("SELECT 1 as health_check")
            .fetch_one(&self.pool)
            .await?;
        
        let health_check: i32 = row.get("health_check");
        Ok(health_check == 1)
    }
}
