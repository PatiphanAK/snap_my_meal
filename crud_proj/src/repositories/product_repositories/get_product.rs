use sqlx::{PgPool, Error};
use crate::models::{pagination::Pagination, products::Product};


pub async fn get_product_list_repo(pool: &PgPool, pagination: Pagination) -> Result<Vec<Product>, Error> {
    let current_limit = pagination.limit.unwrap_or(10) as i64;
    let current_offset = pagination.offset.unwrap_or(0) as i64;

    let search_condition = if let Some(ref search) = pagination.search {
        let pattern = format!("%{}%", search.to_lowercase());
        format!(
            " WHERE (LOWER(p.name) LIKE '{}' OR LOWER(p.brand) LIKE '{}')",
            pattern, pattern
        )
    } else {
        String::new()
    };

    let query = format!(
        r#"
        SELECT 
            p.*, 
            COALESCE(
                ARRAY_AGG(c.name) FILTER (WHERE c.name IS NOT NULL), 
                ARRAY[]::TEXT[]
            ) AS categories
        FROM products p
        LEFT JOIN product_category pc ON p.id = pc.product_id
        LEFT JOIN categories c ON pc.category_id = c.id
        {}
        GROUP BY p.id
        ORDER BY p.name
        LIMIT {} OFFSET {}
        "#,
        search_condition, current_limit, current_offset
    );

    let products_result: Result<Vec<Product>, sqlx::Error> = sqlx::query_as(&query)
        .fetch_all(pool)
        .await;

    match products_result {
        Ok(products) => {
            Ok(products)
        },
        Err(e) => {
            println!("Error fetching products: {:?}", e);
            Err(e)
        }
    }
}