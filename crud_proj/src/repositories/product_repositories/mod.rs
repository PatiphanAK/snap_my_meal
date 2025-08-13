use async_trait::async_trait;
use sqlx::{PgPool};
use std::sync::Arc;
use uuid::Uuid;
use tracing::{debug, error, info, warn};

use crate::{
    errors::AppError,
    models::{
        pagination::Pagination,
        products::{Product, ProductForm, ProductResponse}
    }
};

#[async_trait]
pub trait ProductRepositoryTrait: Send + Sync {
    async fn get_product_list(&self, pagination: Pagination) -> Result<Vec<ProductResponse>, AppError>;
    async fn create_product_with_categories(&self, product: ProductForm) -> Result<ProductResponse, AppError>;
    async fn get_product_by_id(&self, id: Uuid) -> Result<ProductResponse, AppError>;
    async fn update_product_by_id(&self, id: Uuid, product: ProductForm) -> Result<ProductResponse, AppError>;
    async fn delete_product_by_id(&self, id: Uuid) -> Result<u64, AppError>;
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
    async fn get_product_list(&self, pagination: Pagination) -> Result<Vec<ProductResponse>, AppError> {
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

        let mut query_builder = sqlx::query_as::<_, ProductResponse>(base_query);

        if let Some(pattern) = search_pattern {
            query_builder = query_builder.bind(pattern);
        }

        let products_result = query_builder
            .bind(current_limit)
            .bind(current_offset)
            .fetch_all(&*self.pool)
            .await;
        
        if let Ok(products) = &products_result {
            info!("Successfully fetched {} products.", products.len());
        }
        products_result.map_err(|e| {
            error!("Error fetching products: {:?}", e);
            AppError::DatabaseError(e)
        })
    }

    async fn create_product_with_categories(&self, product: ProductForm) -> Result<ProductResponse, AppError> {
        let mut tx = self.pool.begin().await.map_err(|e| {
            error!("❌ Failed to begin transaction: {:?}", e);
            AppError::DatabaseError(e)
        })?;

        let product_row = match sqlx::query_as::<_, Product>(
            r#"
            INSERT INTO products (
                name, brand, image_url, serving_size_grams, calories, fat, sugar, 
                sodium, protein, carbs, saturated_fat, cholesterol, vitamin_c, 
                calcium, vitamin_b1, vitamin_a, price, is_upf, is_healthier
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19
            )
            RETURNING *
            "#
        )
        .bind(&product.name)
        .bind(&product.brand)
        .bind(&product.image_url)
        .bind(product.serving_size_grams)
        .bind(product.calories)
        .bind(product.fat)
        .bind(product.sugar)
        .bind(product.sodium)
        .bind(product.protein)
        .bind(product.carbs)
        .bind(product.saturated_fat)
        .bind(product.cholesterol)
        .bind(product.vitamin_c)
        .bind(product.calcium)
        .bind(product.vitamin_b1)
        .bind(product.vitamin_a)
        .bind(product.price)
        .bind(product.is_upf)
        .bind(product.is_healthier)
        .fetch_one(&mut *tx)
        .await {
            Ok(row) => {
                debug!("✅ Product created successfully: id={}, name={}", row.id, row.name);
                row
            },
            Err(e) => {
                error!("❌ Failed to insert product: {:?}", e);
                error!("📊 Product data: name={:?}, brand={:?}", product.name, product.brand);            
                if let Err(rollback_err) = tx.rollback().await {
                    error!("⚠️  Failed to rollback transaction: {:?}", rollback_err);
                } else {
                    info!("🔄 Transaction rolled back successfully");
                }
                
                return Err(AppError::DatabaseError(e));
            }
        };

        // Insert categories with better error handling
        let mut inserted_categories = Vec::new();
        
        if !product.categories_ids.is_empty() {
            debug!("📝 Inserting {} categories for product {}", 
                    product.categories_ids.len(), product_row.id);

            for (index, category_id) in product.categories_ids.iter().enumerate() {
                if let Err(e) = sqlx::query(
                    "INSERT INTO product_category (product_id, category_id) VALUES ($1, $2)"
                )
                .bind(&product_row.id)
                .bind(category_id)
                .execute(&mut *tx)
                .await {
                    error!("❌ Failed to insert category {} (index {}): {:?}", 
                            category_id, index, e);
                    error!("📊 Context: product_id={}, category_id={}", 
                            product_row.id, category_id);

                    if let Err(rollback_err) = tx.rollback().await {
                        error!("⚠️  Failed to rollback transaction: {:?}", rollback_err);
                    } else {
                        info!("🔄 Transaction rolled back successfully");
                    }
                    
                    return Err(AppError::DatabaseError(e));
                } else {
                    debug!("✅ Category {} linked to product {}", category_id, product_row.id);
                    // 🔧 FIX: Get category name instead of storing UUID
                    inserted_categories.push(format!("{}", category_id)); // This should be category name
                }
            }
        }

        // Commit transaction
        if let Err(e) = tx.commit().await {
            error!("❌ Failed to commit transaction: {:?}", e);
            return Err(AppError::DatabaseError(e));
        }

        debug!("🎉 Transaction committed successfully!");
        
        // 🔧 BETTER: Fetch the created product with proper categories
        self.get_product_by_id(product_row.id).await
    }

