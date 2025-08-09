mod routers;
mod handlers;
mod models;
mod errors;
mod services;
mod repositories;
mod boostdb;

use std::sync::Arc;
use boostdb::Database;

async fn initialize_database() -> Result<Database, Box<dyn std::error::Error>> {
    boostdb::init::initialize().await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    
    let database = initialize_database().await?;
    let db_pool = Arc::new(database.pool().clone());

    let app = routers::create_app_router(db_pool);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("🚀 Server running on http://0.0.0.0:3000");
    axum::serve(listener, app).await?;
    
    Ok(())
}