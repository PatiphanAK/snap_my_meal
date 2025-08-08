mod routers;
mod handlers;
mod models;
mod errors;
mod services;
mod reposistoris;
mod boostdb;
use std::sync::Arc;


#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let db_config = boostdb::DbConfig::from_env();
    let db_pool = match boostdb::create_pool_with_options(db_config).await {
        Ok(pool) => {
            println!("✅ Database connection pool created successfully");
            
            // Test the connection
            if let Err(e) = boostdb::test_connection(&pool).await {
                eprintln!("❌ Database connection test failed: {}", e);
                std::process::exit(1);
            }
            
            pool
        }
        Err(e) => {
            eprintln!("❌ Failed to create database connection pool: {}", e);
            std::process::exit(1);
        }
    };
    let db_pool = Arc::new(db_pool);

    let app = routers::create_app_router(db_pool);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}