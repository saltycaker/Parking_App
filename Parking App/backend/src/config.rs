use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub redis_url: String,
    pub jwt_secret: String,
    pub jwt_expiration_hours: i64,
    pub apify_api_key: String,
    pub apify_google_places_crawler_id: String,
    pub server_host: String,
    pub server_port: u16,
    pub cache_ttl_seconds: u64,
    pub search_cache_ttl_seconds: u64,
    pub report_expiration_hours: i64,
    pub default_search_radius_m: u32,
    pub max_search_radius_m: u32,
    pub rate_limit_per_second: u64,
    pub rate_limit_burst: u32,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Config {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgresql://postgres:postgres@localhost:5432/parking_db".to_string()),
            redis_url: env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379".to_string()),
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| "your_super_secret_jwt_key".to_string()),
            jwt_expiration_hours: env::var("JWT_EXPIRATION_HOURS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(24),
            apify_api_key: env::var("APIFY_API_KEY")
                .expect("APIFY_API_KEY must be set"),
            apify_google_places_crawler_id: env::var("APIFY_GOOGLE_PLACES_CRAWLER_ID")
                .unwrap_or_else(|_| "compass/crawler-google-places".to_string()),
            server_host: env::var("SERVER_HOST")
                .unwrap_or_else(|_| "0.0.0.0".to_string()),
            server_port: env::var("SERVER_PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(8080),
            cache_ttl_seconds: env::var("CACHE_TTL_SECONDS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(3600),
            search_cache_ttl_seconds: env::var("SEARCH_CACHE_TTL_SECONDS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(1800),
            report_expiration_hours: env::var("REPORT_EXPIRATION_HOURS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(2),
            default_search_radius_m: env::var("DEFAULT_SEARCH_RADIUS_M")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(500),
            max_search_radius_m: env::var("MAX_SEARCH_RADIUS_M")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(5000),
            rate_limit_per_second: env::var("RATE_LIMIT_PER_SECOND")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(10),
            rate_limit_burst: env::var("RATE_LIMIT_BURST")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(20),
        })
    }
}
