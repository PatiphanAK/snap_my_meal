use axum::{extract::{Query, State},Json,};
use sqlx::PgPool;
use std::sync::Arc;
use crate::{
    errors::AppError,
    models::{pagination::Pagination, products::Product},
    services::product_service,
};


pub async fn get_product_list(
    State(pool): State<Arc<PgPool>>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<Vec<Product>>, AppError> {
    // Validate pagination parameters
    // pagination.validate()?;
    
    // ส่ง pool ไปยัง product_service
    let products = product_service::list_products(&pool, pagination).await?;
    Ok(Json(products))
}