use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use tower_http::services::ServeDir;
use serde_json::{json, Value};
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod database;
mod handlers;
mod middleware;
mod models;
mod services;

use config::Config;
use database::Database;

pub type AppState = Arc<AppStateInner>;

pub struct AppStateInner {
    pub db: Database,
    pub config: Config,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "fastblog_backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env()?;
    
    // Initialize database
    let db = Database::new(&config.database_url).await?;
    
    // Run migrations
    db.migrate().await?;

    // Create application state
    let state = Arc::new(AppStateInner { db, config });

    // Build the application router
    let app = create_app(state.clone());

    // Start the server
    let listener = tokio::net::TcpListener::bind(&format!("0.0.0.0:{}", state.config.port)).await?;
    
    tracing::info!("🚀 FastBlog server starting on port {}", state.config.port);
    tracing::info!("📖 API Documentation: http://localhost:{}/docs", state.config.port);
    
    axum::serve(listener, app).await?;

    Ok(())
}

fn create_app(state: AppState) -> Router {
    Router::new()
        // Health check
        .route("/health", get(health_check))
        
        // Static file serving for uploads
        .nest_service("/uploads", ServeDir::new("uploads"))
        
        // API routes
        .nest("/api/v1", api_routes())
        
        // Add middleware
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CompressionLayer::new())
                .layer(
                    CorsLayer::new()
                        .allow_origin(Any)
                        .allow_methods(Any)
                        .allow_headers(Any),
                ),
        )
        .with_state(state)
}

fn api_routes() -> Router<AppState> {
    Router::new()
        // Authentication routes
        .nest("/auth", handlers::auth::routes())
        
        // User routes
        .nest("/users", handlers::users::routes())
        
        // Article routes
        .nest("/articles", handlers::articles::routes())
        
        // Engagement routes (claps, comments, bookmarks)
        .nest("/engagement", handlers::engagement::routes())
        
        // Search routes
        .nest("/search", handlers::search::routes())
        
        // Admin routes
        .nest("/admin", handlers::admin::routes())
        
        // Upload routes
        .nest("/upload", upload_routes())
        

}

fn upload_routes() -> Router<AppState> {
    Router::new()
        .route("/avatar", post(handlers::upload::upload_avatar))
        .route("/avatar", axum::routing::delete(handlers::upload::delete_avatar))
}

async fn health_check(State(_state): State<AppState>) -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({
        "status": "ok",
        "service": "fastblog-backend",
        "version": env!("CARGO_PKG_VERSION"),
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}