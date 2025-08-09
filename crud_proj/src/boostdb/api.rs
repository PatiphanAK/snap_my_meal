// use sqlx::{PgPool, Error as SqlxError};
// use super::config::config::DbConfig;
// use super::{pool, health};

// pub async fn create_pool_with_options(config: DbConfig) -> Result<PgPool, SqlxError> {
//     pool::pool::create_pool(config).await
// }

// pub async fn test_connection(pool: &PgPool) -> Result<(), SqlxError> {
//     health::health::basic_health_check(pool).await
// }

// pub async fn close_pool(pool: PgPool) {
//     pool::pool::close_pool(pool).await
// }