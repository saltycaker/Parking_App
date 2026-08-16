use axum::{Json, extract::State};
use crate::models::{RegisterRequest, LoginRequest, AuthResponse};
use crate::auth::AuthService;
use crate::error::AppResult;
use crate::AppState;

pub async fn register(
    State(state): State<AppState>,
    Json(request): Json<RegisterRequest>,
) -> AppResult<Json<AuthResponse>> {
    let auth_service = AuthService::new(state.config);
    let response = auth_service.register(&state.db, request).await?;
    Ok(Json(response))
}

pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> AppResult<Json<AuthResponse>> {
    let auth_service = AuthService::new(state.config);
    let response = auth_service.login(&state.db, request).await?;
    Ok(Json(response))
}

pub async fn logout() -> AppResult<Json<serde_json::Value>> {
    // In a real implementation, this would invalidate the token
    Ok(Json(serde_json::json!({"message": "Logged out successfully"})))
}
