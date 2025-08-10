use crate::{errors::AppError, models::products::Product, repositories::product_repositories::get_product};
use crate::{models::pagination::Pagination};
use sqlx::PgPool;

pub async fn list_products(pool: &PgPool,pagination: Pagination) -> Result<Vec<Product>, AppError> {
    let products = get_product::get_product_list_repo(pool, pagination).await?;
    Ok(products)
}
