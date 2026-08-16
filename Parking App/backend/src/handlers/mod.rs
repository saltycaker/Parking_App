use axum::{
    extract::{Path, State, Query},
    Json,
};
use uuid::Uuid;
use utoipa::ToSchema;

use crate::models::*;
use crate::services::ParkingService;
use crate::auth::AuthService;
use crate::error::{AppError, AppResult};
use crate::AppState;

pub mod auth;

pub use auth::*;

pub async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now(),
    }))
}

pub async fn search_parking(
    State(state): State<AppState>,
    Json(request): Json<SearchRequest>,
) -> AppResult<Json<SearchResponse>> {
    let service = ParkingService::new(state.config);
    let result = service.search_parking(&state.db, &state.cache, request, None).await?;
    Ok(Json(result))
}

pub async fn get_parking(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<ParkingLocation>> {
    let location = sqlx::query_as::<_, ParkingLocation>(
        "SELECT id, name, address, latitude, longitude, place_id, parking_type, \
         is_covered, has_ev_charging, is_free, is_wheelchair_accessible, \
         height_restriction_m, rating, review_count, phone, website, \
         opening_hours, photos, created_at, updated_at \
         FROM parking_locations WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(state.db.pool())
    .await?
    .ok_or_else(|| AppError::NotFound("Parking location not found".to_string()))?;

    Ok(Json(location))
}

pub async fn create_report(
    State(state): State<AppState>,
    Json(request): Json<CreateReportRequest>,
) -> AppResult<Json<ParkingReport>> {
    let now = chrono::Utc::now();
    let expires_at = now + chrono::Duration::hours(state.config.report_expiration_hours);
    let report_id = Uuid::new_v4();

    sqlx::query(
        "INSERT INTO parking_reports (id, parking_id, user_id, reporter_latitude, reporter_longitude, status, comment, created_at, expires_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"
    )
    .bind(report_id)
    .bind(request.parking_id)
    .bind(None::<Uuid>)
    .bind(request.reporter_latitude)
    .bind(request.reporter_longitude)
    .bind(request.status.to_string())
    .bind(&request.comment)
    .bind(now)
    .bind(expires_at)
    .execute(state.db.pool())
    .await?;

    let report = sqlx::query_as::<_, ParkingReport>(
        "SELECT * FROM parking_reports WHERE id = $1"
    )
    .bind(report_id)
    .fetch_one(state.db.pool())
    .await?;

    Ok(Json(report))
}

pub async fn get_report(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<ParkingReport>> {
    let report = sqlx::query_as::<_, ParkingReport>(
        "SELECT * FROM parking_reports WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(state.db.pool())
    .await?
    .ok_or_else(|| AppError::NotFound("Report not found".to_string()))?;

    Ok(Json(report))
}

pub async fn get_favorites(
    State(state): State<AppState>,
) -> AppResult<Json<Vec<Favorite>>> {
    // Would filter by user_id from auth token
    let user_id = Uuid::new_v4(); // Placeholder - would come from auth

    let favorites = sqlx::query_as::<_, Favorite>(
        "SELECT id, user_id, parking_id, name, created_at \
         FROM favorites WHERE user_id = $1"
    )
    .bind(user_id)
    .fetch_all(state.db.pool())
    .await?;

    Ok(Json(favorites))
}

pub async fn add_favorite(
    State(state): State<AppState>,
    Json(request): Json<FavoriteRequest>,
) -> AppResult<Json<Favorite>> {
    let now = chrono::Utc::now();
    let user_id = Uuid::new_v4(); // Placeholder - would come from auth
    let favorite_id = Uuid::new_v4();

    sqlx::query(
        "INSERT INTO favorites (id, user_id, parking_id, name, created_at)
         VALUES ($1, $2, $3, $4, $5)"
    )
    .bind(favorite_id)
    .bind(user_id)
    .bind(request.parking_id)
    .bind(&request.name)
    .bind(now)
    .execute(state.db.pool())
    .await?;

    let favorite = sqlx::query_as::<_, Favorite>(
        "SELECT id, user_id, parking_id, name, created_at \
         FROM favorites WHERE id = $1"
    )
    .bind(favorite_id)
    .fetch_one(state.db.pool())
    .await?;

    Ok(Json(favorite))
}

pub async fn remove_favorite(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    sqlx::query(
        "DELETE FROM favorites WHERE id = $1"
    )
    .bind(id)
    .execute(state.db.pool())
    .await?;

    Ok(Json(serde_json::json!({"message": "Favorite removed"})))
}

pub async fn get_search_history(
    State(state): State<AppState>,
) -> AppResult<Json<Vec<SearchHistoryEntry>>> {
    let user_id = Uuid::new_v4(); // Placeholder - would come from auth

    let history = sqlx::query_as::<_, SearchHistoryEntry>(
        "SELECT * FROM search_history WHERE user_id = $1 ORDER BY created_at DESC LIMIT 20"
    )
    .bind(user_id)
    .fetch_all(state.db.pool())
    .await?;

    Ok(Json(history))
}

pub async fn get_recommendations(
    State(state): State<AppState>,
    Query(params): Query<RecommendationParams>,
) -> AppResult<Json<Vec<Recommendation>>> {
    let service = ParkingService::new(state.config.clone());
    let user_id = Uuid::new_v4(); // Placeholder - would come from auth

    let recommendations = service.get_recommendations(
        &state.db,
        &state.cache,
        user_id,
        params.latitude.unwrap_or(0.0),
        params.longitude.unwrap_or(0.0),
    ).await?;

    Ok(Json(recommendations))
}

#[derive(Debug, serde::Deserialize, ToSchema)]
pub struct RecommendationParams {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

pub async fn get_profile(
    State(state): State<AppState>,
) -> AppResult<Json<UserResponse>> {
    let user_id = Uuid::new_v4(); // Placeholder - would come from auth

    let user = sqlx::query_as::<_, crate::models::User>(
        "SELECT * FROM users WHERE id = $1"
    )
    .bind(user_id)
    .fetch_one(state.db.pool())
    .await?;

    Ok(Json(UserResponse {
        id: user.id,
        email: user.email,
        name: user.name,
        phone: user.phone,
        created_at: user.created_at,
    }))
}

pub async fn update_profile(
    State(state): State<AppState>,
    Json(request): Json<UpdateProfileRequest>,
) -> AppResult<Json<UserResponse>> {
    let user_id = Uuid::new_v4(); // Placeholder - would come from auth
    let now = chrono::Utc::now();

    sqlx::query(
        "UPDATE users SET name = COALESCE($1, name), phone = COALESCE($2, phone), updated_at = $3 WHERE id = $4"
    )
    .bind(&request.name)
    .bind(&request.phone)
    .bind(now)
    .bind(user_id)
    .execute(state.db.pool())
    .await?;

    let user = sqlx::query_as::<_, crate::models::User>(
        "SELECT * FROM users WHERE id = $1"
    )
    .bind(user_id)
    .fetch_one(state.db.pool())
    .await?;

    Ok(Json(UserResponse {
        id: user.id,
        email: user.email,
        name: user.name,
        phone: user.phone,
        created_at: user.created_at,
    }))
}
