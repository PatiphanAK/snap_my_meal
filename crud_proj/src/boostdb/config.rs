use dotenv::dotenv;
use std::env;


pub mod config {
    use super::*;

    #[derive(Debug, Clone)]
    pub struct DbConfig {
        pub user: String,
        pub password: String,
        pub dbname: String,
        pub host: String,
        pub port: u16,
        pub max_connections: u32,
        pub min_connections: u32,
        pub acquire_timeout: u64,
        pub idle_timeout: u64,
    }

    impl DbConfig {
        pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
            dotenv().ok();
            
            let user = env::var("POSTGRES_USER")
                .map_err(|_| "POSTGRES_USER must be set")?;
            let password = env::var("POSTGRES_PASSWORD")
                .map_err(|_| "POSTGRES_PASSWORD must be set")?;
            let dbname = env::var("POSTGRES_DB")
                .map_err(|_| "POSTGRES_DB must be set")?;
            let host = env::var("DB_HOST")
                .unwrap_or_else(|_| "localhost".to_string());
            let port = env::var("DB_PORT")
                .unwrap_or_else(|_| "5432".to_string())
                .parse::<u16>()
                .map_err(|_| "DB_PORT must be a valid number")?;
            let max_connections = env::var("DB_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "10".to_string())
                .parse::<u32>()
                .map_err(|_| "DB_MAX_CONNECTIONS must be a valid number")?;
            let min_connections = env::var("DB_MIN_CONNECTIONS")
                .unwrap_or_else(|_| "1".to_string())
                .parse::<u32>()
                .map_err(|_| "DB_MIN_CONNECTIONS must be a valid number")?;
            let acquire_timeout = env::var("DB_ACQUIRE_TIMEOUT")
                .unwrap_or_else(|_| "30".to_string())
                .parse::<u64>()
                .map_err(|_| "DB_ACQUIRE_TIMEOUT must be a valid number")?;
            let idle_timeout = env::var("DB_IDLE_TIMEOUT")
                .unwrap_or_else(|_| "600".to_string())
                .parse::<u64>()
                .map_err(|_| "DB_IDLE_TIMEOUT must be a valid number")?;

            Ok(DbConfig {
                user,
                password,
                dbname,
                host,
                port,
                max_connections,
                min_connections,
                acquire_timeout,
                idle_timeout,
            })
        }

        pub fn database_url(&self) -> String {
            format!(
                "postgres://{}:{}@{}:{}/{}",
                self.user, self.password, self.host, self.port, self.dbname
            )
        }
        
        pub fn display_info(&self) -> String {
            format!(
                "Database Config:\n  Host: {}:{}\n  Database: {}\n  User: {}\n  Max Connections: {}\n  Min Connections: {}",
                self.host, self.port, self.dbname, self.user, self.max_connections, self.min_connections
            )
        }

        // Validation method
        pub fn validate(&self) -> Result<(), &'static str> {
            if self.user.is_empty() {
                return Err("Database user cannot be empty");
            }
            if self.password.is_empty() {
                return Err("Database password cannot be empty");
            }
            if self.dbname.is_empty() {
                return Err("Database name cannot be empty");
            }
            if self.max_connections == 0 {
                return Err("Max connections must be greater than 0");
            }
            if self.min_connections > self.max_connections {
                return Err("Min connections cannot be greater than max connections");
            }
            Ok(())
        }
    }
}