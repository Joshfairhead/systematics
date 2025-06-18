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

    println!("⏳ Waiting for API server to start...");
    thread::sleep(Duration::from_secs(3));

    println!("🌐 Starting Frontend Server in background...");
    start_frontend_server();

    println!("⏳ Waiting for frontend server to start...");
    thread::sleep(Duration::from_secs(3));

    println!();
    println!("✅ SysteMaster Development Environment Ready!");
    println!("=============================================");
    println!("🔗 Frontend:  http://localhost:8081");
    println!("🔗 API:       http://localhost:3001");
    println!("📚 API Docs:  http://localhost:3001/health");
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
    Command::new("cargo")
        .args(&["run", "--bin", "server", "--features", "server"])
        .current_dir("api")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to start API server");
}

fn start_frontend_server() {
    Command::new("trunk")
        .args(&["serve", "--port", "8081"])
        .current_dir("frontend")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to start frontend server");
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