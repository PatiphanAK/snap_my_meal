use axum::{extract::{Path, Query, State}, http::StatusCode, Json};
use uuid::Uuid;
use crate::{models::{pagination::TemplateResponse, products::{ProductForm, ProductResponse}}};
use sqlx::PgPool;
use std::sync::Arc;

use crate::{
    errors::AppError, models::{pagination::Pagination}, repositories::product_repositories::ProductRepository, services::product_service::{self, ProductServiceTrait}
};


fn create_product_service(pool: Arc<PgPool>) -> product_service::ProductService {
    let repo = Arc::new(ProductRepository::new(pool));
    product_service::ProductService::new(repo)
}

pub async fn get_product_list(
    State(pool): State<Arc<PgPool>>,
    Query(pagination): Query<Pagination>
) -> Result<(StatusCode, Json<TemplateResponse<ProductResponse>>), AppError> {
    let service = create_product_service(pool);
    let pagination_clone = pagination.clone();
    let products: Vec<ProductResponse> = service.list_products(pagination).await?;
    let total = products.len();
    let response = TemplateResponse::<ProductResponse> {
        items: products.clone(),
        total,
        limit: pagination_clone.limit.unwrap_or(0),
        offset: pagination_clone.offset.unwrap_or(0),
    };
    Ok((StatusCode::OK, Json(response)))
}

pub async fn add_product(
    State(pool): State<Arc<PgPool>>, 
    Json(payload): Json<ProductForm>
) -> Result<(StatusCode, Json<ProductResponse>), AppError> {
    let service = create_product_service(pool);
    let new_product = service.add_product(payload).await?;
    Ok((StatusCode::CREATED, Json(new_product)))
}

pub async fn get_product_from_id(
    State(pool): State<Arc<PgPool>>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<ProductResponse>), AppError> {
    let repo = Arc::new(ProductRepository::new(pool.clone()));
    let service = product_service::ProductService::new(repo);
    let product = service.get_product_from_id(id).await?.ok_or_else(|| AppError::NotFound)?;
    Ok((StatusCode::OK, Json(product)))
}

pub async fn update_product_with_id(
    State(pool): State<Arc<PgPool>>, 
    Path(id): Path<Uuid>, 
    Json(product): Json<ProductForm>
) -> Result<(StatusCode, Json<ProductResponse>), AppError> {
    let service = create_product_service(pool);
    let product = service.update_product_from_id(id, product).await?.ok_or_else(|| AppError::NotFound)?;
    Ok((StatusCode::OK, Json(product)))
}

pub async fn delete_product_with_id(
    State(pool): State<Arc<PgPool>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let service = create_product_service(pool);
    service.delete_product_from_id(id).await?;
    Ok(StatusCode::NO_CONTENT)
}