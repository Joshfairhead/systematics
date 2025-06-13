use std::io::{self, Write};
use std::collections::HashMap;
use crate::schemas::{StructureSchema, select_heptad_schema};

pub struct HeptadicStructure {
    pub name: String,
    // Positional instances (A, B, C, D, E, F, G)
    pub positions: [String; 7],
    // Connectives stored as HashMap for scalability
    pub connectives: HashMap<(usize, usize), String>,
    // Current schema (optional, can be applied later)
    pub schema: Option<Box<dyn StructureSchema>>,
}

impl HeptadicStructure {
    /// Creates a new HeptadicStructure with empty positions
    pub fn new(name: &str) -> Self {
        HeptadicStructure {
            name: name.to_string(),
            positions: [
                String::new(),
                String::new(), 
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
            ],
            connectives: HashMap::new(),
            schema: None,
        }
    }

    /// Creates a new HeptadicStructure with specific position values
    pub fn new_with_positions(
        name: &str,
        pos_a: &str,
        pos_b: &str,
        pos_c: &str,
        pos_d: &str,
        pos_e: &str,
        pos_f: &str,
        pos_g: &str,
    ) -> Self {
        let mut structure = HeptadicStructure {
            name: name.to_string(),
            positions: [
                pos_a.to_string(),
                pos_b.to_string(),
                pos_c.to_string(),
                pos_d.to_string(),
                pos_e.to_string(),
                pos_f.to_string(),
                pos_g.to_string(),
            ],
            connectives: HashMap::new(),
            schema: None,
        };

        // Initialize default connectives
        structure.initialize_default_connectives();
        structure
    }

    /// Apply a schema to this structure
    pub fn apply_schema(&mut self, schema: Box<dyn StructureSchema>) {
        if schema.get_position_count() != 7 {
            panic!("Schema must support exactly 7 positions for HeptadicStructure");
        }
        self.schema = Some(schema);
        self.refresh_connectives_with_schema();
    }

    /// Initialize default connectives
    fn initialize_default_connectives(&mut self) {
        for i in 0..7 {
            for j in (i + 1)..7 {
                let connective_name = if let Some(ref schema) = self.schema {
                    schema.get_connective_label(i, j)
                        .unwrap_or(&format!("{}<>{}", 
                            char::from(b'A' + i as u8), 
                            char::from(b'A' + j as u8)))
                        .to_string()
                } else {
                    format!("{}<>{}", 
                        char::from(b'A' + i as u8), 
                        char::from(b'A' + j as u8))
                };
                self.connectives.insert((i, j), connective_name);
            }
        }
    }
    
    /// Refresh connectives with current schema
    pub fn refresh_connectives_with_schema(&mut self) {
        if self.schema.is_some() {
            self.initialize_default_connectives();
        }
    }

    /// Get a connective by position indices
    pub fn get_connective(&self, i: usize, j: usize) -> Option<&String> {
        let key = if i < j { (i, j) } else { (j, i) };
        self.connectives.get(&key)
    }

    /// Set a connective by position indices
    pub fn set_connective(&mut self, i: usize, j: usize, value: String) {
        let key = if i < j { (i, j) } else { (j, i) };
        self.connectives.insert(key, value);
    }

    /// Get the total number of connectives
    pub fn connectives_count(&self) -> usize {
        self.connectives.len()
    }

    /// Check if any connectives are defined
    pub fn has_connectives(&self) -> bool {
        !self.connectives.is_empty()
    }

