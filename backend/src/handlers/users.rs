use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use serde_json::{json, Value};
use uuid::Uuid;

use crate::{
    middleware::auth::AuthUser,
    services::user::UserService,
    AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/profile", get(get_profile).put(update_profile))
        .route("/profile/:username", get(get_user_profile_by_username))
        .route("/me/bookmarks", get(get_my_bookmarks))
        .route("/:user_id", get(get_user_by_id))
        .route("/:user_id/follow", post(follow_user).delete(unfollow_user))
        .route("/:user_id/follow-status", get(get_follow_status))
        .route("/:user_id/followers", get(get_followers))
        .route("/:user_id/following", get(get_following))
        .route("/:user_id/articles", get(get_user_articles))
        .route("/:user_id/bookmarks", get(get_user_bookmarks))
        .route("/:user_id/reading-lists", get(get_reading_lists))
        .route("/:user_id/stats", get(get_user_stats))
        .route("/search", get(search_users))
        .route("/recommendations", get(get_user_recommendations))
}

async fn get_profile(
    State(state): State<AppState>,
    user: AuthUser,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let user_service = UserService::new(state.db.pool.clone());
    
    match user_service.get_user_by_id(&user.user_id).await {
        Ok(Some(user_data)) => {
            Ok(Json(serde_json::to_value(user_data).unwrap()))
        }
        Ok(None) => {
            Err((
                StatusCode::NOT_FOUND,
                Json(json!({"error": "User not found"})),
            ))
        }
        Err(e) => {
            tracing::error!("Failed to get user profile: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to get user profile"})),
            ))
        }
    }
}

#[derive(Debug, Deserialize)]
struct UpdateProfileRequest {
    display_name: Option<String>,
    bio: Option<String>,
}

async fn update_profile(
    State(state): State<AppState>,
    user: AuthUser,
    Json(request): Json<UpdateProfileRequest>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let user_service = UserService::new(state.db.pool.clone());
    
    match user_service.update_profile(
        &user.user_id,
        request.display_name.as_deref(),
        request.bio.as_deref(),
    ).await {
        Ok(_) => {
            Ok(Json(json!({
                "message": "Profile updated successfully"
            })))
        }
        Err(e) => {
            tracing::error!("Failed to update profile: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to update profile"})),
            ))
        }
    }
}

async fn get_user_profile_by_username(
    State(state): State<AppState>,
    Path(username): Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let user_service = UserService::new(state.db.pool.clone());
    
    match user_service.get_user_profile(&username).await {
        Ok(Some(user_profile)) => {
            Ok(Json(serde_json::to_value(user_profile).unwrap()))
        }
        Ok(None) => {
            Err((
                StatusCode::NOT_FOUND,
                Json(json!({"error": "User not found"})),
            ))
        }
        Err(e) => {
            tracing::error!("Failed to get user profile: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to get user profile"})),
            ))
        }
    }
}

async fn get_user_by_id(
    State(_state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    // TODO: Get user by ID
    Ok(Json(json!({
        "message": format!("Get user {} not implemented yet", user_id)
    })))
}

async fn follow_user(
    State(state): State<AppState>,
    user: AuthUser,
    Path(user_id): Path<Uuid>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    // Can't follow yourself
    if user.user_id == user_id {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Cannot follow yourself"})),
        ));
    }

    let user_service = UserService::new(state.db.pool.clone());
    
    match user_service.follow_user(&user.user_id, &user_id).await {
        Ok(_) => {
            Ok(Json(json!({
                "message": "Successfully followed user",
                "is_following": true
            })))
        }
        Err(e) => {
            tracing::error!("Failed to follow user: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to follow user"})),
            ))
        }
    }
}

async fn unfollow_user(
    State(state): State<AppState>,
    user: AuthUser,
    Path(user_id): Path<Uuid>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let user_service = UserService::new(state.db.pool.clone());
    
    match user_service.unfollow_user(&user.user_id, &user_id).await {
        Ok(_) => {
            Ok(Json(json!({
                "message": "Successfully unfollowed user",
                "is_following": false
            })))
        }
        Err(e) => {
            tracing::error!("Failed to unfollow user: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to unfollow user"})),
            ))
        }
    }
}

async fn get_follow_status(
    State(state): State<AppState>,
    user: AuthUser,
    Path(user_id): Path<Uuid>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let user_service = UserService::new(state.db.pool.clone());
    
    match user_service.is_following(&user.user_id, &user_id).await {
        Ok(is_following) => {
            Ok(Json(json!({
                "is_following": is_following
            })))
        }
        Err(e) => {
            tracing::error!("Failed to check follow status: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to check follow status"})),
            ))
        }
    }
}

