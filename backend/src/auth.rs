//! Authentication and authorization

use anyhow::Result;
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::db::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user_id
    pub username: String,
    pub role: String,
    pub exp: i64,
    pub iat: i64,
}

impl Claims {
    pub fn new(user: &User, duration_hours: i64) -> Self {
        let now = Utc::now();
        let exp = now + Duration::hours(duration_hours);

        Self {
            sub: user.id.to_string(),
            username: user.username.clone(),
            role: user.role.clone(),
            exp: exp.timestamp(),
            iat: now.timestamp(),
        }
    }
}

pub fn create_jwt(user: &User, secret: &str, duration_hours: i64) -> Result<String> {
    let claims = Claims::new(user, duration_hours);
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;
    Ok(token)
}

pub fn verify_jwt(token: &str, secret: &str) -> Result<Claims> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}

pub async fn hash_password(password: &str) -> Result<String> {
    // In production, use bcrypt or argon2
    // For now, simple placeholder
    Ok(format!("hashed_{}", password))
}

pub async fn verify_password(password: &str, hash: &str) -> Result<bool> {
    // In production, use bcrypt or argon2
    Ok(hash == format!("hashed_{}", password))
}

#[derive(Clone)]
pub struct AuthContext {
    pub user_id: Uuid,
    pub username: String,
    pub role: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthContext
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        // Get JWT secret from extensions (set in middleware)
        let secret = parts
            .extensions
            .get::<String>()
            .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

        // Verify the JWT
        let claims = verify_jwt(bearer.token(), secret).map_err(|_| StatusCode::UNAUTHORIZED)?;

        let user_id = Uuid::parse_str(&claims.sub).map_err(|_| StatusCode::UNAUTHORIZED)?;

        Ok(AuthContext {
            user_id,
            username: claims.username,
            role: claims.role,
        })
    }
}

// Login request/response types
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user_id: Uuid,
    pub username: String,
    pub role: String,
}

pub async fn authenticate_user(
    pool: &PgPool,
    username: &str,
    password: &str,
) -> Result<Option<User>> {
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE username = $1 AND is_active = true",
    )
    .bind(username)
    .fetch_optional(pool)
    .await?;

    if let Some(ref u) = user {
        if verify_password(password, &u.password_hash).await? {
            return Ok(user);
        }
    }

    Ok(None)
}
