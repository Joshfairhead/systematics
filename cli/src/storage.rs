use systematics_api::{
    SurrealStorage, StoredStructure, SystematicsError, SystematicStructure
};
use std::collections::HashMap;
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
}

pub struct StorageCli {
    storage: SurrealStorage,
}

impl StorageCli {
    pub async fn new() -> Result<Self, SystematicsError> {
        let db_path = "./systematics.db";
        let storage = SurrealStorage::new(db_path).await?;
        
        println!("📚 Connected to SurrealDB at: {}", db_path);
        
        Ok(Self { storage })
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
        }
    }

    async fn list_structures(&self) -> Result<(), SystematicsError> {
        let structures = self.storage.list_structures().await?;
        
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
            println!("  ID: {} | Created: {}", structure.id.id, structure.created_at);
            println!();
        }
        
        Ok(())
    }

    async fn search_structures(&self, query: &str) -> Result<(), SystematicsError> {
        let structures = self.storage.search_structures(query).await?;
        
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
            println!("  ID: {} | Created: {}", structure.id.id, structure.created_at);
            println!();
        }
        
        Ok(())
    }

    async fn view_structure(&self, id: &str) -> Result<(), SystematicsError> {
        let structure = self.storage.get_structure(id).await?;
        
        match structure {
            Some(s) => {
                println!("📋 Structure Details");
                println!("{}", "═".repeat(60));
                println!("Name: {}", s.name);
                println!("Type: {}", s.structure_type);
                println!("ID: {}", s.id.id);
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
        
        Ok(())
    }

    async fn delete_structure(&self, id: &str) -> Result<(), SystematicsError> {
        let deleted = self.storage.delete_structure(id).await?;
        
        if deleted {
            println!("✅ Structure '{}' deleted successfully", id);
        } else {
            println!("❌ Structure '{}' not found", id);
        }
        
        Ok(())
    }

    async fn show_related_structures(&self, id: &str) -> Result<(), SystematicsError> {
        let related = self.storage.get_related_structures(id).await?;
        
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
            println!("  ID: {}", structure.id.id);
            println!();
        }
        
        Ok(())
    }

    async fn find_term_usage(&self, term: &str) -> Result<(), SystematicsError> {
        let structures = self.storage.get_term_usage(term).await?;
        
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
            println!("  ID: {}", structure.id.id);
            println!();
        }
        
        Ok(())
    }

    async fn show_structure_graph(&self, id: &str) -> Result<(), SystematicsError> {
        let (nodes, edges) = self.storage.get_structure_graph(id).await?;
        
        if nodes.is_empty() {
            println!("❌ Structure '{}' not found or has no graph data", id);
            return Ok(());
        }

        println!("🕸️  Graph for Structure '{}':", id);
        println!("{}", "═".repeat(60));
        
        println!("Nodes ({}):", nodes.len());
        for node in &nodes {
            println!("  {} [{}]: {}", node.position + 1, node.id.id, node.term);
        }
        
        println!("\nEdges ({}):", edges.len());
        for edge in &edges {
            let from_pos = nodes.iter().find(|n| n.id.id.to_string() == edge.from_node)
                .map(|n| n.position + 1)
                .unwrap_or(0);
            let to_pos = nodes.iter().find(|n| n.id.id.to_string() == edge.to_node)
                .map(|n| n.position + 1)
                .unwrap_or(0);
                
            println!("  {} → {} ({}, weight: {:.2})", 
                from_pos, to_pos, edge.relationship_type, edge.weight);
        }
        
        Ok(())
    }

    async fn update_metadata(&self, id: &str, metadata: HashMap<String, String>) -> Result<(), SystematicsError> {
        let updated = self.storage.update_structure_metadata(id, metadata.clone()).await?;
        
        if updated {
            println!("✅ Metadata updated for structure '{}'", id);
            for (key, value) in metadata {
                println!("  {}: {}", key, value);
            }
        } else {
            println!("❌ Structure '{}' not found", id);
        }
        
        Ok(())
    }

    async fn init_database(&self) -> Result<(), SystematicsError> {
        println!("🚀 Database initialized successfully!");
        println!("   Location: ./systematics.db");
        println!("   Database: systematics/structures");
        Ok(())
    }

    async fn show_stats(&self) -> Result<(), SystematicsError> {
        let structures = self.storage.list_structures().await?;
        
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

    pub async fn auto_save_structure<T: SystematicStructure>(
        &self,
        structure: &T,
        name: &str,
        description: Option<&str>,
    ) -> Result<String, SystematicsError> {
        let id = self.storage.store_structure(structure, name, description).await?;
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