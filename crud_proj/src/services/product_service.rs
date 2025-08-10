use std::sync::Arc;
use async_trait::async_trait;

use crate::{
    errors::AppError, 
    models::{products::Product, pagination::Pagination},
    repositories::product_repositories::{ProductRepository, ProductRepositoryTrait}
};

#[async_trait]
pub trait ProductServiceTrait: Send + Sync {
    async fn list_products(&self, pagination: Pagination) -> Result<Vec<Product>, AppError>;
}

pub struct ProductService {
    repo: Arc<dyn ProductRepositoryTrait + Send + Sync>,
}

#[allow(dead_code)]
impl ProductService {
    pub fn new(repo: Arc<dyn ProductRepositoryTrait + Send + Sync>) -> Self {
        Self { repo }
    }
    pub fn with_repository(repo: ProductRepository) -> Self {
        Self {
            repo: Arc::new(repo),
        }
    }
}

#[async_trait]
impl ProductServiceTrait for ProductService {
    async fn list_products(&self, pagination: Pagination) -> Result<Vec<Product>, AppError> {
        self.repo.get_product_list(pagination).await
    }
}