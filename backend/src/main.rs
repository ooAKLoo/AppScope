mod db;
mod error;
mod handlers;
mod models;

use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use db::Database;

pub struct AppState {
    pub db: Database,
    pub write_key: String,
    pub read_key: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:appscope.db".into());
    let write_key = std::env::var("WRITE_KEY").unwrap_or_else(|_| "wk_default_key".into());
    let read_key = std::env::var("READ_KEY").unwrap_or_else(|_| "rk_default_key".into());
    let port = std::env::var("PORT").unwrap_or_else(|_| "3001".into());

    let db = Database::new(&database_url).await?;
    db.init().await?;

    let state = Arc::new(AppState {
        db,
        write_key,
        read_key,
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        // Write APIs (client SDK)
        .route("/api/track", post(handlers::track_event))
        .route("/api/feedback", post(handlers::submit_feedback))
        // Read APIs (Dashboard)
        .route("/api/apps", get(handlers::get_apps))
        .route("/api/stats/dau", get(handlers::get_dau))
        .route("/api/stats/retention", get(handlers::get_retention))
        .route("/api/stats/installs", get(handlers::get_installs))
        .route("/api/feedbacks", get(handlers::get_feedbacks))
        // Health check
        .route("/health", get(|| async { "OK" }))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    tracing::info!("Server running on port {}", port);
    axum::serve(listener, app).await?;

    Ok(())
}
