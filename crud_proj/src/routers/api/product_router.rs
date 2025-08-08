use axum::{routing::get, Router};
use crate::handlers::product_handler;
use std::sync::Arc;
use sqlx::PgPool;

pub fn create_router(db_pool: Arc<PgPool>) -> Router {
    Router::new()
        .route("/products", get(product_handler::get_product_list)).with_state(db_pool)
}