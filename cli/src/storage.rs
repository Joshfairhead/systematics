use crate::api_client::{ApiClient, StructureIdValue};
use systematics_api::{SystematicsError, SystematicStructure};
use std::collections::HashMap;
use std::process::{Command, Stdio};
use std::time::Duration;
use tokio::time::sleep;
use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct StorageArgs {
    #[command(subcommand)]
    pub command: StorageCommand,
}

#[derive(Debug, Subcommand)]
pub enum StorageCommand {
    /// List all stored structures
    List,
    /// Search structures by term or name
    Search {
        /// Search query
        query: String,
    },
    /// View structure details
    View {
        /// Structure ID
        id: String,
    },
    /// Delete a structure
    Delete {
        /// Structure ID
        id: String,
    },
    /// Show related structures
    Related {
        /// Structure ID
        id: String,
    },
    /// Find structures containing a specific term
    FindTerm {
        /// Term to search for
        term: String,
    },
    /// Show structure graph
    Graph {
        /// Structure ID
        id: String,
    },
    /// Update structure metadata
    Metadata {
        /// Structure ID
        id: String,
        /// Key-value pairs (key=value)
        #[arg(short, long, value_parser = parse_key_val)]
        pairs: Vec<(String, String)>,
    },
    /// Initialize the database
    Init,
    /// Database statistics
    Stats,
    /// Export database to JSON file
    Export {
        /// Output file path
        #[arg(short, long, default_value = "systematics_export.json")]
        output: String,
    },
}

pub struct ApiStorage {
    api_client: ApiClient,
}

impl ApiStorage {
    pub async fn new() -> Result<Self, SystematicsError> {
        let api_url = std::env::var("SYSTEMATICS_API_URL")
            .unwrap_or_else(|_| "http://localhost:3001".to_string());
        
        let api_client = ApiClient::new(Some(api_url.clone()));
        
        // Test connection - if it fails, try to start the server
        if !api_client.health_check().await.unwrap_or(false) {
            println!("🔄 API server not running. Starting API server...");
            Self::start_api_server().await?;
            
            // Wait for server to be ready
            println!("⏳ Waiting for API server to be ready...");
            let mut attempts = 0;
            let max_attempts = 30; // 30 seconds timeout
            
            while attempts < max_attempts {
                if api_client.health_check().await.unwrap_or(false) {
                    break;
                }
                sleep(Duration::from_secs(1)).await;
                attempts += 1;
            }
            
            if attempts >= max_attempts {
                return Err(SystematicsError::Storage(
                    "API server failed to start within 30 seconds".to_string()
                ));
            }
        }
        
        // Final connection test
        if api_client.health_check().await? {
            println!("🌐 Connected to SysteMaster API at: {}", api_url);
        } else {
            return Err(SystematicsError::Storage(format!(
                "Failed to connect to API server at {}. Is the server running?", api_url
            )));
        }
        
        Ok(Self { api_client })
    }

    async fn start_api_server() -> Result<(), SystematicsError> {
        // Find the API directory relative to the CLI
        let api_path = std::env::current_dir()
            .map_err(|e| SystematicsError::Storage(format!("Failed to get current directory: {}", e)))?
            .parent()
            .ok_or_else(|| SystematicsError::Storage("Cannot find parent directory".to_string()))?
            .join("api");
        
        if !api_path.exists() {
            return Err(SystematicsError::Storage(
                "API directory not found. Make sure you're running from the CLI directory in the SysteMaster workspace.".to_string()
            ));
        }
        
        // Start the API server in the background
        let mut cmd = Command::new("cargo");
        cmd.args(&["run", "--bin", "server", "--features", "server"])
            .current_dir(&api_path)
            .stdout(Stdio::null()) // Suppress output
            .stderr(Stdio::null())
            .stdin(Stdio::null());
        
        // On Windows, we need to handle process creation differently
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW flag
        }
        
        let child = cmd.spawn()
            .map_err(|e| SystematicsError::Storage(format!("Failed to start API server: {}", e)))?;
        
        // We don't wait for the child process - it runs in the background
        std::mem::forget(child); // Prevent the child from being dropped and killed
        
