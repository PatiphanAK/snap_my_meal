use axum::{extract::Query, Json};
use crate::models::pagination::Pagination;
use crate::models::products::Product;
use crate::errors::AppError;
use crate::services::product_service;


pub async fn get_product_list(
    Query(pagination): Query<Pagination>,
) -> Result<Json<Vec<Product>>, AppError> {
    let limit = pagination.limit.unwrap_or(10);
    let offset = pagination.offset.unwrap_or(0);
    let search = pagination.search.clone();
    let products = product_service::list_products(limit, offset, search).await?;

    Ok(Json(products))
}