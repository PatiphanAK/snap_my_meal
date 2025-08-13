use std::sync::Arc;
use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    errors::AppError, 
    models::{pagination::Pagination, products::{ ProductForm, ProductResponse}},
    repositories::product_repositories::{ProductRepository, ProductRepositoryTrait}
};

#[async_trait]
pub trait ProductServiceTrait: Send + Sync {
    async fn list_products(&self, pagination: Pagination) -> Result<Vec<ProductResponse>, AppError>;
    async fn add_product(&self, product: ProductForm) -> Result<ProductResponse, AppError>;
    async fn get_product_from_id(&self, id: Uuid) -> Result<Option<ProductResponse>, AppError>;
    async fn update_product_from_id(&self, id: Uuid, product: ProductForm) -> Result<Option<ProductResponse>, AppError>;
    async fn delete_product_from_id(&self, id: Uuid) -> Result<(), AppError>;
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
    async fn list_products(&self, pagination: Pagination) -> Result<Vec<ProductResponse>, AppError> {
        self.repo.get_product_list(pagination).await
    }
    
    async fn add_product(&self, product: ProductForm) -> Result<ProductResponse, AppError> {
        self.repo.create_product_with_categories(product).await
    }
    
    async fn get_product_from_id(&self, id: Uuid) -> Result<Option<ProductResponse>, AppError> {
        match self.repo.get_product_by_id(id).await {
            Ok(product) => Ok(Some(product)),
            Err(AppError::NotFound) => Ok(None),
            Err(e) => Err(e),
        }
    }
    
    async fn update_product_from_id(&self, id: Uuid, product: ProductForm) -> Result<Option<ProductResponse>, AppError> {
        match self.repo.update_product_by_id(id, product).await {
            Ok(product) => Ok(Some(product)),
            Err(AppError::NotFound) => Ok(None),
            Err(e) => Err(e),
        }
    }
    
    async fn delete_product_from_id(&self, id: Uuid) -> Result<(), AppError> {
        let affected_rows = self.repo.delete_product_by_id(id).await?;
        if affected_rows == 0 {
            return Err(AppError::NotFound);
        }
        Ok(())
    }
}