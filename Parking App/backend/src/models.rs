use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct RegisterRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
    pub name: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct ParkingLocation {
    pub id: Uuid,
    pub name: String,
    pub address: String,
    pub latitude: f64,
    pub longitude: f64,
    pub place_id: Option<String>,
    pub parking_type: ParkingType,
    pub is_covered: Option<bool>,
    pub has_ev_charging: Option<bool>,
    pub is_free: Option<bool>,
    pub is_wheelchair_accessible: Option<bool>,
    pub height_restriction_m: Option<f32>,
    pub rating: Option<f32>,
    pub review_count: Option<i32>,
    pub phone: Option<String>,
    pub website: Option<String>,
    pub opening_hours: Option<serde_json::Value>,
    pub photos: Option<Vec<String>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum ParkingType {
    Lot,
    Garage,
    Street,
    Private,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct SearchRequest {
    pub latitude: f64,
    pub longitude: f64,
    #[validate(range(min = 250, max = 5000))]
    pub radius_m: Option<u32>,
    pub query: Option<String>,
    pub filters: Option<SearchFilters>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate, Default)]
pub struct SearchFilters {
    pub is_free: Option<bool>,
    pub is_covered: Option<bool>,
    pub has_ev_charging: Option<bool>,
    pub is_wheelchair_accessible: Option<bool>,
    pub parking_type: Option<ParkingType>,
    pub open_now: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SearchResponse {
    pub results: Vec<ParkingResult>,
    pub total: i32,
    pub search_metadata: SearchMetadata,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SearchMetadata {
    pub query: String,
    pub latitude: f64,
    pub longitude: f64,
    pub radius_m: u32,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ParkingResult {
    pub location: ParkingLocation,
    pub distance_m: f64,
    pub driving_time_seconds: Option<i32>,
    pub walking_time_seconds: Option<i32>,
    pub availability: AvailabilityEstimate,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AvailabilityEstimate {
    pub score: i32, // 0-100
    pub confidence: i32, // 0-100
    pub level: AvailabilityLevel,
    pub estimated_spaces: Option<i32>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum AvailabilityLevel {
    High,
    Moderate,
    Low,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct CreateReportRequest {
    pub parking_id: Uuid,
    #[validate(range(min = -90, max = 90))]
    pub reporter_latitude: f64,
    #[validate(range(min = -180, max = 180))]
    pub reporter_longitude: f64,
    pub status: ReportStatus,
    pub comment: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum ReportStatus {
    FoundParking,
    AlmostFull,
    CompletelyFull,
    Closed,
    IncorrectInfo,
    TemporaryClosure,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ParkingReport {
    pub id: Uuid,
    pub parking_id: Uuid,
    pub user_id: Option<Uuid>,
    pub reporter_latitude: f64,
    pub reporter_longitude: f64,
    pub status: ReportStatus,
    pub comment: Option<String>,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct FavoriteRequest {
    pub parking_id: Uuid,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Favorite {
    pub id: Uuid,
    pub user_id: Uuid,
    pub parking_id: Uuid,
    pub name: Option<String>,
    pub created_at: DateTime<Utc>,
    pub parking: ParkingLocation,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SearchHistoryEntry {
    pub id: Uuid,
    pub user_id: Uuid,
    pub query: String,
    pub latitude: f64,
    pub longitude: f64,
    pub radius_m: u32,
    pub results_count: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Recommendation {
    pub parking: ParkingLocation,
    pub reason: String,
    pub score: f64,
    pub availability: AvailabilityEstimate,
    pub distance_m: f64,
    pub driving_time_seconds: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct UpdateProfileRequest {
    pub name: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct NavigationRequest {
    pub origin_latitude: f64,
    pub origin_longitude: f64,
    pub destination_latitude: f64,
    pub destination_longitude: f64,
    pub transport_mode: TransportMode,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum TransportMode {
    Driving,
    Walking,
    Transit,
    Bicycling,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct NavigationResponse {
    pub route: Route,
    pub alternatives: Vec<Route>,
    pub eta: DateTime<Utc>,
    pub distance_m: f64,
    pub duration_seconds: i32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Route {
    pub overview_polyline: String,
    pub summary: String,
    pub distance_m: f64,
    pub duration_seconds: i32,
    pub steps: Vec<RouteStep>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RouteStep {
    pub instruction: String,
    pub distance_m: f64,
    pub duration_seconds: i32,
    pub polyline: String,
}
