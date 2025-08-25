use axum::{
    extract::{State, Request},
    http::{StatusCode, HeaderMap},
    response::Json,
    routing::{post, get},
    Router,
};
use serde_json::{json, Value};
use validator::Validate;

use crate::{
    models::{CreateUserRequest, LoginRequest, AuthResponse, UserResponse},
    services::auth::AuthService,
    AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/refresh", post(refresh_token))
        .route("/logout", post(logout))
        .route("/me", get(get_current_user_simple))
        .route("/verify-email", post(verify_email))
        .route("/forgot-password", post(forgot_password))
        .route("/reset-password", post(reset_password))
}

async fn register(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<Value>)> {
    // Validate input
    if let Err(errors) = payload.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "Validation failed",
                "details": errors
            })),
        ));
    }

    let auth_service = AuthService::new(&state.db, &state.config);
    
    match auth_service.register(payload).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => {
            tracing::error!("Registration failed: {}", e);
            Err((
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": "Registration failed",
                    "message": e.to_string()
                })),
            ))
        }
    }
}

async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<Value>)> {
    // Validate input
    if let Err(errors) = payload.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "Validation failed",
                "details": errors
            })),
        ));
    }

    let auth_service = AuthService::new(&state.db, &state.config);
    
    match auth_service.login(payload).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => {
            tracing::error!("Login failed: {}", e);
            Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "error": "Authentication failed",
                    "message": "Invalid email or password"
                })),
            ))
        }
    }
}

async fn refresh_token(
    State(_state): State<AppState>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    // TODO: Implement token refresh logic
    Ok(Json(json!({
        "message": "Token refresh not implemented yet"
    })))
}

async fn logout(
    State(_state): State<AppState>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    // TODO: Implement logout logic (blacklist token)
    Ok(Json(json!({
        "message": "Logged out successfully"
    })))
}

async fn get_current_user_simple(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<UserResponse>, (StatusCode, Json<Value>)> {
    // Extract token from Authorization header
    let auth_header = headers.get("authorization")
        .and_then(|header| header.to_str().ok())
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "error": "Authorization header required"
                }))
            )
        })?;

    let token = auth_header.strip_prefix("Bearer ")
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "error": "Invalid authorization format"
                }))
            )
        })?;

    let auth_service = AuthService::new(&state.db, &state.config);
    
    // Verify token and get claims
    let claims = auth_service.verify_token(token).await
        .map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "error": "Invalid or expired token"
                }))
            )
        })?;
    
    // Get user by ID from claims
    match auth_service.get_user_by_id(&claims.sub).await {
        Ok(user) => Ok(Json(user.into())),
        Err(e) => {
            tracing::error!("Failed to get user: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Failed to get user information"
                }))
            ))
        }
    }
}

async fn verify_email(
    State(_state): State<AppState>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    // TODO: Implement email verification
    Ok(Json(json!({
        "message": "Email verification not implemented yet"
    })))
}

async fn forgot_password(
    State(_state): State<AppState>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    // TODO: Implement forgot password
    Ok(Json(json!({
        "message": "Forgot password not implemented yet"
    })))
}

async fn reset_password(
    State(_state): State<AppState>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    // TODO: Implement reset password
    Ok(Json(json!({
        "message": "Reset password not implemented yet"
    })))
}
