use uuid::Uuid;
/*
Model ของ Categoris ที่ใช้จัดหมวดหมู่ของ Product
*/
#[derive(Debug, Clone, Serialize, Deserialize, Default, FromRow)]
pub struct Categories {
    pub id: Uuid,
    pub name: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoriesForm {
    pub id: Option<String>,
    pub name: String,
}

pub struct CategoriesWithProductsResponse {
    pub categories: Categories,
    pub products: Vec<Product>,
}