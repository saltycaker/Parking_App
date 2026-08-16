use crate::models::{ParkingLocation, ParkingType};
use crate::config::Config;
use crate::error::{AppError, AppResult};
use serde_json::Value;
use chrono::{DateTime, Utc};

pub struct ApifyService {
    config: Config,
    client: reqwest::Client,
}

impl ApifyService {
    pub fn new(config: Config) -> Self {
        ApifyService {
            config,
            client: reqwest::Client::new(),
        }
    }

    pub async fn search_parking_places(
        &self,
        latitude: f64,
        longitude: f64,
        radius_m: u32,
        query: Option<&str>,
    ) -> AppResult<Vec<ParkingLocation>> {
        let search_queries = query.unwrap_or("parking");
        
        // Build the request for Apify Google Places Crawler
        let mut request_body = serde_json::json!({
            "searchQueriesArray": [search_queries],
            "locationQuery": format!("{},{}", latitude, longitude),
            "maxPlaces": 50,
            "maxCrawledPlacesPerSearch": 50,
            "language": "en",
            "countryCode": "US",
        });

        if radius_m > 0 {
            request_body["locationFilter"] = serde_json::json!({
                "radius": radius_m,
                "center": {
                    "latitude": latitude,
                    "longitude": longitude
                }
            });
        }

        let response = self.client
            .post(&format!(
                "https://api.apify.com/v2/acts/{}/runs",
                self.config.apify_google_places_crawler_id
            ))
            .header("Authorization", &format!("Bearer {}", self.config.apify_api_key))
            .json(&request_body)
            .send()
            .await
            .map_err(|e| AppError::ExternalApi(format!("Failed to start Apify crawl: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::ExternalApi(format!("Apify API error: {}", error_text)));
        }

        let run_info: Value = response.json().await
            .map_err(|e| AppError::ExternalApi(format!("Failed to parse Apify response: {}", e)))?;

        let run_id = run_info["data"]["id"]
            .as_str()
            .ok_or_else(|| AppError::ExternalApi("No run ID in Apify response".to_string()))?;

        // Wait for the run to finish
        let dataset_items = self.wait_for_run_completion(run_id).await?;

        // Parse the results into ParkingLocation objects
        let parking_locations = self.parse_parking_locations(dataset_items).await?;

        Ok(parking_locations)
    }

    async fn wait_for_run_completion(&self, run_id: &str) -> AppResult<Vec<Value>> {
        let max_attempts = 30; // 30 seconds timeout
        let mut attempts = 0;

        while attempts < max_attempts {
            let response = self.client
                .get(&format!(
                    "https://api.apify.com/v2/runs/{}/",
                    run_id
                ))
                .header("Authorization", &format!("Bearer {}", self.config.apify_api_key))
                .send()
                .await
                .map_err(|e| AppError::ExternalApi(format!("Failed to check run status: {}", e)))?;

            let run_info: Value = response.json().await
                .map_err(|e| AppError::ExternalApi(format!("Failed to parse run status: {}", e)))?;

            let status = run_info["data"]["status"]
                .as_str()
                .ok_or_else(|| AppError::ExternalApi("No status in run info".to_string()))?;

            match status {
                "SUCCEEDED" => {
                    // Get the dataset items
                    let dataset_id = run_info["data"]["defaultDatasetId"]
                        .as_str()
                        .ok_or_else(|| AppError::ExternalApi("No dataset ID in run info".to_string()))?;

                    return self.get_dataset_items(dataset_id).await;
                }
                "FAILED" => {
                    let error_message = run_info["data"]["statusMessage"]
                        .as_str()
                        .unwrap_or("Unknown error");
                    return Err(AppError::ExternalApi(format!("Apify run failed: {}", error_message)));
                }
                "RUNNING" | "READY" => {
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                    attempts += 1;
                }
                _ => {
                    return Err(AppError::ExternalApi(format!("Unexpected run status: {}", status)));
                }
            }
        }

        Err(AppError::ExternalApi("Apify run timed out".to_string()))
    }

    async fn get_dataset_items(&self, dataset_id: &str) -> AppResult<Vec<Value>> {
        let response = self.client
            .get(&format!(
                "https://api.apify.com/v2/datasets/{}/items",
                dataset_id
            ))
            .header("Authorization", &format!("Bearer {}", self.config.apify_api_key))
            .send()
            .await
            .map_err(|e| AppError::ExternalApi(format!("Failed to get dataset items: {}", e)))?;

        let dataset: Value = response.json().await
            .map_err(|e| AppError::ExternalApi(format!("Failed to parse dataset: {}", e)))?;

        let items = dataset["data"]["items"]
            .as_array()
            .ok_or_else(|| AppError::ExternalApi("No items in dataset".to_string()))?
            .to_vec();

        Ok(items)
    }

    async fn parse_parking_locations(&self, items: Vec<Value>) -> AppResult<Vec<ParkingLocation>> {
        let mut locations = Vec::new();

        for item in items {
            if let Some(location_data) = item.get("location") {
                let name = location_data["name"]
                    .as_str()
                    .unwrap_or("Unknown Parking")
                    .to_string();

                let address = location_data["address"]
                    .as_str()
                    .or_else(|| location_data["formattedAddress"].as_str())
                    .unwrap_or("Unknown Address")
                    .to_string();

                let latitude = location_data["latitude"]
                    .as_f64()
                    .ok_or_else(|| AppError::ExternalApi("Missing latitude".to_string()))?;

                let longitude = location_data["longitude"]
                    .as_f64()
                    .ok_or_else(|| AppError::ExternalApi("Missing longitude".to_string()))?;

                let place_id = location_data["placeId"]
                    .as_str()
                    .map(|s| s.to_string());

                // Determine parking type from categories and title
                let parking_type = self.determine_parking_type(&name, &location_data);

                let is_covered = self.extract_bool_field(&location_data, "isCovered");
                let has_ev_charging = self.extract_bool_field(&location_data, "hasEvCharging");
                let is_free = self.extract_bool_field(&location_data, "isFree");
                let is_wheelchair_accessible = self.extract_bool_field(&location_data, "isWheelchairAccessible");

                let rating = location_data["rating"]
                    .as_f64()
                    .map(|r| r as f32);

                let review_count = location_data["reviewsCount"]
                    .as_i64()
                    .map(|c| c as i32);

                let phone = location_data["phone"]
                    .as_str()
                    .map(|s| s.to_string());

                let website = location_data["website"]
                    .as_str()
                    .map(|s| s.to_string());

                let opening_hours = location_data["openingHours"]
                    .as_object()
                    .cloned()
                    .map(|o| serde_json::to_value(o).unwrap());

                let photos = location_data["photos"]
                    .as_array()
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|p| p.as_str())
                            .map(|s| s.to_string())
                            .collect()
                    });

                let now = Utc::now();

                locations.push(ParkingLocation {
                    id: uuid::Uuid::new_v4(),
                    name,
                    address,
                    latitude,
                    longitude,
                    place_id,
                    parking_type,
                    is_covered,
                    has_ev_charging,
                    is_free,
                    is_wheelchair_accessible,
                    height_restriction_m: None,
                    rating,
                    review_count,
                    phone,
                    website,
                    opening_hours,
                    photos,
                    created_at: now,
                    updated_at: now,
                });
            }
        }

        Ok(locations)
    }

    fn determine_parking_type(&self, name: &str, location_data: &Value) -> ParkingType {
        let name_lower = name.to_lowercase();
        
        if name_lower.contains("garage") || name_lower.contains("multi-level") {
            return ParkingType::Garage;
        }
        
        if name_lower.contains("street") || name_lower.contains("on-street") {
            return ParkingType::Street;
        }
        
        if name_lower.contains("private") || name_lower.contains("reserved") {
            return ParkingType::Private;
        }
        
        ParkingType::Lot
    }

    fn extract_bool_field(&self, location_data: &Value, field_name: &str) -> Option<bool> {
        location_data[field_name]
            .as_bool()
            .or_else(|| {
                location_data["categories"]
                    .as_array()
                    .and_then(|categories| {
                        categories.iter()
                            .any(|cat| cat.as_str().unwrap_or("").to_lowercase().contains(field_name))
                            .then_some(true)
                    })
            })
    }
}
