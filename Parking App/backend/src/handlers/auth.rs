use axum::{Json, State};
use crate::models::{RegisterRequest, LoginRequest, AuthResponse};
use crate::db::Database;
use crate::config::Config;
use crate::auth::AuthService;
use crate::error::AppResult;

pub async fn register(
    State(db): State<Database>,
    State(config): State<Config>,
    Json(request): Json<RegisterRequest>,
) -> AppResult<Json<AuthResponse>> {
    let auth_service = AuthService::new(config);
    let response = auth_service.register(&db, request).await?;
    Ok(Json(response))
}

pub async fn login(
    State(db): State<Database>,
    State(config): State<Config>,
    Json(request): Json<LoginRequest>,
) -> AppResult<Json<AuthResponse>> {
    let auth_service = AuthService::new(config);
    let response = auth_service.login(&db, request).await?;
    Ok(Json(response))
}

pub async fn logout() -> AppResult<Json<serde_json::Value>> {
    // In a real implementation, this would invalidate the token
    Ok(Json(serde_json::json!({"message": "Logged out successfully"})))
}
