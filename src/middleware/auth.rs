use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use uuid::Uuid;

use crate::app_state::AppState;
use crate::services::auth::decode_token;

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: Uuid,
    pub email: Option<String>,
}

pub async fn auth_middleware(
    State(_state): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Response {
    // Read and validate the bearer token.
    let auth_header = req
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    let token = auth_header.strip_prefix("Bearer ").unwrap_or("");

    if token.is_empty() {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "status": "error",
                "message": "missing bearer token"
            })),
        )
            .into_response();
    }

    match decode_token(token) {
        Ok(claims) => {
            req.extensions_mut().insert(AuthUser {
                user_id: claims.user_id,
                email: claims.email,
            });
            return next.run(req).await;
        }
        Err(_) => {}
    }

    (
        StatusCode::UNAUTHORIZED,
        Json(json!({
            "status": "error",
            "message": "invalid or expired token"
        })),
    )
        .into_response()
}
