use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, put},
    Router,
};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::{
    models::{Article, ArticleQueryParams, CreateArticleRequest, UpdateArticleRequest, engagement::{ClapRequest, CreateCommentRequest}},
    services::{article::ArticleService, engagement::EngagementService},
    middleware::auth::OptionalAuthUser,
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
        .route("/:article_id/read", post(record_read))
        .route("/:article_id/comments", post(create_comment))
        .route("/:article_id/highlights", post(create_highlight))
        .route("/feed", get(get_personalized_feed))
        .route("/drafts", get(get_drafts))
        .route("/drafts/:article_id", get(get_draft))
        
        // Specific routes before parameterized ones
        .route("/draft/auto-save", post(auto_save_draft))
        .route("/:article_id/stats", get(get_article_stats))
        .route("/:article_id/featured", post(toggle_featured))
        .route("/categories", get(get_categories))
        .route("/tags", get(get_tags))
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
            let error_msg = format!("Failed to get articles: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": error_msg})),
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
            let error_msg = format!("Failed to create article: {}", e);
            tracing::error!("{}", error_msg);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": error_msg})),
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
    headers: axum::http::HeaderMap,
    Path(article_id): Path<Uuid>,
    Json(payload): Json<UpdateArticleRequest>,
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
    
    match article_service.update_article(article_id, user_id, payload).await {
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
    headers: axum::http::HeaderMap,
    Path(article_id): Path<Uuid>,
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
    
    match article_service.delete_article(article_id, user_id).await {
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
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Path(article_id): Path<Uuid>,
    Json(payload): Json<ClapRequest>,
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

    let engagement_service = EngagementService::new(state.db.pool.clone());
    
    match engagement_service.clap_article(article_id, user_id, payload).await {
        Ok((total_claps, user_claps, is_clapped)) => {
            Ok(Json(json!({
                "total_claps": total_claps,
                "user_claps": user_claps,
                "is_clapped": is_clapped
            })))
        }
        Err(e) => {
            tracing::error!("Failed to clap article: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to clap article"})),
            ))
        }
    }
}

async fn bookmark_article(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Path(article_id): Path<Uuid>,
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

    // Check if article exists
    let article_exists = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM articles WHERE id = $1 AND status = 'published')",
        article_id
    )
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or(Some(false))
    .unwrap_or(false);

    if !article_exists {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Article not found"})),
        ));
    }

    // Check if already bookmarked
    let already_bookmarked = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM bookmarks WHERE user_id = $1 AND article_id = $2)",
        user_id,
        article_id
    )
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or(Some(false))
    .unwrap_or(false);

    if already_bookmarked {
        return Ok(Json(json!({
            "bookmarked": true,
            "message": "Article already bookmarked"
        })));
    }

    // Add bookmark
    match sqlx::query!(
        "INSERT INTO bookmarks (user_id, article_id, created_at) VALUES ($1, $2, NOW())",
        user_id,
        article_id
    )
    .execute(&state.db.pool)
    .await
    {
        Ok(_) => {
            // Update bookmarks_count on article
            let _ = sqlx::query!(
                "UPDATE articles SET bookmarks_count = bookmarks_count + 1 WHERE id = $1",
                article_id
            )
            .execute(&state.db.pool)
            .await;

            Ok(Json(json!({
                "bookmarked": true,
                "message": "Article bookmarked successfully"
            })))
        }
        Err(e) => {
            tracing::error!("Failed to bookmark article: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to bookmark article"})),
            ))
        }
    }
}

async fn unbookmark_article(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Path(article_id): Path<Uuid>,
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

    // Remove bookmark
    let result = sqlx::query!(
        "DELETE FROM bookmarks WHERE user_id = $1 AND article_id = $2",
        user_id,
        article_id
    )
    .execute(&state.db.pool)
    .await;

    match result {
        Ok(res) => {
            if res.rows_affected() > 0 {
                // Update bookmarks_count on article
                let _ = sqlx::query!(
                    "UPDATE articles SET bookmarks_count = GREATEST(bookmarks_count - 1, 0) WHERE id = $1",
                    article_id
                )
                .execute(&state.db.pool)
                .await;

                Ok(Json(json!({
                    "bookmarked": false,
                    "message": "Bookmark removed successfully"
                })))
            } else {
                Ok(Json(json!({
                    "bookmarked": false,
                    "message": "Bookmark not found"
                })))
            }
        }
        Err(e) => {
            tracing::error!("Failed to remove bookmark: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to remove bookmark"})),
            ))
        }
    }
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

