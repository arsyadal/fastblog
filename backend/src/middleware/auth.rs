use axum::{
    extract::{Request, State, FromRequestParts},
    http::{header::AUTHORIZATION, StatusCode, request::Parts},
    middleware::Next,
    response::Response,
    async_trait,
};
use serde_json::json;
use uuid::Uuid;

use crate::{services::auth::{AuthService, Claims}, models::UserType, AppState};

// Auth extractor for required authentication
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: Uuid,
    pub username: String,
    pub user_type: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, axum::Json<serde_json::Value>);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let claims = parts
            .extensions
            .get::<Claims>()
            .ok_or_else(|| {
                (
                    StatusCode::UNAUTHORIZED,
                    axum::Json(json!({"error": "Authentication required"})),
                )
            })?;

        Ok(AuthUser {
            user_id: claims.sub.parse().map_err(|_| {
                (
                    StatusCode::UNAUTHORIZED,
                    axum::Json(json!({"error": "Invalid user ID in token"})),
                )
            })?,
            username: claims.username.clone(),
            user_type: format!("{:?}", claims.user_type),
        })
    }
}

// Optional auth extractor
#[derive(Debug, Clone)]
pub struct OptionalAuthUser(pub Option<AuthUser>);

#[async_trait]
impl<S> FromRequestParts<S> for OptionalAuthUser
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_user = parts
            .extensions
            .get::<Claims>()
            .and_then(|claims| {
                claims.sub.parse().ok().map(|user_id| AuthUser {
                    user_id,
                    username: claims.username.clone(),
                    user_type: format!("{:?}", claims.user_type),
                })
            });

        Ok(OptionalAuthUser(auth_user))
    }
}

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let token = match auth_header {
        Some(header) if header.starts_with("Bearer ") => {
            header.trim_start_matches("Bearer ")
        }
        _ => {
            tracing::warn!("Missing or invalid Authorization header");
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    let auth_service = AuthService::new(&state.db, &state.config);

    match auth_service.verify_token(token).await {
        Ok(claims) => {
            // Add user info to request extensions
            request.extensions_mut().insert(claims);
            Ok(next.run(request).await)
        }
        Err(e) => {
            tracing::warn!("Token verification failed: {}", e);
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

// Optional auth middleware - doesn't fail if no token provided
pub async fn optional_auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Response {
    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    if let Some(header) = auth_header {
        if let Some(token) = header.strip_prefix("Bearer ") {
            let auth_service = AuthService::new(&state.db, &state.config);
            
            if let Ok(claims) = auth_service.verify_token(token).await {
                request.extensions_mut().insert(claims);
            }
        }
    }

    next.run(request).await
}