async fn get_followers(
    State(_state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    // TODO: Get user followers
    Ok(Json(json!({
        "message": format!("Get followers for user {} not implemented yet", user_id)
    })))
}

async fn get_following(
    State(_state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    // TODO: Get users that this user is following
    Ok(Json(json!({
        "message": format!("Get following for user {} not implemented yet", user_id)
    })))
}

async fn get_user_articles(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let article_service = crate::services::article::ArticleService::new(state.db.pool.clone());
    
    // Create query params to get articles by this user
    let params = crate::models::ArticleQueryParams {
        author: Some(user_id.to_string()),
        status: Some(crate::models::article::ArticleStatus::Published),
        page: Some(1),
        limit: Some(50),
        ..Default::default()
    };
    
    match article_service.get_articles(params, None).await {
        Ok(response) => {
            Ok(Json(serde_json::to_value(response).unwrap()))
        }
        Err(e) => {
            tracing::error!("Failed to get user articles: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to get user articles"})),
            ))
        }
    }
}

async fn get_my_bookmarks(
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

    // Get bookmarked articles for this user
    let bookmarks = sqlx::query!(
        r#"
        SELECT 
            a.id, a.title, a.subtitle, a.excerpt, a.slug, a.featured_image_url,
            a.reading_time_minutes, a.claps_count, a.comments_count, a.views_count,
            a.published_at, a.created_at,
            u.id as author_id, u.username as author_username, 
            u.display_name as author_display_name, u.avatar_url as author_avatar_url,
            b.created_at as bookmarked_at
        FROM bookmarks b
        INNER JOIN articles a ON b.article_id = a.id
        INNER JOIN users u ON a.author_id = u.id
        WHERE b.user_id = $1 AND a.status = 'published'
        ORDER BY b.created_at DESC
        "#,
        user_id
    )
    .fetch_all(&state.db.pool)
    .await;

    match bookmarks {
        Ok(rows) => {
            let articles: Vec<Value> = rows
                .into_iter()
                .map(|row| {
                    json!({
                        "id": row.id,
                        "title": row.title,
                        "subtitle": row.subtitle,
                        "excerpt": row.excerpt,
                        "slug": row.slug,
                        "featured_image_url": row.featured_image_url,
                        "reading_time_minutes": row.reading_time_minutes,
                        "claps_count": row.claps_count,
                        "comments_count": row.comments_count,
                        "views_count": row.views_count,
                        "published_at": row.published_at,
                        "created_at": row.created_at,
                        "bookmarked_at": row.bookmarked_at,
                        "author": {
                            "id": row.author_id,
                            "username": row.author_username,
                            "display_name": row.author_display_name,
                            "avatar_url": row.author_avatar_url
                        }
                    })
                })
                .collect();

            Ok(Json(json!({
                "articles": articles,
                "total": articles.len()
            })))
        }
        Err(e) => {
            tracing::error!("Failed to get bookmarks: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to get bookmarks"})),
            ))
        }
    }
}

async fn get_user_bookmarks(
    State(_state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    // TODO: Get user bookmarked articles (requires auth middleware for privacy)
    Ok(Json(json!({
        "message": format!("Get bookmarks for user {} not implemented yet", user_id)
    })))
}

async fn get_reading_lists(
    State(_state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    // TODO: Get user reading lists
    Ok(Json(json!({
        "message": format!("Get reading lists for user {} not implemented yet", user_id)
    })))
}

async fn search_users(
    State(_state): State<AppState>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    // TODO: Search users by username/name
    Ok(Json(json!({
        "message": "Search users not implemented yet"
    })))
}

async fn get_user_stats(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let article_service = crate::services::article::ArticleService::new(state.db.pool.clone());
    
    match article_service.get_author_stats(user_id).await {
        Ok(stats) => {
            Ok(Json(serde_json::to_value(stats).unwrap()))
        }
        Err(e) => {
            tracing::error!("Failed to get user stats: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to get user statistics"})),
            ))
        }
    }
}

async fn get_user_recommendations(
    State(state): State<AppState>,
    user: AuthUser,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let limit = params.get("limit")
        .and_then(|l| l.parse::<i64>().ok())
        .unwrap_or(10);

    let user_service = UserService::new(state.db.pool.clone());
    
    match user_service.get_user_recommendations(&user.user_id, Some(limit)).await {
        Ok(recommendations) => {
            Ok(Json(json!({
                "recommendations": recommendations
            })))
        }
        Err(e) => {
            tracing::error!("Failed to get user recommendations: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to get recommendations"})),
            ))
        }
    }
}
