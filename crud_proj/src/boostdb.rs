use dotenv::dotenv;
use std::env;
use sqlx::{PgPool, postgres::PgPoolOptions, Error as SqlxError};

#[derive(Debug)]
pub struct DbConfig {
    user: String,
    password: String,
    dbname: String,
    host: String,
    port: u16,
    max_connections: u32,
}

impl DbConfig {
    pub fn from_env() -> Self {
        dotenv().ok();
        
        let user = env::var("POSTGRES_USER").expect("POSTGRES_USER must be set");
        let password = env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set");
        let dbname = env::var("POSTGRES_DB").expect("POSTGRES_DB must be set");
        let host = env::var("DB_URL").unwrap_or_else(|_| "localhost".to_string());
        let port = env::var("DB_PORT")
            .unwrap_or_else(|_| "5432".to_string())
            .parse()
            .expect("DB_PORT must be a valid number");
        let max_connections = env::var("DB_MAX_CONNECTIONS")
            .unwrap_or_else(|_| "10".to_string())
            .parse()
            .expect("DB_MAX_CONNECTIONS must be a valid number");

        DbConfig {
            user,
            password,
            dbname,
            host,
            port,
            max_connections,
        }
    }

    pub fn database_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.dbname
        )
    }
}


pub async fn create_pool_with_options(config: DbConfig) -> Result<PgPool, SqlxError> {
    let pool = PgPoolOptions::new()
        .max_connections(config.max_connections)
        .connect(&config.database_url())
        .await?;
    
    Ok(pool)
}

pub async fn test_connection(pool: &PgPool) -> Result<(), SqlxError> {
    sqlx::query("SELECT 1")
        .execute(pool)
        .await?;
    
    println!("Database connection successful!");
    Ok(())
}