    /// Interactive creation method with schema selection
    pub fn create_interactive() -> Result<Self, Box<dyn std::error::Error>> {
        println!("\n--- Creating a Heptad ---");
        
        // Schema selection first
        let schema = select_heptad_schema();
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

        // Helper for yes/no input
        let get_yes_no_input = |prompt: &str, default: &str| -> Result<String, Box<dyn std::error::Error>> {
            loop {
                let mut input = String::new();
                print!("{}", prompt);
                
                if let Err(e) = io::stdout().flush() {
                    eprintln!("Warning: Could not flush output: {}", e);
                }
                
                match io::stdin().read_line(&mut input) {
                    Ok(_) => {
                        let trimmed = input.trim().to_lowercase();
                        
                        if trimmed.is_empty() {
                            return Ok(default.to_string());
                        }
                        
                        match trimmed.as_str() {
                            "y" | "yes" | "n" | "no" => return Ok(trimmed),
                            _ => {
                                println!("Please enter 'y' or 'n' (or press Enter for default: {})", default);
                                continue;
                            }
                        }
                    }
                    Err(e) => return Err(format!("Error reading input: {}", e).into())
                }
            }
        };

        // Get structure name
        let name = get_optional_input(
            &format!("Enter a name for your {}: ", 
                schema.get_structure_name().to_lowercase()), 
            &format!("Unnamed {}", schema.get_structure_name())
        )?;

        // Get position instances using schema labels
        let mut positions = [String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new()];
        let labels = schema.get_canonical_labels();
        
        for (i, &label) in labels.iter().enumerate() {
            let prompt = format!("{}: ", label);
            positions[i] = get_optional_input(&prompt, label)?;
        }

        // Create structure with schema
        let mut structure = HeptadicStructure {
            name,
            positions,
            connectives: HashMap::new(),
            schema: Some(schema),
        };

        // Initialize connectives with schema-aware defaults
        structure.initialize_default_connectives();

        // Ask if user wants to modify the default connectives
        let modify_connectives = get_yes_no_input("\nWould you like to modify the default connectives? (y/n): ", "y")?;
        
        if modify_connectives.starts_with('y') {
            println!("\nModifying connectives (press Enter to keep default, or input new value):");
            println!("Note: Heptad has 21 connectives.");
            
            // Get all connective keys and sort them for consistent ordering
            let mut keys: Vec<_> = structure.connectives.keys().cloned().collect();
            keys.sort();
            
            for (i, j) in keys {
                let current_value = structure.connectives.get(&(i, j)).unwrap();
                let prompt = format!("{} (or press enter): ", current_value);
                
                let new_value = get_optional_input(&prompt, current_value)?;
                structure.set_connective(i, j, new_value);
            }
        }
        
        // Display the created structure
        structure.display();
        
        // Show connectives if any were defined
        if structure.has_connectives() {
            structure.display_connectives();
        }
        
        Ok(structure)
    }

    /// Display structure details
    pub fn display(&self) {
        let schema_name = self.schema.as_ref()
            .map(|s| s.get_schema_name())
            .unwrap_or("No Schema");
        let attribute = self.schema.as_ref()
            .map(|s| s.get_attribute_description())
            .unwrap_or("Sevenfold structure");

        println!("\n--- Heptad Details ---");
        println!("Heptad Name: {}", self.name);
        println!("Schema: {}", schema_name);
        println!("Core Attribute: {}", attribute);
        
        if let Some(ref schema) = self.schema {
            let labels = schema.get_canonical_labels();
            for (i, &label) in labels.iter().enumerate() {
                println!("{}: {}", label, self.positions[i]);
            }
        } else {
            for (i, position) in self.positions.iter().enumerate() {
                let pos_char = char::from(b'A' + i as u8);
                println!("{}: {}", pos_char, position);
            }
        }
        println!("---------------------");
    }

    /// Display all connectives
    pub fn display_connectives(&self) {
        println!("\nConnectives:");
        
        // Get all connective keys and sort them for consistent ordering
        let mut keys: Vec<_> = self.connectives.keys().cloned().collect();
        keys.sort();
        
        for (i, j) in keys {
            let connective = self.connectives.get(&(i, j)).unwrap();
            
            println!("  {} <--[{}]--> {}", 
                self.positions[i], connective, self.positions[j]);
        }
    }

