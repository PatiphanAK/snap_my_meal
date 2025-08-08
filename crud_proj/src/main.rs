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
    let database = initialize_database().await?;
    let db_pool = Arc::new(db_pool);

    let app = routers::create_app_router(db_pool);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}