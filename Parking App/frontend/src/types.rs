use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParkingType {
    Lot,
    Garage,
    Street,
    Private,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchRequest {
    pub latitude: f64,
    pub longitude: f64,
    pub radius_m: Option<u32>,
    pub query: Option<String>,
    pub filters: Option<SearchFilters>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchFilters {
    pub is_free: Option<bool>,
    pub is_covered: Option<bool>,
    pub has_ev_charging: Option<bool>,
    pub is_wheelchair_accessible: Option<bool>,
    pub parking_type: Option<ParkingType>,
    pub open_now: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponse {
    pub results: Vec<ParkingResult>,
    pub total: i32,
    pub search_metadata: SearchMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParkingResult {
    pub location: ParkingLocation,
    pub distance_m: f64,
    pub driving_time_seconds: Option<i32>,
    pub walking_time_seconds: Option<i32>,
    pub availability: AvailabilityEstimate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailabilityEstimate {
    pub score: i32,
    pub confidence: i32,
    pub level: AvailabilityLevel,
    pub estimated_spaces: Option<i32>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AvailabilityLevel {
    High,
    Moderate,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationRequest {
    pub origin_latitude: f64,
    pub origin_longitude: f64,
    pub destination_latitude: f64,
    pub destination_longitude: f64,
    pub transport_mode: TransportMode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransportMode {
    Driving,
    Walking,
    Transit,
    Bicycling,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationResponse {
    pub route: Route,
    pub alternatives: Vec<Route>,
    pub eta: DateTime<Utc>,
    pub distance_m: f64,
    pub duration_seconds: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    pub overview_polyline: String,
    pub summary: String,
    pub distance_m: f64,
    pub duration_seconds: i32,
    pub steps: Vec<RouteStep>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteStep {
    pub instruction: String,
    pub distance_m: f64,
    pub duration_seconds: i32,
    pub polyline: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Favorite {
    pub id: Uuid,
    pub user_id: Uuid,
    pub parking_id: Uuid,
    pub name: Option<String>,
    pub created_at: DateTime<Utc>,
    pub parking: ParkingLocation,
}
