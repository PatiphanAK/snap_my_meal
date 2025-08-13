use std::sync::Arc;
use axum::{routing::{get}, Router};
use sqlx::{Pool, Postgres};

use crate::handlers::product_handler;

pub fn create_router() -> Router<Arc<Pool<Postgres>>> {
    Router::new()
        .route(
            "/",
            get(product_handler::get_product_list)
                .post(product_handler::add_product),
        )
        .route(
            "/{id}",
            get(product_handler::get_product_from_id)
                .patch(product_handler::update_product_with_id)
                .delete(product_handler::delete_product_with_id),
        )
}