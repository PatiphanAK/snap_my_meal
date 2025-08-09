use sqlx::{PgPool, Error as SqlxError};

pub mod pool;
pub mod config;
pub mod health;
pub mod init;
pub mod api;

#[derive(Debug)]
pub struct Database {
    pool: PgPool,
    config: config::config::DbConfig,
}

impl Database {
    #[allow(dead_code)]
    pub async fn new(config: config::config::DbConfig) -> Result<Self, SqlxError> {
        let pool = pool::pool::create_pool(config.clone()).await?;
        Ok(Database { pool, config })
    }

    pub async fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let config = config::config::DbConfig::from_env()?;
        let pool = pool::pool::create_pool(config.clone()).await?;
        Ok(Database { pool, config })
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub fn config(&self) -> &config::config::DbConfig {
        &self.config
    }

    pub async fn health_check(&self) -> Result<health::health::HealthStatus, SqlxError> {
        health::health::detailed_health_check(&self.pool).await
    }
    #[allow(dead_code)]
    pub async fn test_connection(&self) -> Result<(), SqlxError> {
        health::health::basic_health_check(&self.pool).await
    }
    #[allow(dead_code)]
    pub async fn close(self) {
        pool::pool::close_pool(self.pool).await;
    }
}