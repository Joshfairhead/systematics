use std::io::{self, Write};
use std::collections::HashMap;
use crate::schemas::{StructureSchema, select_triad_schema};

pub struct TriadicStructure {
    pub name: String,
    // Three position instances (1, 2, 3)
    pub positions: [String; 3],
    // Additional user attributes for the triad
    pub attributes: Vec<String>,
    // Connectives stored as HashMap for scalability
    pub connectives: HashMap<(usize, usize), String>,
    // Current schema (optional, can be applied later)
    pub schema: Option<Box<dyn StructureSchema>>,
}

impl TriadicStructure {
    /// Creates a new TriadicStructure with empty positions
    pub fn new(name: &str) -> Self {
        TriadicStructure {
            name: name.to_string(),
            positions: [String::new(), String::new(), String::new()],
            attributes: Vec::new(),
            connectives: HashMap::new(),
            schema: None,
        }
    }

    /// Creates a new TriadicStructure with specific position values
    pub fn new_with_positions(name: &str, pos_0: &str, pos_1: &str, pos_2: &str) -> Self {
        let mut structure = TriadicStructure {
            name: name.to_string(),
            positions: [pos_0.to_string(), pos_1.to_string(), pos_2.to_string()],
            attributes: Vec::new(),
            connectives: HashMap::new(),
            schema: None,
        };
        structure.initialize_default_connectives();
        structure
    }

    /// Apply a schema to this structure
    pub fn apply_schema(&mut self, schema: Box<dyn StructureSchema>) {
        if schema.get_position_count() != 3 {
            panic!("Schema must support exactly 3 positions for TriadicStructure");
        }
        self.schema = Some(schema);
    }

    /// Initialize default connectives (none for triad)
    fn initialize_default_connectives(&mut self) {
        // Triad has no connectives to initialize
    }

    /// Interactive creation method with schema selection
    pub fn create_interactive() -> Result<Self, Box<dyn std::error::Error>> {
        println!("\n--- Creating a Triad ---");
        let schema = select_triad_schema();
        println!("Schema: {}", schema.get_schema_name());
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
        let name = get_optional_input("Enter a name for your Triad (or press Enter for 'Unnamed Triad'): ", "Unnamed Triad")?;
        let labels = schema.get_canonical_labels();
        let mut positions = [String::new(), String::new(), String::new()];
        for i in 0..3 {
            let prompt = format!("{}: ", labels[i]);
            positions[i] = get_optional_input(&prompt, labels[i])?;
        }
        let structure = TriadicStructure {
            name,
            positions,
            attributes: Vec::new(),
            connectives: HashMap::new(),
            schema: Some(schema),
        };
        structure.display();
        Ok(structure)
    }

    /// Display structure details
    pub fn display(&self) {
        println!("\n--- Triad Details ---");
        println!("Name: {}", self.name);
        if let Some(ref schema) = self.schema {
            let labels = schema.get_canonical_labels();
            for i in 0..3 {
                println!("{} ({}): {}", labels[i], i + 1, self.positions[i]);
            }
        } else {
            println!("Position 1: {}", self.positions[0]);
            println!("Position 2: {}", self.positions[1]);
            println!("Position 3: {}", self.positions[2]);
        }
        
        // Display attributes if any
        if !self.attributes.is_empty() {
            print!("Attributes: ");
            for (i, attribute) in self.attributes.iter().enumerate() {
                if i > 0 {
                    print!(", ");
                }
                print!("{}", attribute);
            }
            println!();
        }
        
        println!("----------------------");
    }

    /// Get canonical term names from schema or positional defaults
    pub fn get_canonical_terms(&self) -> Vec<String> {
        if let Some(ref schema) = self.schema {
            schema.get_canonical_labels().iter().map(|&s| s.to_string()).collect()
        } else {
            vec!["1".to_string(), "2".to_string(), "3".to_string()]  // Base layer uses 1-based numbers
        }
    }

    /// Get all user instances
    pub fn get_instances(&self) -> Vec<String> {
        self.positions.to_vec()
    }
}

impl std::fmt::Debug for TriadicStructure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let schema_name = self.schema.as_ref().map(|s| s.get_schema_name()).unwrap_or("No Schema");
        f.debug_struct("TriadicStructure")
            .field("name", &self.name)
            .field("positions", &self.positions)
            .field("connectives", &self.connectives)
            .field("schema", &schema_name)
            .finish()
    }
} 