use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, put, delete},
    Router, middleware,
};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::{
    models::{ArticleQueryParams, CreateArticleRequest, UpdateArticleRequest},
    services::article::ArticleService,
    middleware::auth::{AuthUser, OptionalAuthUser, optional_auth_middleware, auth_middleware},
    AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        // Public article routes (no auth required, but optional auth)
        .route("/", get(get_articles))
        .route("/:article_id", get(get_article))
        .route("/slug/:slug", get(get_article_by_slug))
        .route("/trending", get(get_trending_articles))
        .route("/featured", get(get_featured_articles))
        .route("/:article_id/comments", get(get_comments))
        .route("/:article_id/highlights", get(get_highlights))
        
        // Auth required routes
        .route("/", post(create_article))
        .route("/:article_id", put(update_article).delete(delete_article))
        .route("/:article_id/publish", post(publish_article))
        .route("/:article_id/clap", post(clap_article))
        .route("/:article_id/bookmark", post(bookmark_article).delete(unbookmark_article))
        .route("/:article_id/view", post(record_view))
        .route("/:article_id/comments", post(create_comment))
        .route("/:article_id/highlights", post(create_highlight))
        .route("/feed", get(get_personalized_feed))
        .route("/drafts", get(get_drafts))
        .route("/drafts/:article_id", get(get_draft))

}

async fn get_articles(
    State(state): State<AppState>,
    OptionalAuthUser(user): OptionalAuthUser,
    Query(params): Query<ArticleQueryParams>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let article_service = ArticleService::new(state.db.pool.clone());
    let user_id = user.map(|u| u.user_id);
    
    match article_service.get_articles(params, user_id).await {
        Ok(response) => Ok(Json(serde_json::to_value(response).unwrap())),
        Err(e) => {
            tracing::error!("Failed to get articles: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to get articles"})),
            ))
        }
    }
}

async fn create_article(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<CreateArticleRequest>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    // Manual auth check
    let auth_header = headers
        .get("authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "));

    let token = match auth_header {
        Some(token) => token,
        None => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Authentication required"})),
            ));
        }
    };

    let auth_service = crate::services::auth::AuthService::new(&state.db, &state.config);
    let claims = match auth_service.verify_token(token).await {
        Ok(claims) => claims,
        Err(_) => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Invalid token"})),
            ));
        }
    };

    let user_id: uuid::Uuid = match claims.sub.parse() {
        Ok(id) => id,
        Err(_) => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Invalid user ID in token"})),
            ));
        }
    };

    let article_service = ArticleService::new(state.db.pool.clone());
    
    match article_service.create_article(user_id, payload).await {
        Ok(response) => Ok(Json(serde_json::to_value(response).unwrap())),
        Err(e) => {
            tracing::error!("Failed to create article: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to create article"})),
            ))
        }
    }
}

async fn get_article(
    State(state): State<AppState>,
    OptionalAuthUser(user): OptionalAuthUser,
    Path(article_id): Path<Uuid>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let article_service = ArticleService::new(state.db.pool.clone());
    let user_id = user.map(|u| u.user_id);
    
    match article_service.get_article_by_id(article_id, user_id).await {
        Ok(response) => Ok(Json(serde_json::to_value(response).unwrap())),
        Err(e) => {
            tracing::error!("Failed to get article: {}", e);
            Err((
                StatusCode::NOT_FOUND,
                Json(json!({"error": "Article not found"})),
            ))
        }
    }
}

async fn get_article_by_slug(
    State(state): State<AppState>,
    OptionalAuthUser(user): OptionalAuthUser,
    Path(slug): Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let article_service = ArticleService::new(state.db.pool.clone());
    let user_id = user.map(|u| u.user_id);
    
    match article_service.get_article_by_slug(&slug, user_id).await {
        Ok(response) => Ok(Json(serde_json::to_value(response).unwrap())),
        Err(e) => {
            tracing::error!("Failed to get article by slug: {}", e);
            Err((
                StatusCode::NOT_FOUND,
                Json(json!({"error": "Article not found"})),
            ))
        }
    }
}

async fn update_article(
    State(state): State<AppState>,
    user: AuthUser,
    Path(article_id): Path<Uuid>,
    Json(payload): Json<UpdateArticleRequest>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let article_service = ArticleService::new(state.db.pool.clone());
    
    match article_service.update_article(article_id, user.user_id, payload).await {
        Ok(response) => Ok(Json(serde_json::to_value(response).unwrap())),
        Err(e) => {
            tracing::error!("Failed to update article: {}", e);
            let status = if e.to_string().contains("not found") {
                StatusCode::NOT_FOUND
            } else if e.to_string().contains("Unauthorized") {
                StatusCode::FORBIDDEN
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            };
            Err((status, Json(json!({"error": e.to_string()}))))
        }
    }
}