async fn record_read(
    State(state): State<AppState>,
    Path(article_id): Path<Uuid>,
    OptionalAuthUser(user): OptionalAuthUser,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let article_service = ArticleService::new(state.db.pool.clone());
    let user_id = user.map(|u| u.user_id);
    
    match article_service.record_read(article_id, user_id).await {
        Ok(_) => {
            Ok(Json(json!({
                "message": "Read recorded successfully"
            })))
        }
        Err(e) => {
            tracing::error!("Failed to record read: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to record read"})),
            ))
        }
    }
}

async fn get_comments(
    State(state): State<AppState>,
    OptionalAuthUser(user): OptionalAuthUser,
    Path(article_id): Path<Uuid>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let engagement_service = EngagementService::new(state.db.pool.clone());
    let user_id = user.map(|u| u.user_id);
    
    match engagement_service.get_comments(article_id, user_id).await {
        Ok(comments) => {
            Ok(Json(json!({
                "comments": comments
            })))
        }
        Err(e) => {
            tracing::error!("Failed to get comments: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to get comments"})),
            ))
        }
    }
}

async fn create_comment(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Path(article_id): Path<Uuid>,
    Json(payload): Json<CreateCommentRequest>,
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

    let engagement_service = EngagementService::new(state.db.pool.clone());
    
    match engagement_service.create_comment(article_id, user_id, payload).await {
        Ok(comment) => {
            Ok(Json(serde_json::to_value(comment).unwrap()))
        }
        Err(e) => {
            tracing::error!("Failed to create comment: {}", e);
            let status = if e.to_string().contains("not found") {
                StatusCode::NOT_FOUND
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            };
            Err((
                status,
                Json(json!({"error": e.to_string()})),
            ))
        }
    }
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
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Query(params): Query<std::collections::HashMap<String, String>>,
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

    let page = params.get("page")
        .and_then(|p| p.parse::<i64>().ok())
        .unwrap_or(1);
    let limit = params.get("limit")
        .and_then(|l| l.parse::<i64>().ok())
        .unwrap_or(20);

    let article_service = ArticleService::new(state.db.pool.clone());
    
    match article_service.get_user_feed(user_id, Some(page), Some(limit)).await {
        Ok(response) => Ok(Json(serde_json::to_value(response).unwrap())),
        Err(e) => {
            tracing::error!("Failed to get user feed: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to get feed"})),
            ))
        }
    }
}

async fn get_trending_articles(
    State(state): State<AppState>,
    OptionalAuthUser(user): OptionalAuthUser,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let page = params.get("page")
        .and_then(|p| p.parse::<i64>().ok())
        .unwrap_or(1);
    let limit = params.get("limit")
        .and_then(|l| l.parse::<i64>().ok())
        .unwrap_or(20);
    let time_window = params.get("time_window")
        .and_then(|t| t.parse::<i32>().ok())
        .unwrap_or(168); // Default: 7 days

    let article_service = ArticleService::new(state.db.pool.clone());
    let user_id = user.map(|u| u.user_id);
    
    match article_service.get_trending_articles(user_id, Some(page), Some(limit), Some(time_window)).await {
        Ok(response) => Ok(Json(serde_json::to_value(response).unwrap())),
        Err(e) => {
            tracing::error!("Failed to get trending articles: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to get trending articles"})),
            ))
        }
    }
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
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
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

    // Get drafts for this user
    let drafts = sqlx::query_as::<_, Article>(
        r#"
        SELECT 
            id, title, subtitle, content, content_html, excerpt, featured_image_url, 
            author_id, publication_id, status, 
            is_member_only, is_featured, paywall_position, slug, 
            COALESCE(tags, ARRAY[]::TEXT[])::TEXT[] as tags, 
            COALESCE(categories, ARRAY[]::TEXT[])::TEXT[] as categories, 
            reading_time_minutes, 
            claps_count, comments_count, bookmarks_count, views_count, reads_count,
            published_at, created_at, updated_at, last_auto_save, auto_save_version
        FROM articles 
        WHERE author_id = $1 AND status = 'draft'
        ORDER BY updated_at DESC, created_at DESC
        LIMIT 50
        "#,
    )
    .bind(user_id)
    .fetch_all(&state.db.pool)
    .await;

    match drafts {
        Ok(articles) => {
            let draft_list: Vec<Value> = articles
                .into_iter()
                .map(|article| {
                    json!({
                        "id": article.id,
                        "title": article.title,
                        "subtitle": article.subtitle,
                        "excerpt": article.excerpt,
                        "updated_at": article.updated_at,
                        "created_at": article.created_at,
                        "last_auto_save": article.last_auto_save,
                    })
                })
                .collect();
            
            Ok(Json(json!({
                "drafts": draft_list,
                "total": draft_list.len()
            })))
        }
        Err(e) => {
            tracing::error!("Failed to get drafts: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Failed to get drafts: {}", e)})),
            ))
        }
    }
}

