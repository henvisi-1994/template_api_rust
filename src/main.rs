use crate::app_state::AppState;
use axum::Router;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use tokio::net::TcpListener;
use tokio::time::Duration;
mod app_state;
mod controllers;
mod models;
mod repositories;
mod requests;
mod resources;
mod routes;
mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .init();

    // =========================
    // Database (Supabase)
    // =========================
    tracing::info!("acquiring connection...");

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connect_options = database_url
        .parse::<PgConnectOptions>()?
        .statement_cache_capacity(0);

    let pool = PgPoolOptions::new()
        .max_connections(15)
        .min_connections(5)
        .idle_timeout(Duration::from_secs(300))
        .acquire_timeout(Duration::from_secs(10))
        .connect_with(connect_options)
        .await?;
tracing::info!("connection acquired");

    // =========================
    // AppState
    // =========================
    let app_state = AppState { db: pool };

    // =========================
    // Router
    // =========================
    let app = Router::new().merge(routes::api::routes(app_state));

    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("Server listening on http://0.0.0.0:3000");
    axum::serve(listener, app).await?;
    Ok(())
}
