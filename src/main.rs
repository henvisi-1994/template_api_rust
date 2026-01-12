use axum::Router;
use tokio::net::TcpListener;
use sqlx::postgres::PgPoolOptions;
use crate::app_state::AppState;

mod app_state;
mod routes;
mod controllers;
mod requests;
mod services;
mod models;
mod repositories;

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
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // =========================
    // AppState
    // =========================
    let app_state = AppState { db };

    // =========================
    // Router
    // =========================
    let app = Router::new()
        .merge(routes::api::routes(app_state));

    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    tracing::info!("Server listening on http://127.0.0.1:3000");

    axum::serve(listener, app).await?;

    Ok(())
}