async fn delete_article(
    State(state): State<AppState>,
    user: AuthUser,
    Path(article_id): Path<Uuid>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let article_service = ArticleService::new(state.db.pool.clone());
    
    match article_service.delete_article(article_id, user.user_id).await {
        Ok(()) => Ok(Json(json!({"message": "Article deleted successfully"}))),
        Err(e) => {
            tracing::error!("Failed to delete article: {}", e);
            let status = if e.to_string().contains("not found") || e.to_string().contains("unauthorized") {
                StatusCode::NOT_FOUND
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            };
            Err((status, Json(json!({"error": e.to_string()}))))
        }
    }
}

async fn clap_article(
    State(_state): State<AppState>,
    Path(article_id): Path<Uuid>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    // TODO: Clap article (requires auth middleware)
    Ok(Json(json!({
        "message": format!("Clap article {} not implemented yet", article_id)
    })))
}

async fn bookmark_article(
    State(_state): State<AppState>,
    Path(article_id): Path<Uuid>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    // TODO: Bookmark article (requires auth middleware)
    Ok(Json(json!({
        "message": format!("Bookmark article {} not implemented yet", article_id)
    })))
}

async fn unbookmark_article(
    State(_state): State<AppState>,
    Path(article_id): Path<Uuid>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    // TODO: Remove bookmark (requires auth middleware)
    Ok(Json(json!({
        "message": format!("Unbookmark article {} not implemented yet", article_id)
    })))
}

async fn record_view(
    State(_state): State<AppState>,
    Path(article_id): Path<Uuid>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    // TODO: Record article view for analytics
    Ok(Json(json!({
        "message": format!("Record view for article {} not implemented yet", article_id)
    })))
}

async fn get_comments(
    State(_state): State<AppState>,
    Path(article_id): Path<Uuid>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    // TODO: Get comments for article
    Ok(Json(json!({
        "message": format!("Get comments for article {} not implemented yet", article_id)
    })))
}

async fn create_comment(
    State(_state): State<AppState>,
    Path(article_id): Path<Uuid>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    // TODO: Create comment on article (requires auth middleware)
    Ok(Json(json!({
        "message": format!("Create comment on article {} not implemented yet", article_id)
    })))
}

async fn get_highlights(
    State(_state): State<AppState>,
    Path(article_id): Path<Uuid>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    // TODO: Get highlights for article (user-specific if authenticated)
    Ok(Json(json!({
        "message": format!("Get highlights for article {} not implemented yet", article_id)
    })))
}

async fn create_highlight(
    State(_state): State<AppState>,
    Path(article_id): Path<Uuid>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    // TODO: Create highlight on article (requires auth middleware)
    Ok(Json(json!({
        "message": format!("Create highlight on article {} not implemented yet", article_id)
    })))
}

async fn get_personalized_feed(
    State(_state): State<AppState>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    // TODO: Get personalized article feed (requires auth middleware)
    Ok(Json(json!({
        "message": "Get personalized feed not implemented yet"
    })))
}

async fn get_trending_articles(
    State(_state): State<AppState>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    // TODO: Get trending articles
    Ok(Json(json!({
        "message": "Get trending articles not implemented yet"
    })))
}

async fn get_featured_articles(
    State(_state): State<AppState>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    // TODO: Get featured/staff-picked articles
    Ok(Json(json!({
        "message": "Get featured articles not implemented yet"
    })))
}

async fn get_drafts(
    State(_state): State<AppState>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    // TODO: Get user's draft articles (requires auth middleware)
    Ok(Json(json!({
        "message": "Get drafts not implemented yet"
    })))
}

async fn get_draft(
    State(_state): State<AppState>,
    Path(article_id): Path<Uuid>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    // TODO: Get specific draft (requires auth middleware + ownership check)
    Ok(Json(json!({
        "message": format!("Get draft {} not implemented yet", article_id)
    })))
}

async fn publish_article(
    State(state): State<AppState>,
    user: AuthUser,
    Path(article_id): Path<Uuid>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let article_service = ArticleService::new(state.db.pool.clone());
    
    match article_service.publish_article(article_id, user.user_id).await {
        Ok(response) => Ok(Json(serde_json::to_value(response).unwrap())),
        Err(e) => {
            tracing::error!("Failed to publish article: {}", e);
            let status = if e.to_string().contains("not found") || e.to_string().contains("unauthorized") {
                StatusCode::NOT_FOUND
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            };
            Err((status, Json(json!({"error": e.to_string()}))))
        }
    }
}
