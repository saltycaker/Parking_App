mod apify;

use crate::models::*;
use crate::error::{AppError, AppResult};
use crate::db::Database;
use crate::cache::Cache;
use crate::config::Config;
use chrono::{Utc, Timelike, Weekday};
use uuid::Uuid;

pub use apify::ApifyService;

pub struct ParkingService {
    config: Config,
    apify_service: ApifyService,
}

impl ParkingService {
    pub fn new(config: Config) -> Self {
        let apify_service = ApifyService::new(config.clone());
        ParkingService { config, apify_service }
    }

    pub async fn search_parking(
        &self,
        db: &Database,
        cache: &Cache,
        request: SearchRequest,
        user_id: Option<Uuid>,
    ) -> AppResult<SearchResponse> {
        let radius = request.radius_m.unwrap_or(self.config.default_search_radius_m);
        
        // Check cache first
        let cache_key = format!("search:{}:{}:{}:{:?}",
            request.latitude, request.longitude, radius, request.filters);
        
        if let Some(cached) = cache.get(&cache_key).await? {
            if let Ok(cached_response) = serde_json::from_str::<SearchResponse>(&cached) {
                return Ok(cached_response);
            }
        }

        // Query Apify Google Places Crawler
        let locations = self.query_apify(&request).await?;

        // Apply filters
        let filtered = self.apply_filters(locations, &request.filters);

        // Calculate availability for each location
        let mut results = Vec::new();
        for location in filtered {
            let distance = self.calculate_distance(
                request.latitude, request.longitude,
                location.latitude, location.longitude
            );

            let availability = self.calculate_availability(db, &location).await?;

            let driving_time = self.estimate_driving_time(
                request.latitude, request.longitude,
                location.latitude, location.longitude
            ).await.ok();

            let walking_time = self.estimate_walking_time(distance);

            results.push(ParkingResult {
                location,
                distance_m: distance,
                driving_time_seconds: driving_time,
                walking_time_seconds: Some(walking_time),
                availability,
            });
        }

        // Sort by distance
        results.sort_by(|a, b| a.distance_m.partial_cmp(&b.distance_m).unwrap());

        let total = results.len() as i32;
        let response = SearchResponse {
            results,
            total,
            search_metadata: SearchMetadata {
                query: request.query.unwrap_or_else(|| "nearby parking".to_string()),
                latitude: request.latitude,
                longitude: request.longitude,
                radius_m: radius,
                timestamp: Utc::now(),
            },
        };

        // Cache the results
        let cache_value = serde_json::to_string(&response)?;
        cache.set(&cache_key, &cache_value, self.config.search_cache_ttl_seconds).await?;

        // Save to search history if user is authenticated
        if let Some(uid) = user_id {
            self.save_search_history(db, uid, &request, total).await?;
        }

        Ok(response)
    }

    async fn query_apify(&self, request: &SearchRequest) -> AppResult<Vec<ParkingLocation>> {
        let radius = request.radius_m.unwrap_or(self.config.default_search_radius_m);
        let query = request.query.as_deref();
        
        self.apify_service
            .search_parking_places(
                request.latitude,
                request.longitude,
                radius,
                query,
            )
            .await
    }

    fn apply_filters(&self, locations: Vec<ParkingLocation>, filters: &Option<SearchFilters>) -> Vec<ParkingLocation> {
        match filters {
            None => locations,
            Some(f) => {
                locations.into_iter().filter(|loc| {
                    if let Some(is_free) = f.is_free {
                        if loc.is_free != Some(is_free) {
                            return false;
                        }
                    }
                    if let Some(is_covered) = f.is_covered {
                        if loc.is_covered != Some(is_covered) {
                            return false;
                        }
                    }
                    if let Some(has_ev) = f.has_ev_charging {
                        if loc.has_ev_charging != Some(has_ev) {
                            return false;
                        }
                    }
                    if let Some(is_accessible) = f.is_wheelchair_accessible {
                        if loc.is_wheelchair_accessible != Some(is_accessible) {
                            return false;
                        }
                    }
                    if let Some(parking_type) = f.parking_type {
                        if loc.parking_type != parking_type {
                            return false;
                        }
                    }
                    if let Some(open_now) = f.open_now {
                        let is_open = self.is_location_open(loc);
                        if is_open != open_now {
                            return false;
                        }
                    }
                    true
                }).collect()
            }
        }
    }

    fn is_location_open(&self, location: &ParkingLocation) -> bool {
        // Check opening hours if available
        if let Some(_hours) = &location.opening_hours {
            // Parse and check if currently open
            // This is a simplified check - full implementation would parse Google's opening hours format
            return true; // Placeholder
        }
        true // Assume open if no hours specified
    }

    fn calculate_distance(&self, lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
        // Haversine formula
        const EARTH_RADIUS: f64 = 6371000.0; // meters
        
        let d_lat = (lat2 - lat1).to_radians();
        let d_lon = (lon2 - lon1).to_radians();
        
        let a = (d_lat / 2.0).sin().powi(2)
            + lat1.to_radians().cos() * lat2.to_radians().cos() * (d_lon / 2.0).sin().powi(2);
        
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
        
        EARTH_RADIUS * c
    }

