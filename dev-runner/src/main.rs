use std::process::{Command, Stdio};
use std::env;
use std::thread;
use std::time::Duration;

fn main() {
    println!("🚀 Starting SysteMaster Development Environment");
    println!("===============================================");

    // Get the workspace root (parent of dev-runner)
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let workspace_root = if current_dir.ends_with("dev-runner") {
        current_dir.parent().unwrap().to_path_buf()
    } else {
        current_dir
    };

    // Change to workspace root
    env::set_current_dir(&workspace_root).expect("Failed to change to workspace root");

    println!("📡 Starting API Server in background...");
    start_api_server();

    println!("⏳ Waiting for API server to be ready...");
    println!("   (Note: First compilation can take 2-3 minutes due to SurrealDB dependencies)");
    if !wait_for_api_server(180) { // Increased timeout to 3 minutes
        println!("❌ API server failed to start within 3 minutes");
        println!("   Try running 'cd api && cargo run --bin server --features server' manually to see detailed errors");
        return;
    }
    println!("✅ API Server is ready!");

    println!("🌐 Starting Frontend Server in background...");
    start_frontend_server();

    println!("⏳ Waiting for frontend server to be ready...");
    if !wait_for_frontend_server(30) {
        println!("❌ Frontend server failed to start within 30 seconds");
        return;
    }
    println!("✅ Frontend Server is ready!");

    println!();
    println!("🎉 SysteMaster Development Environment Ready!");
    println!("=============================================");
    println!("🔗 Frontend:  http://localhost:8081");
    println!("🔗 API:       http://localhost:3001");
    println!("📚 API Health: http://localhost:3001/health");
    println!();
    
    // Ask if user wants CLI
    println!("🤔 Start CLI interface? (y/N):");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    
    if input.trim().to_lowercase().starts_with('y') {
        println!("🖥️  Starting CLI...");
        start_cli();
    } else {
        println!("💡 Servers running in background. To stop:");
        println!("   - Press Ctrl+C to stop this process");
        println!("   - Or use: pkill -f 'cargo run --bin server'");
        println!("   - Or use: pkill -f 'trunk serve'");
        println!();
        println!("🎉 Development environment ready! Happy coding! 🦀");
        
        // Keep the process alive so user can see the output
        println!("Press Enter to exit...");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");
    }
}

fn start_api_server() {
    println!("   Compiling API server (this may take a while on first run)...");
    Command::new("cargo")
        .args(&["run", "--bin", "server", "--features", "server"])
        .current_dir("api")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start API server");
}

fn start_frontend_server() {
    println!("   Starting frontend development server...");
    Command::new("trunk")
        .args(&["serve", "--port", "8081"])
        .current_dir("frontend")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start frontend server");
}

fn wait_for_api_server(timeout_seconds: u64) -> bool {
    for i in 0..timeout_seconds {
        if i > 0 && i % 15 == 0 { // Show progress every 15 seconds instead of 5
            println!("   Still compiling/starting API server... ({}/{}s)", i, timeout_seconds);
        }
        
        if check_api_health() {
            return true;
        }
        
        thread::sleep(Duration::from_secs(1));
    }
    false
}

fn wait_for_frontend_server(timeout_seconds: u64) -> bool {
    for i in 0..timeout_seconds {
        if i > 0 && i % 5 == 0 {
            println!("   Still waiting for frontend server... ({}/{}s)", i, timeout_seconds);
        }
        
        if check_frontend_health() {
            return true;
        }
        
        thread::sleep(Duration::from_secs(1));
    }
    false
}

fn check_api_health() -> bool {
    match ureq::get("http://localhost:3001/health")
        .timeout(std::time::Duration::from_secs(2))
        .call()
    {
        Ok(response) => response.status() == 200,
        Err(_) => false,
    }
}

fn check_frontend_health() -> bool {
    match ureq::get("http://localhost:8081")
        .timeout(std::time::Duration::from_secs(2))
        .call()
    {
        Ok(response) => response.status() == 200,
        Err(_) => false,
    }
}

fn start_cli() {
    let status = Command::new("cargo")
        .args(&["run"])
        .current_dir("cli")
        .status()
        .expect("Failed to start CLI");
    
    if !status.success() {
        println!("⚠️  CLI exited with error");
    }
} 