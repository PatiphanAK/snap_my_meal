use crate::{errors::AppError, models::products::Product};


pub async fn list_products(
    limit: u32,
    offset: u32,
    search: Option<String>,
) -> Result<Vec<Product>, AppError> {
    println!("Limit: {:?}", limit);
    println!("Offset: {:?}", offset);
    println!("Search: {:?}", search);

    let p1 = Product {
        id: "id_001".to_string(),
        name: "product_001".to_string(),
        ..Default::default()
    };

    let p2 = Product {
        id: "id_002".to_string(),
        name: "product_002".to_string(),
        calories: 150,
        ..Default::default()
    };

    let products = vec![p1, p2];
    Ok(products)
}
