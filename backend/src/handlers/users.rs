use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, put, post, delete},
    Router,
};
use serde::{Deserialize, Serialize};
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
        .route("/:user_id", get(get_user_by_id))
        .route("/:user_id/follow", post(follow_user).delete(unfollow_user))
        .route("/:user_id/followers", get(get_followers))
        .route("/:user_id/following", get(get_following))
        .route("/:user_id/articles", get(get_user_articles))
        .route("/:user_id/bookmarks", get(get_user_bookmarks))
        .route("/:user_id/reading-lists", get(get_reading_lists))
        .route("/search", get(search_users))
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
    State(_state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    // TODO: Follow user (requires auth middleware)
    Ok(Json(json!({
        "message": format!("Follow user {} not implemented yet", user_id)
    })))
}

async fn unfollow_user(
    State(_state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    // TODO: Unfollow user (requires auth middleware)
    Ok(Json(json!({
        "message": format!("Unfollow user {} not implemented yet", user_id)
    })))
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
    State(_state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    // TODO: Get articles by user
    Ok(Json(json!({
        "message": format!("Get articles for user {} not implemented yet", user_id)
    })))
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
