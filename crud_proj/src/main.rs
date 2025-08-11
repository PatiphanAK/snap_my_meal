mod routers;
mod handlers;
mod models;
mod errors;
mod services;
mod repositories;
mod boostdb;

use std::sync::Arc;
use boostdb::Database;
use tracing::{info};
use env_logger::Env;

async fn initialize_database() -> Result<Database, Box<dyn std::error::Error>> {
    boostdb::init::initialize().await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let env_mode = std::env::var("APP_ENV").unwrap_or_else(|_| "development".into());
    let default_level = if env_mode == "production" { "error" } else { "trace" };
    let env = Env::default().filter_or("RUST_LOG", default_level);
    env_logger::init_from_env(env);

    
    let database = initialize_database().await?;
    let db_pool = Arc::new(database.pool().clone());

    let app = routers::create_app_router(db_pool);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    info!("Server running on http://0.0.0.0:3000");
    axum::serve(listener, app).await?;
    
    Ok(())
}