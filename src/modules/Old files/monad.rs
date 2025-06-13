use std::io::{self, Write};
use std::collections::HashMap;
use crate::schemas::{StructureSchema, select_monad_schema};

#[derive(Debug)]
pub struct MonadicStructure {
    pub name: String,
    // Single position instance (A)
    pub positions: [String; 1],
    // Connectives stored as HashMap for scalability (empty for monad)
    pub connectives: HashMap<(usize, usize), String>,
    // Current schema (optional, can be applied later)
    pub schema: Option<Box<dyn StructureSchema>>,
}

impl MonadicStructure {
    /// Creates a new MonadicStructure with empty position
    pub fn new(name: &str) -> Self {
        MonadicStructure {
            name: name.to_string(),
            positions: [String::new()],
            connectives: HashMap::new(),
            schema: None,
        }
    }

    /// Creates a new MonadicStructure with specific position value
    pub fn new_with_positions(name: &str, pos_a: &str) -> Self {
        let mut structure = MonadicStructure {
            name: name.to_string(),
            positions: [pos_a.to_string()],
            connectives: HashMap::new(),
            schema: None,
        };

        // Initialize default connectives (none for monad)
        structure.initialize_default_connectives();
        structure
    }

    /// Apply a schema to this structure
    pub fn apply_schema(&mut self, schema: Box<dyn StructureSchema>) {
        if schema.get_position_count() != 1 {
            panic!("Schema must support exactly 1 position for MonadicStructure");
        }
        self.schema = Some(schema);
    }

    /// Initialize default connectives (empty for monad)
    fn initialize_default_connectives(&mut self) {
        // Monad has no connectives to initialize
    }

    /// Interactive creation method with schema selection
    pub fn create_interactive() -> Result<Self, Box<dyn std::error::Error>> {
        println!("\n--- Creating a Monad ---");
        
        // Schema selection first
        let schema = select_monad_schema();
        println!("Selected schema: {}", schema.get_schema_name());
        
        // Helper for optional input with default
        let get_optional_input = |prompt: &str, default: &str| -> Result<String, Box<dyn std::error::Error>> {
            let mut input = String::new();
            print!("{}", prompt);
            
            if let Err(e) = io::stdout().flush() {
                eprintln!("Warning: Could not flush output: {}", e);
            }
            
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    let trimmed = input.trim();
                    
                    if trimmed.is_empty() {
                        Ok(default.to_string())
                    } else {
                        // Validate optional input if provided
                        if trimmed.len() > 100 {
                            return Err("Input is too long (max 100 characters)".into());
                        }
                        
                        if !trimmed.chars().all(|c| c.is_alphanumeric() || c.is_whitespace() || ".,!?'-()".contains(c)) {
                            return Err("Input contains invalid characters".into());
                        }
                        
                        Ok(trimmed.to_string())
                    }
                }
                Err(e) => {
                    eprintln!("Error reading input: {}. Using default: {}", e, default);
                    Ok(default.to_string())
                }
            }
        };

        // Get monad name and position
        let name = get_optional_input("Enter a name for your Monad (or press Enter for 'Unnamed Monad'): ", "Unnamed Monad")?;
        let labels = schema.get_canonical_labels();
        let mut positions = [String::new()];
        
        let prompt = format!("{}: ", labels[0]);
        positions[0] = get_optional_input(&prompt, labels[0])?;

        // Create structure with schema
        let mut structure = MonadicStructure {
            name,
            positions,
            connectives: HashMap::new(),
            schema: Some(schema),
        };

        // Display the created structure
        structure.display();
        
        Ok(structure)
    }

    /// Display structure details
    pub fn display(&self) {
        let schema_name = self.schema.as_ref()
            .map(|s| s.get_schema_name())
            .unwrap_or("No Schema");
        let attribute = self.schema.as_ref()
            .map(|s| s.get_attribute_description())
            .unwrap_or("Unity");

        println!("\n--- Monad Details ---");
        println!("Monad Name: {}", self.name);
        println!("Schema: {}", schema_name);
        println!("Core Attribute: {}", attribute);
        
        if let Some(ref schema) = self.schema {
            let labels = schema.get_canonical_labels();
            println!("{}: {}", labels[0], self.positions[0]);
        } else {
            println!("A: {}", self.positions[0]);
        }
        println!("----------------------");
    }

    /// Get canonical term names from schema or positional defaults
    pub fn get_canonical_terms(&self) -> Vec<String> {
        if let Some(ref schema) = self.schema {
            schema.get_canonical_labels().iter().map(|&s| s.to_string()).collect()
        } else {
            vec!["A".to_string()]
        }
    }

    /// Get all user instances
    pub fn get_instances(&self) -> Vec<String> {
        self.positions.to_vec()
    }
}

impl std::fmt::Debug for MonadicStructure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let schema_name = self.schema.as_ref()
            .map(|s| s.get_schema_name())
            .unwrap_or("No Schema");
        
        f.debug_struct("MonadicStructure")
            .field("name", &self.name)
            .field("positions", &self.positions)
            .field("connectives", &self.connectives)
            .field("schema", &schema_name)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schemas::monad::BennettMonadSchema;

    #[test]
    fn test_monadic_structure_creation() {
        let structure = MonadicStructure::new_with_positions(
            "Test Monad",
            "Unity Instance"
        );
        
        assert_eq!(structure.name, "Test Monad");
        assert_eq!(structure.positions[0], "Unity Instance");
        
        // Should have no connectives
        assert_eq!(structure.connectives.len(), 0);
    }

    #[test]
    fn test_schema_application() {
        let mut structure = MonadicStructure::new("Test");
        let schema = Box::new(BennettMonadSchema);
        
        structure.apply_schema(schema);
        assert!(structure.schema.is_some());
        assert_eq!(structure.schema.as_ref().unwrap().get_schema_name(), "JGB's Monad");
    }

    #[test]
    fn test_canonical_terms() {
        let mut structure = MonadicStructure::new("Test");
        let schema = Box::new(BennettMonadSchema);
        structure.apply_schema(schema);
        
        let terms = structure.get_canonical_terms();
        assert_eq!(terms, vec!["Unity"]);
    }

    #[test]
    fn test_instances() {
        let structure = MonadicStructure::new_with_positions(
            "Test",
            "Unity Instance"
        );
        
        let instances = structure.get_instances();
        assert_eq!(instances, vec!["Unity Instance"]);
    }
}