pub mod pool;
pub mod config;
pub mod health;
pub mod init;

#[derive(Debug)]
pub struct Database {
    pool: PgPool,
    config: config::DbConfig,
}

impl Database {
    pub async fn new(config: config::DbConfig) -> Result<Self, SqlxError> {
        let pool = pool::create_pool(config.clone()).await?;
        Ok(Database { pool, config })
    }

    pub async fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let config = config::DbConfig::from_env()?;
        let pool = pool::create_pool(config.clone()).await?;
        Ok(Database { pool, config })
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub fn config(&self) -> &config::DbConfig {
        &self.config
    }

    pub async fn health_check(&self) -> Result<health::HealthStatus, SqlxError> {
        health::detailed_health_check(&self.pool).await
    }

    pub async fn test_connection(&self) -> Result<(), SqlxError> {
        health::basic_health_check(&self.pool).await
    }

    pub async fn close(self) {
        pool::close_pool(self.pool).await;
    }
}