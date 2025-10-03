//! Authentication routes

use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::post,
    Router,
};
use serde_json::{json, Value};

use crate::auth::{authenticate_user, create_jwt, LoginRequest, LoginResponse};
use crate::AppState;

pub fn auth_routes() -> Router<AppState> {
    Router::new().route("/login", post(login))
}

async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    let user = authenticate_user(&state.db, &payload.username, &payload.password)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let token = create_jwt(&user, &state.config.jwt_secret, state.config.session_duration_hours)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(LoginResponse {
        token,
        user_id: user.id,
        username: user.username,
        role: user.role,
    }))
}
