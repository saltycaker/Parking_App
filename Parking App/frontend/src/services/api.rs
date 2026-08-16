use crate::types::*;
use gloo_net::http::Request;
use leptos::*;

const API_BASE_URL: &str = "http://localhost:8080";

pub async fn search_parking(request: SearchRequest) -> Result<SearchResponse, String> {
    Request::post(&format!("{}/search", API_BASE_URL))
        .json(&request)
        .map_err(|e| format!("Failed to create request: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))
}

pub async fn get_parking(id: uuid::Uuid) -> Result<ParkingLocation, String> {
    Request::get(&format!("{}/parking/{}", API_BASE_URL, id))
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))
}

pub async fn get_navigation(
    origin_lat: f64,
    origin_lon: f64,
    dest_lat: f64,
    dest_lon: f64,
) -> Result<NavigationResponse, String> {
    let request = NavigationRequest {
        origin_latitude: origin_lat,
        origin_longitude: origin_lon,
        destination_latitude: dest_lat,
        destination_longitude: dest_lon,
        transport_mode: TransportMode::Driving,
    };

    Request::post(&format!("{}/navigation", API_BASE_URL))
        .json(&request)
        .map_err(|e| format!("Failed to create request: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))
}

pub async fn get_favorites() -> Result<Vec<Favorite>, String> {
    Request::get(&format!("{}/favorites", API_BASE_URL))
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))
}

pub async fn add_favorite(parking_id: uuid::Uuid, name: Option<String>) -> Result<Favorite, String> {
    #[derive(Serialize)]
    struct FavoriteRequest {
        parking_id: uuid::Uuid,
        name: Option<String>,
    }

    Request::post(&format!("{}/favorites", API_BASE_URL))
        .json(&FavoriteRequest { parking_id, name })
        .map_err(|e| format!("Failed to create request: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))
}

pub async fn remove_favorite(id: uuid::Uuid) -> Result<(), String> {
    Request::delete(&format!("{}/favorites/{}", API_BASE_URL, id))
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;
    Ok(())
}
