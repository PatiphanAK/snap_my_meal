use sqlx::{PgPool, Error as SqlxError, Row};

pub mod health {
    use super::*;

    #[derive(Debug)]
    pub struct HealthStatus {
        pub connection: bool,
        pub database_name: String,
        pub user: String,
        pub version: String,
        pub table_count: i64,
        pub database_size: String,
        pub pool_size: u32,
        pub idle_connections: usize,
    }

    pub async fn basic_health_check(pool: &PgPool) -> Result<(), SqlxError> {
        sqlx::query("SELECT 1")
            .execute(pool)
            .await?;
        
        println!("✓ Database connection successful!");
        Ok(())
    }

    pub async fn detailed_health_check(pool: &PgPool) -> Result<HealthStatus, SqlxError> {
        println!("=== Database Health Check ===");
        
        // Basic connection test
        sqlx::query("SELECT 1").execute(pool).await?;
        println!("✓ Basic connection: OK");
        
        // Pool status
        println!("✓ Pool connections: {}", pool.size());
        println!("✓ Idle connections: {}", pool.num_idle());
        
        // Database info
        let db_info = sqlx::query("SELECT current_database(), current_user, version()")
            .fetch_one(pool)
            .await?;
        
        let database_name: String = db_info.get(0);
        let user: String = db_info.get(1);
        let full_version: String = db_info.get(2);
        let version = full_version
            .split_whitespace()
            .take(2)
            .collect::<Vec<_>>()
            .join(" ");
        
        println!("✓ Database: {}", database_name);
        println!("✓ User: {}", user);
        println!("✓ Version: {}", version);
        
        // Table count
        let table_count = sqlx::query(
            "SELECT COUNT(*) FROM information_schema.tables 
             WHERE table_schema = 'public'"
        )
        .fetch_one(pool)
        .await?;
        let table_count: i64 = table_count.get(0);
        println!("✓ Tables in public schema: {}", table_count);
        
        // Database size
        let db_size = sqlx::query(
            "SELECT pg_size_pretty(pg_database_size(current_database()))"
        )
        .fetch_one(pool)
        .await?;
        let database_size: String = db_size.get(0);
        println!("✓ Database size: {}", database_size);
        
        println!("=== Health Check Complete ===\n");
        
        Ok(HealthStatus {
            connection: true,
            database_name,
            user,
            version,
            table_count,
            database_size,
            pool_size: pool.size(),
            idle_connections: pool.num_idle(),
        })
    }
}