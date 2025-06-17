use systematics_api::{start_server, SurrealStorage, SystematicsError};
use std::env;

#[tokio::main]
async fn main() -> Result<(), SystematicsError> {
    // Get configuration from environment variables
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3001".to_string())
        .parse::<u16>()
        .unwrap_or(3001);
        
    let db_path = env::var("SYSTEMATICS_DB_PATH")
        .unwrap_or_else(|_| "./data/systematics.db".to_string());
    
    println!("🚀 Starting SysteMaster API Server");
    println!("   Port: {}", port);
    println!("   Database: {}", db_path);
    
    // Initialize storage with detailed logging
    println!("📡 Initializing database connection...");
    let storage = match SurrealStorage::new(&db_path).await {
        Ok(storage) => {
            println!("✅ Database connected successfully");
            storage
        }
        Err(e) => {
            eprintln!("❌ Database connection failed: {}", e);
            return Err(e);
        }
    };
    
    // Start the server with detailed logging
    println!("🌐 Starting HTTP server...");
    if let Err(e) = start_server(storage, port).await {
        eprintln!("❌ Server failed to start: {}", e);
        return Err(e);
    }
    
    Ok(())
} 