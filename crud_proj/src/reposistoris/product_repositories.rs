use sqlx::{PgPool, Error, QueryBuilder, Postgres};
use crate::models::{pagination::Pagination, products::Product};

pub async fn get_product_list_repo(pool: &PgPool,pagination: Pagination,) -> Result<Vec<Product>, Error> {
    let current_limit = pagination.limit.unwrap_or(10) as i64;
    let current_offset = pagination.offset.unwrap_or(0) as i64;

    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
                r#"
        SELECT *
        FROM Products
        "#
    );

    // Optional WHERE clause (safe bind)
    let search_pattern = pagination.search.as_ref().map(|query| {
        format!("%{}%", query.to_lowercase())
    });

    if let Some(ref pattern) = search_pattern {
        query_builder.push(" WHERE ");
        query_builder.push("(");
        query_builder.push("LOWER(name) LIKE ");
        query_builder.push_bind(pattern.clone());
        query_builder.push(" OR LOWER(brand) LIKE ");
        query_builder.push_bind(pattern.clone());
        query_builder.push(" OR LOWER(id::text) LIKE ");
        query_builder.push_bind(pattern.clone());
        query_builder.push(")");
    }
    // ORDER BY, LIMIT, OFFSET
    query_builder.push(" ORDER BY name");
    query_builder.push(" LIMIT ");
    query_builder.push_bind(current_limit);
    query_builder.push(" OFFSET ");
    query_builder.push_bind(current_offset);

    // Run query
        let products_result = query_builder
        .build_query_as::<Product>()
        .fetch_all(pool)
        .await; // <-- ไม่ใช้ ?

    match products_result {
        Ok(products) => {
            // ถ้าสำเร็จ ก็ส่งค่า products กลับไป
            Ok(products)
        },
        Err(e) => {
            // ถ้าเกิด error
            println!("Error fetching products: {:?}", e); // <-- ตรงนี้คือส่วนที่ print error
            Err(e) // ส่ง error กลับไปให้ caller
        }
    }
}

// pub async fn get_product_list_repo(
//     pool: &PgPool,
//     pagination: Pagination,
// ) -> Result<Vec<Product>, Error> {
//     let q = "SELECT * FROM products LIMIT $1 OFFSET $2;";
    
//     let product_result = sqlx::query_as::<_, Product>(q)
//         .bind(pagination.limit.unwrap_or(10) as i64)
//         .bind(pagination.offset.unwrap_or(0) as i64)
//         .fetch_all(pool)
//         .await;

//     match product_result {
//         Ok(products) => Ok(products),
//         Err(e) => {
//             println!("Error fetching products: {:?}", e);
//             Err(e)
//         }
//     }
// }