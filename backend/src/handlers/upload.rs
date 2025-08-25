use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    response::Json,
};
use serde_json::{json, Value};
use std::path::Path;
use tokio::fs;
use uuid::Uuid;

use crate::{
    services::user::UserService,
    AppState,
};

const MAX_FILE_SIZE: usize = 5 * 1024 * 1024; // 5MB
const ALLOWED_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png", "gif", "webp"];

pub async fn upload_avatar(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    mut multipart: Multipart,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    // Manual auth check
    tracing::debug!("Upload avatar - All headers: {:?}", headers);
    
    let auth_header = headers
        .get("authorization")
        .or_else(|| headers.get("Authorization"))
        .and_then(|header| header.to_str().ok());
    
    tracing::debug!("Upload avatar - Auth header received: {:?}", auth_header);
    
    let token = match auth_header {
        Some(header) if header.starts_with("Bearer ") => {
            header.strip_prefix("Bearer ").unwrap()
        }
        Some(header) => {
            tracing::warn!("Invalid authorization header format: {}", header);
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Invalid authorization format"})),
            ));
        }
        None => {
            tracing::warn!("No authorization header found");
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
    // Create uploads directory if it doesn't exist
    let uploads_dir = "uploads/avatars";
    if let Err(_) = fs::create_dir_all(uploads_dir).await {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to create uploads directory"})),
        ));
    }

    while let Some(field) = multipart.next_field().await.map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Invalid multipart data"})),
        )
    })? {
        let name = field.name().unwrap_or("");
        
        if name == "avatar" {
            let filename = field.file_name().unwrap_or("").to_string();
            
            // Validate file extension
            let extension = Path::new(&filename)
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("")
                .to_lowercase();
            
            if !ALLOWED_EXTENSIONS.contains(&extension.as_str()) {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(json!({"error": "Invalid file type. Allowed: jpg, jpeg, png, gif, webp"})),
                ));
            }

            // Get file data
            let data = field.bytes().await.map_err(|_| {
                (
                    StatusCode::BAD_REQUEST,
                    Json(json!({"error": "Failed to read file data"})),
                )
            })?;

            // Check file size
            if data.len() > MAX_FILE_SIZE {
                return Err((
                    StatusCode::PAYLOAD_TOO_LARGE,
                    Json(json!({"error": "File too large. Maximum size is 5MB"})),
                ));
            }

            // Generate unique filename
            let file_id = Uuid::new_v4();
            let new_filename = format!("{}.{}", file_id, extension);
            let file_path = format!("{}/{}", uploads_dir, new_filename);

            // Save file
            if let Err(_) = fs::write(&file_path, &data).await {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "Failed to save file"})),
                ));
            }

            // Update user avatar URL in database
            let avatar_url = format!("/uploads/avatars/{}", new_filename);
            let user_service = UserService::new(state.db.pool.clone());
            
            match user_service.update_avatar(&user_id, &avatar_url).await {
                Ok(_) => {
                    return Ok(Json(json!({
                        "message": "Avatar uploaded successfully",
                        "avatar_url": avatar_url
                    })));
                }
                Err(e) => {
                    // Clean up uploaded file if database update fails
                    let _ = fs::remove_file(&file_path).await;
                    
                    tracing::error!("Failed to update user avatar: {}", e);
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error": "Failed to update user avatar"})),
                    ));
                }
            }
        }
    }

    Err((
        StatusCode::BAD_REQUEST,
        Json(json!({"error": "No avatar file found"})),
    ))
}

pub async fn delete_avatar(
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
    let user_service = UserService::new(state.db.pool.clone());
    
    // Get current user data to find avatar file
    match user_service.get_user_by_id(&user_id).await {
        Ok(Some(current_user)) => {
            // Remove avatar URL from database
            match user_service.update_avatar(&user_id, "").await {
                Ok(_) => {
                    // Try to delete the physical file if it exists
                    if let Some(avatar_url) = current_user.avatar_url {
                        if avatar_url.starts_with("/uploads/avatars/") {
                            let file_path = format!(".{}", avatar_url);
                            let _ = fs::remove_file(&file_path).await;
                        }
                    }
                    
                    Ok(Json(json!({
                        "message": "Avatar deleted successfully"
                    })))
                }
                Err(e) => {
                    tracing::error!("Failed to delete user avatar: {}", e);
                    Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error": "Failed to delete avatar"})),
                    ))
                }
            }
        }
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "User not found"})),
        )),
        Err(e) => {
            tracing::error!("Failed to get user: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to get user data"})),
            ))
        }
    }
}
