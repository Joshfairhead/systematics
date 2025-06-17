use systematics_api::{start_server, SurrealStorage, SystematicsError};
use std::env;

#[tokio::main]
async fn main() -> Result<(), SystematicsError> {
    // Get configuration from environment variables
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .unwrap_or(3000);
        
    let db_path = env::var("SYSTEMATICS_DB_PATH")
        .unwrap_or_else(|_| "./data/systematics.db".to_string());
    
    println!("🚀 Starting SysteMaster API Server");
    println!("   Port: {}", port);
    println!("   Database: {}", db_path);
    
    // Initialize storage
    let storage = SurrealStorage::new(&db_path).await?;
    println!("✅ Database connected");
    
    // Start the server
    start_server(storage, port).await?;
    
    Ok(())
} 