        println!("🚀 API server starting in background...");
        Ok(())
    }

    pub async fn handle_command(&self, args: &StorageArgs) -> Result<(), SystematicsError> {
        match &args.command {
            StorageCommand::List => self.list_structures().await,
            StorageCommand::Search { query } => self.search_structures(query).await,
            StorageCommand::View { id } => self.view_structure(id).await,
            StorageCommand::Delete { id } => self.delete_structure(id).await,
            StorageCommand::Related { id } => self.show_related_structures(id).await,
            StorageCommand::FindTerm { term } => self.find_term_usage(term).await,
            StorageCommand::Graph { id } => self.show_structure_graph(id).await,
            StorageCommand::Metadata { id, pairs } => {
                let metadata: HashMap<String, String> = pairs.iter().cloned().collect();
                self.update_metadata(id, metadata).await
            },
            StorageCommand::Init => self.init_database().await,
            StorageCommand::Stats => self.show_stats().await,
            StorageCommand::Export { output } => self.export_database(output).await,
        }
    }

    async fn list_structures(&self) -> Result<(), SystematicsError> {
        let structures = self.api_client.list_structures().await?;
        
        if structures.is_empty() {
            println!("📭 No structures found in the database.");
            return Ok(());
        }

        println!("📚 Stored Structures ({} total):", structures.len());
        println!("{}", "─".repeat(80));
        
        for structure in structures {
            println!("🔹 {} ({})", structure.name, structure.structure_type);
            println!("  Terms: {}", structure.terms.join(" → "));
            
            // Show connectives if they exist
            display_connectives(&structure.connectives, &structure.terms);
            
            if let Some(desc) = &structure.description {
                println!("  Description: {}", desc);
            }
            println!("  ─────────────────────────────────────────");
            println!("  ID: {} | Created: {}", get_structure_id_string(&structure.id), structure.created_at);
            println!();
        }
        
        Ok(())
    }

    async fn search_structures(&self, query: &str) -> Result<(), SystematicsError> {
        let structures = self.api_client.search_structures(query).await?;
        
        if structures.is_empty() {
            println!("🔍 No structures found matching '{}'", query);
            return Ok(());
        }

        println!("🔍 Search Results for '{}' ({} found):", query, structures.len());
        println!("{}", "─".repeat(80));
        
        for structure in structures {
            println!("🔹 {} ({})", structure.name, structure.structure_type);
            println!("  Terms: {}", structure.terms.join(" → "));
            
            // Show connectives if they exist
            display_connectives(&structure.connectives, &structure.terms);
            
            if let Some(desc) = &structure.description {
                println!("  Description: {}", desc);
            }
            println!("  ─────────────────────────────────────────");
            println!("  ID: {} | Created: {}", get_structure_id_string(&structure.id), structure.created_at);
            println!();
        }
        
        Ok(())
    }

    async fn view_structure(&self, id: &str) -> Result<(), SystematicsError> {
        let structure = self.api_client.get_structure(id).await?;
        
        match structure {
            Some(s) => {
                println!("📋 Structure Details");
                println!("{}", "═".repeat(60));
                println!("Name: {}", s.name);
                println!("Type: {}", s.structure_type);
                println!("ID: {}", get_structure_id_string(&s.id));
                println!("Created: {}", s.created_at);
                println!("Updated: {}", s.updated_at);
                
                if let Some(desc) = &s.description {
                    println!("Description: {}", desc);
                }
                
                println!("\nTerms ({}):", s.terms.len());
                for (i, term) in s.terms.iter().enumerate() {
                    println!("  {}: {}", i + 1, term);
                }
                
                // Show connectives if they exist
                if !s.connectives.is_empty() {
                    println!("\nConnectives ({}):", s.connectives.len());
                    
                    // Collect and format all connectives for column alignment
                    let mut formatted_connectives = Vec::new();
                    let mut max_left_width = 0;
                    let mut max_middle_width = 0;
                    let mut max_right_width = 0;
                    
                    for (key, relationship) in &s.connectives {
                        if let Some((from_str, to_str)) = key.split_once(':') {
                            if let (Ok(from), Ok(to)) = (from_str.parse::<usize>(), to_str.parse::<usize>()) {
                                let from_term = get_term_name(&s.terms, from);
                                let to_term = get_term_name(&s.terms, to);
                                
                                max_left_width = max_left_width.max(from_term.len());
                                max_middle_width = max_middle_width.max(relationship.len());
                                max_right_width = max_right_width.max(to_term.len());
                                
                                formatted_connectives.push((from_term, relationship, to_term));
                            }
                        }
                    }
                    
                    // Display with proper column alignment
                    for (from_term, relationship, to_term) in formatted_connectives {
                        println!("  {:^width_left$} <---({:^width_middle$})---> {:^width_right$}", 
                            from_term, relationship, to_term,
                            width_left = max_left_width,
                            width_middle = max_middle_width,
                            width_right = max_right_width
                        );
                    }
                }
                
                if !s.metadata.is_empty() {
                    println!("\nMetadata:");
                    for (key, value) in &s.metadata {
                        println!("  {}: {}", key, value);
                    }
                }
            }
            None => {
                println!("❌ Structure with ID '{}' not found", id);
            }
        }
        // Graph functionality not yet implemented in API
        Ok(())
    }

    async fn delete_structure(&self, id: &str) -> Result<(), SystematicsError> {
        let deleted = self.api_client.delete_structure(id).await?;
        
        if deleted {
            println!("✅ Structure '{}' deleted successfully", id);
        } else {
            println!("❌ Structure '{}' not found", id);
        }
        
        Ok(())
    }

    async fn show_related_structures(&self, id: &str) -> Result<(), SystematicsError> {
        let related = self.api_client.get_related_structures(id).await?;
        
        if related.is_empty() {
            println!("🔗 No related structures found for '{}'", id);
            return Ok(());
        }

        println!("🔗 Related Structures for '{}' ({} found):", id, related.len());
        println!("{}", "─".repeat(80));
        
        for structure in related {
            println!("🔹 {} ({})", structure.name, structure.structure_type);
            println!("  Terms: {}", structure.terms.join(" → "));
            
            // Show connectives if they exist
            display_connectives(&structure.connectives, &structure.terms);
            
            println!("  ─────────────────────────────────────────");
            println!("  ID: {}", get_structure_id_string(&structure.id));
            println!();
        }
        
        Ok(())
    }

    async fn find_term_usage(&self, term: &str) -> Result<(), SystematicsError> {
        // For now, use search functionality to find term usage
        let structures = self.api_client.search_structures(term).await?;
        
        if structures.is_empty() {
            println!("🔍 No structures found containing term '{}'", term);
            return Ok(());
        }

        println!("🔍 Structures containing '{}' ({} found):", term, structures.len());
        println!("{}", "─".repeat(80));
        
        for structure in structures {
            println!("🔹 {} ({})", structure.name, structure.structure_type);
            println!("  Terms: {}", structure.terms.join(" → "));
            
            // Show connectives if they exist
            display_connectives(&structure.connectives, &structure.terms);
            
            // Highlight the matching term
            let positions: Vec<usize> = structure.terms
                .iter()
                .enumerate()
                .filter(|(_, t)| t.contains(term))
                .map(|(i, _)| i + 1)
                .collect();
            
            if !positions.is_empty() {
                println!("  Found at positions: {}", 
                    positions.iter()
                        .map(|p| p.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                );
            }
            println!("  ─────────────────────────────────────────");
            println!("  ID: {}", get_structure_id_string(&structure.id));
            println!();
        }
        
        Ok(())
    }

    async fn show_structure_graph(&self, id: &str) -> Result<(), SystematicsError> {
        // Graph functionality not yet implemented in API
        println!("⚠️  Graph functionality not yet available via API");
        println!("   This feature will be added in a future update");
        println!("   Structure ID: {}", id);
        Ok(())
    }

    async fn update_metadata(&self, id: &str, metadata: HashMap<String, String>) -> Result<(), SystematicsError> {
        // Metadata update functionality not yet implemented in API
        println!("⚠️  Metadata update functionality not yet available via API");
        println!("   This feature will be added in a future update");
        println!("   Structure ID: {}", id);
        for (key, value) in metadata {
            println!("   Requested: {} = {}", key, value);
        }
        Ok(())
    }

    async fn init_database(&self) -> Result<(), SystematicsError> {
        println!("🚀 API-based database access initialized!");
        println!("   API Server: Connected");
        println!("   Database: Managed by API server");
        Ok(())
    }

    async fn show_stats(&self) -> Result<(), SystematicsError> {
        let structures = self.api_client.list_structures().await?;
        
        println!("📊 Database Statistics");
        println!("{}", "═".repeat(40));
        println!("Total structures: {}", structures.len());
        
        // Count by type
        let mut type_counts: HashMap<String, usize> = HashMap::new();
        let mut total_terms = 0;
        
        for structure in &structures {
            *type_counts.entry(structure.structure_type.clone()).or_insert(0) += 1;
            total_terms += structure.terms.len();
        }
        
        println!("Total terms: {}", total_terms);
        println!("\nBy structure type:");
        for (structure_type, count) in type_counts {
            println!("  {}: {}", structure_type, count);
        }
        
        if !structures.is_empty() {
            let oldest = structures.iter().min_by_key(|s| &s.created_at).unwrap();
            let newest = structures.iter().max_by_key(|s| &s.created_at).unwrap();
            
            println!("\nOldest: {} ({})", oldest.name, oldest.created_at);
            println!("Newest: {} ({})", newest.name, newest.created_at);
        }
        
        Ok(())
    }

    async fn export_database(&self, output_path: &str) -> Result<(), SystematicsError> {
        let structures = self.api_client.list_structures().await?;
        
        // Create export data structure
        let export_data = serde_json::json!({
            "export_info": {
                "timestamp": chrono::Utc::now().to_rfc3339(),
                "version": "1.0",
                "total_structures": structures.len()
            },
            "structures": structures
        });
        
        // Write to file
        let json_string = serde_json::to_string_pretty(&export_data)
            .map_err(|e| SystematicsError::Serialization(e.to_string()))?;
        std::fs::write(output_path, json_string)?;
        
        println!("📤 Database exported successfully!");
        println!("   File: {}", output_path);
        println!("   Structures: {}", structures.len());
        println!("   Size: {} bytes", std::fs::metadata(output_path)?.len());
        
        Ok(())
    }

    pub async fn auto_save_structure<T: SystematicStructure>(
        &self,
        structure: &T,
        name: &str,
        description: Option<&str>,
    ) -> Result<String, SystematicsError> {
        // Convert connectives from (usize, usize) keys to string keys for API
        let connectives: HashMap<String, String> = structure.connectives_traits()
            .iter()
            .map(|((from, to), relationship)| {
                (format!("{}:{}", from, to), relationship.clone())
            })
            .collect();
        
        let id = self.api_client.create_structure(
            name,
            structure.structure_type(),
            structure.terms().to_vec(),
            connectives,
            description.map(|s| s.to_string()),
        ).await?;
        
        println!("💾 Structure '{}' saved with ID: {}", name, id);
        Ok(id)
    }
}