async fn get_draft(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Path(article_id): Path<Uuid>,
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
    
    // Get draft article (must be draft and owned by user)
    let article = match sqlx::query_as::<_, Article>(
        r#"
        SELECT 
            id, title, subtitle, content, content_html, excerpt, featured_image_url, 
            author_id, publication_id, status, 
            is_member_only, is_featured, paywall_position, slug, 
            COALESCE(tags, ARRAY[]::TEXT[])::TEXT[] as tags, 
            COALESCE(categories, ARRAY[]::TEXT[])::TEXT[] as categories, 
            reading_time_minutes, 
            claps_count, comments_count, bookmarks_count, views_count, reads_count,
            published_at, created_at, updated_at, last_auto_save, auto_save_version
        FROM articles 
        WHERE id = $1 AND author_id = $2 AND status = 'draft'
        "#,
    )
    .bind(article_id)
    .bind(user_id)
    .fetch_optional(&state.db.pool)
    .await
    {
        Ok(article) => article,
        Err(e) => {
            tracing::error!("Database error fetching draft: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to fetch draft"})),
            ));
        }
    };

    match article {
        Some(article) => {
            match article_service.get_article_response(&article, Some(user_id)).await {
                Ok(response) => Ok(Json(serde_json::to_value(response).unwrap())),
                Err(e) => {
                    tracing::error!("Failed to get draft response: {}", e);
                    Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error": format!("Failed to get draft: {}", e)})),
                    ))
                }
            }
        }
        None => Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Draft not found or unauthorized"})),
        )),
    }
}

async fn publish_article(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Path(article_id): Path<Uuid>,
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
    
    match article_service.publish_article(article_id, user_id).await {
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

async fn auto_save_draft(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<crate::models::AutoSaveDraftRequest>,
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
    
    match article_service.auto_save_draft(user_id, &payload).await {
        Ok(article_id) => {
            Ok(Json(json!({
                "message": "Draft auto-saved successfully",
                "article_id": article_id,
                "auto_saved_at": chrono::Utc::now()
            })))
        }
        Err(e) => {
            tracing::error!("Failed to auto-save draft: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to auto-save draft"})),
            ))
        }
    }
}

async fn get_article_stats(
    State(state): State<AppState>,
    Path(article_id): Path<Uuid>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let article_service = ArticleService::new(state.db.pool.clone());
    
    match article_service.get_article_stats(article_id).await {
        Ok(stats) => {
            Ok(Json(serde_json::to_value(stats).unwrap()))
        }
        Err(e) => {
            tracing::error!("Failed to get article stats: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to get article stats"})),
            ))
        }
    }
}

async fn toggle_featured(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Path(article_id): Path<Uuid>,
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
    
    match article_service.toggle_featured(article_id, Some(user_id)).await {
        Ok(is_featured) => {
            Ok(Json(json!({
                "message": if is_featured { "Article featured successfully" } else { "Article unfeatured successfully" },
                "is_featured": is_featured
            })))
        }
        Err(e) => {
            tracing::error!("Failed to toggle featured: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to toggle featured status"})),
            ))
        }
    }
}

async fn get_categories(
    State(state): State<AppState>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    // Get unique categories from published articles
    let categories = sqlx::query_scalar!(
        "SELECT DISTINCT unnest(categories) as category FROM articles WHERE status = 'published' AND categories IS NOT NULL AND array_length(categories, 1) > 0 ORDER BY category"
    )
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to get categories: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to get categories"})),
        )
    })?;
    
    Ok(Json(json!({
        "categories": categories
    })))
}

async fn get_tags(
    State(state): State<AppState>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    // Get unique tags from published articles with count
    let tags_with_count = sqlx::query!(
        r#"
        SELECT 
            unnest(tags) as tag,
            COUNT(*) as article_count
        FROM articles 
        WHERE status = 'published' AND tags IS NOT NULL AND array_length(tags, 1) > 0 
        GROUP BY tag 
        ORDER BY article_count DESC, tag
        LIMIT 100
        "#
    )
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to get tags: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to get tags"})),
        )
    })?;
    
    let tags: Vec<serde_json::Value> = tags_with_count
        .into_iter()
        .map(|row| {
            json!({
                "tag": row.tag,
                "article_count": row.article_count
            })
        })
        .collect();
    
    Ok(Json(json!({
        "tags": tags
    })))
}
