use axum::{extract::{Query, State}, Json};
use crate::models::{pagination::TemplateResponse, products::{ProductForm, ProductResponse}};
use sqlx::PgPool;
use std::sync::Arc;

use crate::{
    errors::AppError, models::{pagination::Pagination}, repositories::product_repositories::ProductRepository, services::product_service::{self, ProductServiceTrait}
};

pub async fn get_product_list(State(pool): State<Arc<PgPool>>,Query(pagination): Query<Pagination>) 
-> Result<Json<TemplateResponse<ProductResponse>>, AppError> {
    let repo = Arc::new(ProductRepository::new(pool.clone()));
    let service = product_service::ProductService::new(repo);
    let pagination_clone = pagination.clone();
    let products: Vec<ProductResponse> = service.list_products(pagination).await?;
    let total = products.len();
    let response = TemplateResponse::<ProductResponse> {
        items: products.clone(),
        total,
        limit: pagination_clone.limit.unwrap_or(0),
        offset: pagination_clone.offset.unwrap_or(0),
    };
    Ok(Json(response))
}

pub async  fn add_product(State(pool): State<Arc<PgPool>>, Json(payload): Json<ProductForm>)
-> Result<Json<ProductResponse>, AppError> {
    let repo = Arc::new(ProductRepository::new(pool.clone()));
    let service = product_service::ProductService::new(repo);
    let new_product = service.add_product(payload).await?;
    Ok(Json(new_product))
}

// pub async  fn get_single_product(State(pool): State<Arc<PgPool>>)
// -> Result<Json<ProductResponse>, AppError>{

// }