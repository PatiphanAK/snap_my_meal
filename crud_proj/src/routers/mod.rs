use axum::{response::Json, routing::get, Router};
use serde_json::{Value, json};
use chrono::Utc;
use sqlx::{PgPool};
use std::sync::Arc;

// Modules
pub mod api;

pub fn create_app_router(db_pool: Arc<PgPool>) -> Router {
    Router::new()
        // Main routes
        .route("/", get(root_handler))
        .route("/hello", get(hello_handler))
        .route("/health", get(health_handler))
        
        // api route
        .nest("/api/v1", api_v1_routes(db_pool))
}

// สร้าง API v1 routes
fn api_v1_routes(db_pool: Arc<PgPool>) -> Router {
    Router::new()
        .merge(api::product_router::create_router(db_pool))
}


async fn root_handler() -> Json<Value> {
    Json(json!({
        "message": "Welcome to API",
        "version": "1.0.0",
        "endpoints": [
            "/hello",
            "/health",
            "/api/v1/products"
        ]
    }))
}


async fn hello_handler() -> Json<Value> {
    Json(json!({"data": "My Name is Patiphan Akkahadsri"}))
}


async fn health_handler() -> Json<Value> {
    let now = Utc::now();
    let timestamp_str = now.to_rfc3339();
    Json(json!({
        "status": "ok",
        "message": "Server is healthy",
        "timestamp": timestamp_str,
    }))
}