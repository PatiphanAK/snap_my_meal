pub async fn initialize() -> Result<Database, Box<dyn std::error::Error>> {
    println!("🚀 Initializing BoostDB...");
    
    let database = Database::from_env().await?;
    
    // Run health check
    match database.health_check().await {
        Ok(status) => {
            println!("✅ BoostDB initialized successfully!");
            println!("   📊 Pool: {}/{} connections", status.pool_size, database.config().max_connections);
            println!("   💾 Database: {} ({})", status.database_name, status.database_size);
        }
        Err(e) => {
            return Err(format!("Health check failed: {}", e).into());
        }
    }
    
    Ok(database)
}