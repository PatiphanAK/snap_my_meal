use sqlx::{PgPool, postgres::PgPoolOptions, Error as SqlxError};
use std::time::Duration;
use tracing::{info};
use super::config::config::DbConfig;

pub mod pool {
    use super::*;

    pub async fn create_pool(config: DbConfig) -> Result<PgPool, SqlxError> {
        // Validate configuration
        if let Err(e) = config.validate() {
            return Err(SqlxError::Configuration(e.into()));
        }

        info!("{}", config.display_info());
        
        let pool = PgPoolOptions::new()
            .max_connections(config.max_connections)
            .min_connections(config.min_connections)
            .acquire_timeout(Duration::from_secs(config.acquire_timeout))
            .idle_timeout(Duration::from_secs(config.idle_timeout))
            .test_before_acquire(true)
            .connect(&config.database_url())
            .await?;
        
        Ok(pool)
    }

    pub async fn close_pool(pool: PgPool) {
        info!("Closing database pool...");
        pool.close().await;
        info!("Database pool closed successfully");
    }
}