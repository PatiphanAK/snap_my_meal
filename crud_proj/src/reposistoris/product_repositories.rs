use sqlx::{PgPool, Error, QueryBuilder, Postgres};
use crate::models::{pagination::Pagination, products::Product};

pub async fn get_product_list_repo(pool: &PgPool,pagination: Pagination,) -> Result<Vec<Product>, Error> {
    let current_limit = pagination.limit.unwrap_or(10) as i64;
    let current_offset = pagination.offset.unwrap_or(0) as i64;

    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
        r#"
        SELECT
            id,
            name,
            brand,
            image_url,
            serving_size_grams,
            calories,
            fat,
            sugar,
            sodium,
            protein,
            carbs,
            saturated_fat,
            cholesterol,
            vitamin_c,
            calcium,
            vitamin_b1,
            vitamin_a,
            is_upf,
            is_healthier
        FROM products
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
        query_builder.push_bind(pattern);
        query_builder.push(" OR LOWER(brand) LIKE ");
        query_builder.push_bind(pattern);
        query_builder.push(" OR LOWER(id::text) LIKE ");
        query_builder.push_bind(pattern);
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
        .await?;

    Ok(products_result)
}
