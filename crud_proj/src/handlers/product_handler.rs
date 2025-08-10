use axum::{extract::{Query, State}, Json};
use sqlx::PgPool;
use std::sync::Arc;

use crate::{
    errors::AppError, models::{pagination::Pagination, products::Product}, repositories::product_repositories::ProductRepository, services::product_service::{self, ProductServiceTrait}
};

pub async fn get_product_list(State(pool): State<Arc<PgPool>>,Query(pagination): Query<Pagination>) 
-> Result<Json<Vec<Product>>, AppError> {
    let repo = Arc::new(ProductRepository::new(pool.clone()));
    let service = product_service::ProductService::new(repo);
    let products: Vec<Product> = service.list_products(pagination).await?;
    Ok(Json(products))
}
