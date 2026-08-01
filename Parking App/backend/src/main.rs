mod config;
mod db;
mod cache;
mod auth;
mod models;
mod handlers;
mod services;
mod error;

use axum::{
    routing::{get, post, patch, delete},
    Router,
    http::Method,
};
use tower_http::{
    cors::{CorsLayer, Any},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use config::Config;
use db::Database;
use cache::Cache;
use error::AppError;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Load environment variables
    dotenv::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "parking_api=info,tower_http=info,axum=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env()?;
    tracing::info!("Configuration loaded successfully");

    // Initialize database
    let db = Database::new(&config.database_url).await?;
    tracing::info!("Database connected successfully");

    // Initialize cache
    let cache = Cache::new(&config.redis_url).await?;
    tracing::info!("Redis connected successfully");

    // Build our application with routes
    let app = Router::new()
        .route("/health", get(handlers::health))
        .route("/search", post(handlers::search_parking))
        .route("/parking/:id", get(handlers::get_parking))
        .route("/reports", post(handlers::create_report))
        .route("/reports/:id", get(handlers::get_report))
        .route("/favorites", get(handlers::get_favorites))
        .route("/favorites", post(handlers::add_favorite))
        .route("/favorites/:id", delete(handlers::remove_favorite))
        .route("/history", get(handlers::get_search_history))
        .route("/recommendations", get(handlers::get_recommendations))
        .route("/auth/register", post(handlers::auth::register))
        .route("/auth/login", post(handlers::auth::login))
        .route("/auth/logout", post(handlers::auth::logout))
        .route("/profile", get(handlers::get_profile))
        .route("/profile", patch(handlers::update_profile))
        .merge(SwaggerUi::new("/swagger-ui")
            .url("/api-docs/openapi.json", handlers::ApiDoc::openapi()))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
                .allow_headers(Any),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(db)
        .with_state(cache)
        .with_state(config.clone());

    let addr = format!("{}:{}", config.server_host, config.server_port);
    tracing::info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
