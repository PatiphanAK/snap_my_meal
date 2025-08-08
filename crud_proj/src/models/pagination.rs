use serde::{Serialize, Deserialize};


#[derive(Debug, Deserialize)]
pub struct Pagination {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
    pub search: Option<String>,
}

#[derive(Serialize)]
pub struct ProductPaginatedResponse<T> {
    pub items: Vec<T>,
    pub total: i32,
    pub limit: i32,
    pub offset: i32,
}