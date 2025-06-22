use systematics_api::{start_server, SurrealStorage, SystematicsError, DatabaseEnvironment};
use std::env;

#[tokio::main]
async fn main() -> Result<(), SystematicsError> {
    // Get configuration from environment variables
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3001".to_string())
        .parse::<u16>()
        .unwrap_or(3001);
        
    // Start with testing environment by default (safer for UI development)
    let environment = DatabaseEnvironment::Testing;
    let db_path = environment.db_path();
    
    println!("🚀 Starting SysteMaster API Server");
    println!("   Port: {}", port);
    println!("   Environment: {:?}", environment);
    println!("   Database: {}", db_path);
    
    // Initialize storage with environment-based connection
    println!("📡 Initializing database connection...");
    let storage = match SurrealStorage::new_with_environment(environment).await {
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