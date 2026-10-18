use axum::{
    extract::{FromRequestParts, State},
    http::request::Parts,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::{
    app_state::AppState,
    middleware::auth::AuthUser,
    requests::{request_auth_login::AuthLoginRequest, request_auth_register::AuthRegisterRequest},
    services::auth::create_token,
};

fn json_response(status: axum::http::StatusCode, body: Value) -> Response {
    (status, Json(body)).into_response()
}

/// Extractor: reads the authenticated user inserted by `auth_middleware`.
///
/// Routes using it must be wrapped with the middleware, otherwise it always
/// answers 401.
#[axum::async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        match parts.extensions.get::<AuthUser>() {
            Some(user) => Ok(user.clone()),
            None => Err(json_response(
                axum::http::StatusCode::UNAUTHORIZED,
                json!({ "status": "error", "message": "unauthorized" }),
            )),
        }
    }
}

/// POST /auth/register
pub async fn register(
    State(state): State<AppState>,
    Json(body): Json<AuthRegisterRequest>,
) -> Response {
    let email = body.email.trim().to_lowercase();

    if !email.contains('@') || body.password.len() < 8 {
        return json_response(
            axum::http::StatusCode::UNPROCESSABLE_ENTITY,
            json!({
                "status": "error",
                "message": "email inválido o contraseña menor a 8 caracteres"
            }),
        );
    }

    let password_hash = match bcrypt::hash(&body.password, bcrypt::DEFAULT_COST) {
        Ok(hash) => hash,
        Err(e) => {
            return json_response(
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                json!({ "status": "error", "message": e.to_string() }),
            )
        }
    };

    let user_id = Uuid::new_v4();

    let result = sqlx::query(
        "INSERT INTO users (id, email, password_hash, name) VALUES ($1, $2, $3, $4)",
    )
    .bind(user_id)
    .bind(&email)
    .bind(&password_hash)
    .bind(body.name.as_deref())
    .execute(&state.db)
    .await;

    match result {
        Ok(_) => match create_token(user_id, &email) {
            Ok(token) => json_response(
                axum::http::StatusCode::CREATED,
                json!({
                    "status": "success",
                    "data": { "token": token, "user": { "id": user_id, "email": email } }
                }),
            ),
            Err(e) => json_response(
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                json!({ "status": "error", "message": e.to_string() }),
            ),
        },
        Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => json_response(
            axum::http::StatusCode::CONFLICT,
            json!({ "status": "error", "message": "el email ya está registrado" }),
        ),
        Err(e) => json_response(
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            json!({ "status": "error", "message": e.to_string() }),
        ),
    }
}

/// POST /auth/login
pub async fn login(State(state): State<AppState>, Json(body): Json<AuthLoginRequest>) -> Response {
    let email = body.email.trim().to_lowercase();

    let row = sqlx::query_as::<_, (Uuid, String)>(
        "SELECT id, password_hash FROM users WHERE email = $1 AND status = 'active'",
    )
    .bind(&email)
    .fetch_optional(&state.db)
    .await;

    let (user_id, password_hash) = match row {
        Ok(Some(row)) => row,
        // Same message for "no user" and "wrong password" to avoid leaking
        // which emails exist.
        Ok(None) => {
            return json_response(
                axum::http::StatusCode::UNAUTHORIZED,
                json!({ "status": "error", "message": "credenciales inválidas" }),
            )
        }
        Err(e) => {
            return json_response(
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                json!({ "status": "error", "message": e.to_string() }),
            )
        }
    };

    match bcrypt::verify(&body.password, &password_hash) {
        Ok(true) => match create_token(user_id, &email) {
            Ok(token) => json_response(
                axum::http::StatusCode::OK,
                json!({
                    "status": "success",
                    "data": { "token": token, "user": { "id": user_id, "email": email } }
                }),
            ),
            Err(e) => json_response(
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                json!({ "status": "error", "message": e.to_string() }),
            ),
        },
        Ok(false) => json_response(
            axum::http::StatusCode::UNAUTHORIZED,
            json!({ "status": "error", "message": "credenciales inválidas" }),
        ),
        Err(e) => json_response(
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            json!({ "status": "error", "message": e.to_string() }),
        ),
    }
}

/// GET /auth/me (protected)
pub async fn me(user: AuthUser) -> Response {
    json_response(
        axum::http::StatusCode::OK,
        json!({
            "status": "success",
            "data": { "user": { "id": user.user_id, "email": user.email } }
        }),
    )
}
