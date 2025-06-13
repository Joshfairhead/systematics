use std::io::{self, Write};
use std::collections::HashMap;
use crate::schemas::{StructureSchema, select_dyad_schema};

pub struct DyadicStructure {
    pub name: String,
    pub positions: [String; 2],
    pub connectives: HashMap<(usize, usize), String>,
    pub schema: Option<Box<dyn StructureSchema>>,
}

impl DyadicStructure {
    /// Creates a new DyadicStructure with empty positions
    pub fn new(name: &str) -> Self {
        DyadicStructure {
            name: name.to_string(),
            positions: [String::new(), String::new()],
            connectives: HashMap::new(),
            schema: None,
        }
    }

    /// Creates a new DyadicStructure with specific position values
    pub fn new_with_positions(name: &str, pos_0: &str, pos_1: &str) -> Self {
        let mut structure = DyadicStructure {
            name: name.to_string(),
            positions: [pos_0.to_string(), pos_1.to_string()],
            connectives: HashMap::new(),
            schema: None,
        };
        structure.initialize_default_connectives();
        structure
    }

    /// Apply a schema to this structure
    pub fn apply_schema(&mut self, schema: Box<dyn StructureSchema>) {
        if schema.get_position_count() != 2 {
            panic!("Schema must support exactly 2 positions for DyadicStructure");
        }
        self.schema = Some(schema);
    }

    /// Initialize default connectives (none for dyad)
    fn initialize_default_connectives(&mut self) {
        // Dyad has no connectives to initialize
    }

    /// Interactive creation method with schema selection
    pub fn create_interactive() -> Result<Self, Box<dyn std::error::Error>> {
        println!("\n--- Creating a Dyad ---");
        let schema = select_dyad_schema();
        println!("Selected schema: {}", schema.get_schema_name());
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
        let name = get_optional_input("Enter a name for your Dyad (or press Enter for 'Unnamed Dyad'): ", "Unnamed Dyad")?;
        let labels = schema.get_canonical_labels();
        let mut positions = [String::new(), String::new()];
        for i in 0..2 {
            let prompt = format!("{}: ", labels[i]);
            positions[i] = get_optional_input(&prompt, labels[i])?;
        }
        let structure = DyadicStructure {
            name,
            positions,
            connectives: HashMap::new(),
            schema: Some(schema),
        };
        structure.display();
        Ok(structure)
    }

    /// Display structure details
    pub fn display(&self) {
        let schema_name = self.schema.as_ref().map(|s| s.get_schema_name()).unwrap_or("No Schema");
        let attribute = self.schema.as_ref().map(|s| s.get_attribute_description()).unwrap_or("Complementarity, polarity or force");
        println!("\n--- Dyad Details ---");
        println!("Dyad Name: {}", self.name);
        println!("Schema: {}", schema_name);
        println!("Core Attribute: {}", attribute);
        if let Some(ref schema) = self.schema {
            let labels = schema.get_canonical_labels();
            for i in 0..2 {
                println!("{}: {}", labels[i], self.positions[i]);
            }
        } else {
            println!("A: {}", self.positions[0]);
            println!("B: {}", self.positions[1]);
        }
        println!("----------------------");
    }

    /// Get canonical term names from schema or positional defaults
    pub fn get_canonical_terms(&self) -> Vec<String> {
        if let Some(ref schema) = self.schema {
            schema.get_canonical_labels().iter().map(|&s| s.to_string()).collect()
        } else {
            vec!["A".to_string(), "B".to_string()]
        }
    }

    /// Get all user instances
    pub fn get_instances(&self) -> Vec<String> {
        self.positions.to_vec()
    }
}

impl std::fmt::Debug for DyadicStructure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let schema_name = self.schema.as_ref().map(|s| s.get_schema_name()).unwrap_or("No Schema");
        f.debug_struct("DyadicStructure")
            .field("name", &self.name)
            .field("positions", &self.positions)
            .field("connectives", &self.connectives)
            .field("schema", &schema_name)
            .finish()
    }
} 