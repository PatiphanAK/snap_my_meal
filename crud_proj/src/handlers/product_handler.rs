use axum::{extract::{Query, State}, Json};
use crate::models::products::{ProductForm, ProductResponse};
use sqlx::PgPool;
use std::sync::Arc;

use crate::{
    errors::AppError, models::{pagination::Pagination}, repositories::product_repositories::ProductRepository, services::product_service::{self, ProductServiceTrait}
};

pub async fn get_product_list(State(pool): State<Arc<PgPool>>,Query(pagination): Query<Pagination>) 
-> Result<Json<Vec<ProductResponse>>, AppError> {
    let repo = Arc::new(ProductRepository::new(pool.clone()));
    let service = product_service::ProductService::new(repo);
    let products: Vec<ProductResponse> = service.list_products(pagination).await?;
    Ok(Json(products))
}

pub async  fn add_product(State(pool): State<Arc<PgPool>>, Json(payload): Json<ProductForm>)
-> Result<Json<ProductResponse>, AppError> {
    let repo = Arc::new(ProductRepository::new(pool.clone()));
    let service = product_service::ProductService::new(repo);
    let new_product = service.add_product(payload).await?;
    Ok(Json(new_product))
}