    /// Get canonical term names from schema or positional defaults
    pub fn get_canonical_terms(&self) -> Vec<String> {
        if let Some(ref schema) = self.schema {
            schema.get_canonical_labels().iter().map(|&s| s.to_string()).collect()
        } else {
            vec!["A".to_string(), "B".to_string(), "C".to_string(), "D".to_string(), "E".to_string(), "F".to_string(), "G".to_string()]
        }
    }

    /// Get all user instances
    pub fn get_instances(&self) -> Vec<String> {
        self.positions.iter().cloned().collect()
    }
}

impl std::fmt::Debug for HeptadicStructure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let schema_name = self.schema.as_ref()
            .map(|s| s.get_schema_name())
            .unwrap_or("No Schema");
        
        f.debug_struct("HeptadicStructure")
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
    use crate::schemas::BennettHeptadSchema;

    #[test]
    fn test_heptadic_structure_creation() {
        let structure = HeptadicStructure::new_with_positions(
            "Test Heptad",
            "Core Insight",
            "Deep Research",
            "Creative Design",
            "Smart Synthesis",
            "Practical Application",
            "Effective Delivery",
            "Real Value"
        );
        
        assert_eq!(structure.name, "Test Heptad");
        assert_eq!(structure.positions[0], "Core Insight");
        assert_eq!(structure.positions[1], "Deep Research");
        assert_eq!(structure.positions[2], "Creative Design");
        assert_eq!(structure.positions[3], "Smart Synthesis");
        assert_eq!(structure.positions[4], "Practical Application");
        assert_eq!(structure.positions[5], "Effective Delivery");
        assert_eq!(structure.positions[6], "Real Value");
        
        // Should have default connectives (7 choose 2 = 21)
        assert!(structure.has_connectives());
        assert_eq!(structure.connectives_count(), 21);
    }

    #[test]
    fn test_schema_application() {
        let mut structure = HeptadicStructure::new("Test");
        let schema = Box::new(BennettHeptadSchema);
        
        structure.apply_schema(schema);
        assert!(structure.schema.is_some());
        assert_eq!(structure.schema.as_ref().unwrap().get_schema_name(), "JGB's Heptad");
    }

    #[test]
    fn test_connective_operations() {
        let mut structure = HeptadicStructure::new_with_positions(
            "Test", "A", "B", "C", "D", "E", "F", "G"
        );
        
        // Test get/set connective methods
        assert!(structure.get_connective(0, 1).is_some());
        
        structure.set_connective(0, 1, "custom connection".to_string());
        assert_eq!(structure.get_connective(0, 1).unwrap(), "custom connection");
        
        // Test bidirectional access (should work both ways)
        assert_eq!(structure.get_connective(1, 0).unwrap(), "custom connection");
    }

    #[test]
    fn test_schema_aware_connectives() {
        let mut structure = HeptadicStructure::new("Test");
        let schema = Box::new(BennettHeptadSchema);
        structure.apply_schema(schema);
        structure.initialize_default_connectives();
        
        // Should use schema-defined connective labels
        assert_eq!(structure.get_connective(0, 1).unwrap(), "Insight <> Research");
        assert_eq!(structure.get_connective(2, 3).unwrap(), "Design <> Synthesis");
        assert_eq!(structure.get_connective(5, 6).unwrap(), "Delivery <> Value");
    }

    #[test]
    fn test_jgb_schema() {
        let mut structure = HeptadicStructure::new("JGB Test");
        structure.apply_schema(Box::new(BennettHeptadSchema));
        structure.initialize_default_connectives();
        
        // Should use JGB's schema connective labels
        assert_eq!(structure.get_connective(0, 1).unwrap(), "Insight <> Research");
        assert_eq!(structure.get_connective(0, 2).unwrap(), "Insight <> Design");
        assert_eq!(structure.get_connective(1, 3).unwrap(), "Research <> Synthesis");
        assert_eq!(structure.get_connective(4, 5).unwrap(), "Application <> Delivery");
        assert_eq!(structure.get_connective(5, 6).unwrap(), "Delivery <> Value");
    }
} 