    async fn calculate_availability(&self, db: &Database, location: &ParkingLocation) -> AppResult<AvailabilityEstimate> {
        let now = Utc::now();
        
        // Get recent reports
        let recent_reports = sqlx::query_as::<_, ParkingReport>(
            "SELECT * FROM parking_reports 
             WHERE parking_id = $1 
             AND created_at > $2 
             ORDER BY created_at DESC"
        )
        .bind(location.id)
        .bind(now - chrono::Duration::hours(3))
        .fetch_all(db.pool())
        .await?;

        // Calculate base score from historical data
        let mut score = 50i32;
        let mut confidence = 50i32;

        // Adjust based on time of day
        let hour = now.hour();
        if hour >= 9 && hour <= 17 {
            // Business hours - lower availability
            score -= 20;
        } else if hour >= 18 && hour <= 22 {
            // Evening - moderate availability
            score -= 10;
        } else {
            // Night/early morning - higher availability
            score += 20;
        }

        // Adjust based on day of week
        match now.weekday() {
            Weekday::Sat | Weekday::Sun => score += 15, // Weekend - higher availability
            _ => score -= 5, // Weekday - lower availability
        }

        // Adjust based on recent reports
        if !recent_reports.is_empty() {
            confidence = 70;
            let full_count = recent_reports.iter()
                .filter(|r| matches!(r.status, ReportStatus::CompletelyFull | ReportStatus::AlmostFull))
                .count();
            
            let available_count = recent_reports.iter()
                .filter(|r| matches!(r.status, ReportStatus::FoundParking))
                .count();

            if full_count > available_count {
                score -= 30;
            } else if available_count > full_count {
                score += 20;
            }
        }

        // Adjust based on rating
        if let Some(rating) = location.rating {
            if rating > 4.5 {
                score -= 15; // Popular spots fill up faster
            } else if rating < 3.5 {
                score += 10; // Less popular spots have more availability
            }
        }

        // Clamp values
        score = score.clamp(0, 100);
        confidence = confidence.clamp(0, 100);

        let level = match score {
            s if s >= 70 => AvailabilityLevel::High,
            s if s >= 40 => AvailabilityLevel::Moderate,
            _ => AvailabilityLevel::Low,
        };

        Ok(AvailabilityEstimate {
            score,
            confidence,
            level,
            estimated_spaces: None, // Could be estimated based on lot size
            last_updated: now,
        })
    }

    async fn estimate_driving_time(&self, origin_lat: f64, origin_lon: f64, dest_lat: f64, dest_lon: f64) -> AppResult<i32> {
        // Estimate based on distance using Haversine formula
        let distance = self.calculate_distance(origin_lat, origin_lon, dest_lat, dest_lon);
        // Assume average city driving speed of 30 km/h
        Ok(((distance / 1000.0) / 30.0 * 3600.0) as i32)
    }

    fn estimate_walking_time(&self, distance_m: f64) -> i32 {
        // Assume average walking speed of 5 km/h
        ((distance_m / 1000.0) / 5.0 * 3600.0) as i32
    }

    async fn save_search_history(&self, db: &Database, user_id: Uuid, request: &SearchRequest, results_count: i32) -> AppResult<()> {
        sqlx::query(
            "INSERT INTO search_history (user_id, query, latitude, longitude, radius_m, results_count, created_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7)"
        )
        .bind(user_id)
        .bind(request.query.as_deref().unwrap_or("nearby parking"))
        .bind(request.latitude)
        .bind(request.longitude)
        .bind(request.radius_m.unwrap_or(self.config.default_search_radius_m))
        .bind(results_count)
        .bind(Utc::now())
        .execute(db.pool())
        .await?;

        Ok(())
    }

    pub async fn get_recommendations(
        &self,
        db: &Database,
        user_id: Uuid,
        latitude: f64,
        longitude: f64,
    ) -> AppResult<Vec<Recommendation>> {
        // Get user's search history to understand preferences
        let history = sqlx::query_as::<_, SearchHistoryEntry>(
            "SELECT * FROM search_history WHERE user_id = $1 ORDER BY created_at DESC LIMIT 10"
        )
        .bind(user_id)
        .fetch_all(db.pool())
        .await?;

        // Get recent searches
        let search_request = SearchRequest {
            latitude,
            longitude,
            radius_m: Some(1000),
            query: None,
            filters: None,
        };

        let search_results = self.search_parking(db, &Cache::new(&self.config.redis_url).await?, search_request, Some(user_id)).await?;

        // Score and rank recommendations
        let mut recommendations: Vec<Recommendation> = search_results.results.into_iter().map(|result| {
            let mut score = 0.0;

            // Distance score (closer is better)
            score += (1.0 - (result.distance_m / 1000.0).min(1.0)) * 30.0;

            // Availability score
            score += (result.availability.score as f64 / 100.0) * 25.0;

            // Rating score
            if let Some(rating) = result.location.rating {
                score += (rating / 5.0) * 20.0;
            }

            // Walking distance score
            if let Some(walking_time) = result.walking_time_seconds {
                score += (1.0 - (walking_time as f64 / 600.0).min(1.0)) * 15.0;
            }

            // Generate reason
            let reason = self.generate_recommendation_reason(&result);

            Recommendation {
                parking: result.location,
                reason,
                score,
                availability: result.availability,
                distance_m: result.distance_m,
                driving_time_seconds: result.driving_time_seconds,
            }
        }).collect();

        // Sort by score
        recommendations.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

        // Return top 5
        recommendations.truncate(5);

        Ok(recommendations)
    }

    fn generate_recommendation_reason(&self, result: &ParkingResult) -> String {
        let mut reasons = Vec::new();

        if result.distance_m < 200.0 {
            reasons.push(format!("{} meters away", result.distance_m as i32));
        }

        if result.availability.score >= 70 {
            reasons.push("high estimated availability".to_string());
        }

        if let Some(rating) = result.location.rating {
            if rating >= 4.5 {
                reasons.push("highly rated".to_string());
            }
        }

        if let Some(walking_time) = result.walking_time_seconds {
            if walking_time < 300 {
                reasons.push("short walk".to_string());
            }
        }

        if reasons.is_empty() {
            "Good parking option".to_string()
        } else {
            format!("Recommended because it is {}", reasons.join(", "))
        }
    }
}