    async fn get_product_by_id(&self, id: Uuid) -> Result<ProductResponse, AppError> {
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
            WHERE p.id = $1
            GROUP BY p.id
        "#;

        let product_result = sqlx::query_as::<_, ProductResponse>(query)
            .bind(id)
            .fetch_one(&*self.pool)
            .await;

        match product_result {
            Ok(product) => {
                info!("Successfully fetched product with id: {}", id);
                Ok(product)
            }
            Err(sqlx::Error::RowNotFound) => {
                warn!("Product with id {} not found", id);
                Err(AppError::NotFound)
            }
            Err(e) => {
                error!("Error fetching product by id {}: {:?}", id, e);
                Err(AppError::DatabaseError(e))
            }
        }
    }

    async fn update_product_by_id(&self, id: Uuid, product: ProductForm) -> Result<ProductResponse, AppError> {
        let mut tx = self.pool.begin().await
            .map_err(|e| AppError::DatabaseError(e))?;

        // 🔧 FIX: Check if product exists first
        let exists_query = "SELECT EXISTS(SELECT 1 FROM products WHERE id = $1)";
        let exists: bool = sqlx::query_scalar(exists_query)
            .bind(id)
            .fetch_one(&mut *tx)
            .await
            .map_err(|e| AppError::DatabaseError(e))?;

        if !exists {
            let _ = tx.rollback().await;
            return Err(AppError::NotFound);
        }

        // Update product - 🔧 FIX: Use proper UPDATE with all fields
        let product_query = r#"
            UPDATE products 
            SET 
                name = $2,
                brand = $3,
                image_url = $4,
                serving_size_grams = $5,
                calories = $6,
                fat = $7,
                sugar = $8,
                sodium = $9,
                protein = $10,
                carbs = $11,
                saturated_fat = $12,
                cholesterol = $13,
                vitamin_c = $14,
                calcium = $15,
                vitamin_b1 = $16,
                vitamin_a = $17,
                price = $18,
                is_upf = $19,
                is_healthier = $20
            WHERE id = $1
        "#;

        let update_result = sqlx::query(product_query)
            .bind(id)
            .bind(&product.name)
            .bind(&product.brand)
            .bind(&product.image_url)
            .bind(product.serving_size_grams)
            .bind(product.calories)
            .bind(product.fat)
            .bind(product.sugar)
            .bind(product.sodium)
            .bind(product.protein)
            .bind(product.carbs)
            .bind(product.saturated_fat)
            .bind(product.cholesterol)
            .bind(product.vitamin_c)
            .bind(product.calcium)
            .bind(product.vitamin_b1)
            .bind(product.vitamin_a)
            .bind(product.price)
            .bind(product.is_upf)
            .bind(product.is_healthier)
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::DatabaseError(e))?;

        if update_result.rows_affected() == 0 {
            let _ = tx.rollback().await;
            return Err(AppError::NotFound);
        }

        // Update categories
        if !product.categories_ids.is_empty() {
            // Delete existing categories
            let delete_query = "DELETE FROM product_category WHERE product_id = $1";
            sqlx::query(delete_query)
                .bind(id)
                .execute(&mut *tx)
                .await
                .map_err(|e| AppError::DatabaseError(e))?;

            // Insert new categories
            let insert_query = "INSERT INTO product_category (product_id, category_id) VALUES ($1, $2)";
            
            for category_id in &product.categories_ids {
                sqlx::query(insert_query)
                    .bind(id)
                    .bind(category_id)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| AppError::DatabaseError(e))?;
            }
        }

        // Commit transaction
        tx.commit().await
            .map_err(|e| AppError::DatabaseError(e))?;

        // Fetch updated product
        self.get_product_by_id(id).await
    }

    // 🔧 ADD: Missing delete method implementation
    async fn delete_product_by_id(&self, id: Uuid) -> Result<u64, AppError> {
        let mut tx = self.pool.begin().await
            .map_err(|e| AppError::DatabaseError(e))?;

        // Delete categories first (foreign key constraint)
        sqlx::query("DELETE FROM product_category WHERE product_id = $1")
            .bind(id)
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::DatabaseError(e))?;

        // Delete product
        let result = sqlx::query("DELETE FROM products WHERE id = $1")
            .bind(id)
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::DatabaseError(e))?;

        let affected_rows = result.rows_affected();

        // Commit transaction
        tx.commit().await
            .map_err(|e| AppError::DatabaseError(e))?;

        info!("Successfully deleted product with id: {}, affected rows: {}", id, affected_rows);
        Ok(affected_rows)
    }
}