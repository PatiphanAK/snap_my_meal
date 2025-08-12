use sqlx::{PgPool};
use std::sync::Arc;
use crate::models::{pagination::Pagination, products::{Product, ProductForm, ProductResponse}};
use crate::errors::AppError;
use async_trait::async_trait;
use tracing::{self, debug, error, info};

#[async_trait]
pub trait ProductRepositoryTrait:Send + Sync  {
    async fn get_product_list(&self, pagination: Pagination) -> Result<Vec<ProductResponse>, AppError>;
    async fn create_product_with_categories(&self, product: ProductForm ) -> Result<ProductResponse, AppError>;
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

        
        let mut inserted_categories = Vec::new();
        
        if !product.categories.is_empty() {
            debug!("📝 Inserting {} categories for product {}", 
                    product.categories.len(), product_row.id);

            for (index, category_id) in product.categories.iter().enumerate() {
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
                    inserted_categories.push(category_id.clone());
                }
            }
        }

        // Commit transaction
        if let Err(e) = tx.commit().await {
            error!("❌ Failed to commit transaction: {:?}", e);
            return Err(AppError::DatabaseError(e));
        }

        debug!("🎉 Transaction committed successfully!");
        
        
        let response = ProductResponse {
            id: product_row.id,
            name: product_row.name,
            brand: product_row.brand,
            image_url: product_row.image_url,
            serving_size_grams: product_row.serving_size_grams,
            calories: product_row.calories,
            fat: product_row.fat,
            sugar: product_row.sugar,
            sodium: product_row.sodium,
            protein: product_row.protein,
            carbs: product_row.carbs,
            saturated_fat: product_row.saturated_fat,
            cholesterol: product_row.cholesterol,
            vitamin_c: product_row.vitamin_c,
            calcium: product_row.calcium,
            vitamin_b1: product_row.vitamin_b1,
            vitamin_a: product_row.vitamin_a,
            price: product_row.price,
            is_upf: product_row.is_upf,
            is_healthier: product_row.is_healthier,
            categories: inserted_categories,
        };
        
        Ok(response)
    }
}
