use super::Database;
use tracing::{info};

pub async fn initialize() -> Result<Database, Box<dyn std::error::Error>> {
    info!("🚀 Initializing BoostDB...");
    
    let database = Database::from_env().await?;
    
    // Run health check
    match database.health_check().await {
        Ok(status) => {
            info!("✅ BoostDB initialized successfully!");
            info!("   📊 Pool: {}/{} connections", status.pool_size, database.config().max_connections);
            info!("   💾 Database: {} ({})", status.database_name, status.database_size);
        }
        Err(e) => {
            return Err(format!("Health check failed: {}", e).into());
        }
    }
    
    Ok(database)
}