use sqlx::{PgPool};
use std::sync::Arc;
use crate::models::{pagination::Pagination, products::Product};
use crate::errors::AppError;
use async_trait::async_trait;
use tracing;

#[async_trait]
pub trait ProductRepositoryTrait:Send + Sync  {
    async fn get_product_list(&self, pagination: Pagination) -> Result<Vec<Product>, AppError>;
}

pub struct ProductRepository {
     pool: Arc<PgPool>,
}

impl ProductRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        ProductRepository { pool }
    }
}

#[async_trait]
impl ProductRepositoryTrait for ProductRepository {
    async fn get_product_list(&self, pagination: Pagination) -> Result<Vec<Product>, AppError> {
        let current_limit = pagination.limit.unwrap_or(10) as i64;
        let current_offset = pagination.offset.unwrap_or(0) as i64;

        let (base_query, search_pattern) = if let Some(search) = &pagination.search {
            let query = r#"
                SELECT 
                    p.*, 
                    COALESCE(
                        ARRAY_AGG(c.name) FILTER (WHERE c.name IS NOT NULL), 
                        ARRAY[]::TEXT[]
                    ) AS categories
                FROM products p
                LEFT JOIN product_category pc ON p.id = pc.product_id
                LEFT JOIN categories c ON pc.category_id = c.id
                WHERE (p.name ILIKE $1 OR p.brand ILIKE $1)
                GROUP BY p.id 
                ORDER BY p.name 
                LIMIT $2 OFFSET $3
            "#;
            let pattern = format!("%{}%", search.to_lowercase());
            (query, Some(pattern))
        } else {
            let query = r#"
                SELECT 
                    p.*, 
                    COALESCE(
                        ARRAY_AGG(c.name) FILTER (WHERE c.name IS NOT NULL), 
                        ARRAY[]::TEXT[]
                    ) AS categories
                FROM products p
                LEFT JOIN product_category pc ON p.id = pc.product_id
                LEFT JOIN categories c ON pc.category_id = c.id
                GROUP BY p.id 
                ORDER BY p.name 
                LIMIT $1 OFFSET $2
            "#;
            (query, None)
        };

        let mut query_builder = sqlx::query_as::<_, Product>(base_query);

        if let Some(pattern) = search_pattern {
            query_builder = query_builder.bind(pattern);
        }

        let products_result = query_builder
            .bind(current_limit)
            .bind(current_offset)
            .fetch_all(&*self.pool)
            .await;

        products_result.map_err(|e| {
            tracing::error!("Error fetching products: {:?}", e);
            AppError::DatabaseError(e)
        })
    }
}