fn parse_key_val(s: &str) -> Result<(String, String), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{s}`"))?;
    Ok((s[..pos].to_string(), s[pos + 1..].to_string()))
}

fn get_term_name(terms: &[String], index: usize) -> String {
    terms.get(index)
        .map(|s| s.clone())
        .unwrap_or_else(|| format!("Term{}", index))
}

fn get_structure_id_string(id: &crate::api_client::StructureId) -> &str {
    match &id.id {
        StructureIdValue::String(s) => s,
    }
}

fn display_connectives(connectives: &HashMap<String, String>, terms: &[String]) {
    if !connectives.is_empty() {
        println!("  Connectives:");
        
        // Collect and format all connectives for column alignment
        let mut formatted_connectives = Vec::new();
        let mut max_left_width = 0;
        let mut max_middle_width = 0;
        let mut max_right_width = 0;
        
        for (key, relationship) in connectives {
            if let Some((from_str, to_str)) = key.split_once(':') {
                if let (Ok(from), Ok(to)) = (from_str.parse::<usize>(), to_str.parse::<usize>()) {
                    let from_term = get_term_name(terms, from);
                    let to_term = get_term_name(terms, to);
                    
                    max_left_width = max_left_width.max(from_term.len());
                    max_middle_width = max_middle_width.max(relationship.len());
                    max_right_width = max_right_width.max(to_term.len());
                    
                    formatted_connectives.push((from_term, relationship, to_term));
                }
            }
        }
        
        // Display with proper column alignment
        for (from_term, relationship, to_term) in formatted_connectives {
            println!("    {:^width_left$} <---[{:^width_middle$}]---> {:^width_right$}", 
                from_term, relationship, to_term,
                width_left = max_left_width,
                width_middle = max_middle_width,
                width_right = max_right_width
            );
        }
    }
} 