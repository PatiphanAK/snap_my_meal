use serde::{Serialize, Deserialize};


#[derive(Debug, Deserialize, Clone)]
pub struct Pagination {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
    pub search: Option<String>,
}

#[derive(Serialize)]
pub struct TemplateResponse<T> {
    pub items: Vec<T>,
    pub total: usize,
    pub limit: i32,
    pub offset: i32,
}