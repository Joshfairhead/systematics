use std::io::{self, Write};
use std::collections::HashMap;
use crate::schemas::{StructureSchema, select_dodecad_schema};

pub struct DodecadicStructure {
    pub name: String,
    // Positional instances (A through L)
    pub positions: [String; 12],
    // Connectives stored as HashMap for scalability
    pub connectives: HashMap<(usize, usize), String>,
    // Current schema (optional, can be applied later)
    pub schema: Option<Box<dyn StructureSchema>>,
}

impl DodecadicStructure {
    /// Creates a new DodecadicStructure with empty positions
    pub fn new(name: &str) -> Self {
        DodecadicStructure {
            name: name.to_string(),
            positions: [
                String::new(), String::new(), String::new(), String::new(),
                String::new(), String::new(), String::new(), String::new(),
                String::new(), String::new(), String::new(), String::new(),
            ],
            connectives: HashMap::new(),
            schema: None,
        }
    }

    /// Creates a new DodecadicStructure with specific position values
    #[allow(clippy::too_many_arguments)]
    pub fn new_with_positions(
        name: &str,
        pos_a: &str, pos_b: &str, pos_c: &str, pos_d: &str,
        pos_e: &str, pos_f: &str, pos_g: &str, pos_h: &str,
        pos_i: &str, pos_j: &str, pos_k: &str, pos_l: &str,
    ) -> Self {
        let mut structure = DodecadicStructure {
            name: name.to_string(),
            positions: [
                pos_a.to_string(), pos_b.to_string(), pos_c.to_string(), pos_d.to_string(),
                pos_e.to_string(), pos_f.to_string(), pos_g.to_string(), pos_h.to_string(),
                pos_i.to_string(), pos_j.to_string(), pos_k.to_string(), pos_l.to_string(),
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
        if schema.get_position_count() != 12 {
            panic!("Schema must support exactly 12 positions for DodecadicStructure");
        }
        self.schema = Some(schema);
        self.refresh_connectives_with_schema();
    }

    /// Initialize default connectives
    fn initialize_default_connectives(&mut self) {
        for i in 0..12 {
            for j in (i + 1)..12 {
                let connective_name = if let Some(ref schema) = self.schema {
                    schema.get_connective_label(i, j)
                        .unwrap_or(&format!("{}{}", 
                            char::from(b'A' + i as u8), 
                            char::from(b'A' + j as u8)))
                        .to_string()
                } else {
                    format!("{}{}", 
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
        println!("\n--- Creating a Dodecad ---");
        
        // Schema selection first
        let schema = select_dodecad_schema();
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
        let mut positions = [
            String::new(), String::new(), String::new(), String::new(),
            String::new(), String::new(), String::new(), String::new(),
            String::new(), String::new(), String::new(), String::new(),
        ];
        let labels = schema.get_canonical_labels();
        
        for (i, &label) in labels.iter().enumerate() {
            let prompt = format!("{}: ", label);
            positions[i] = get_optional_input(&prompt, label)?;
        }

        // Create structure with schema
        let mut structure = DodecadicStructure {
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
            println!("Note: Dodecad has 66 connectives - this will take several moments to review.");
            
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
            .unwrap_or("Twelvefold structure");

        println!("\n--- Dodecad Details ---");
        println!("Dodecad Name: {}", self.name);
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
        println!("----------------------");
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
            vec!["A".to_string(), "B".to_string(), "C".to_string(), "D".to_string(), 
                 "E".to_string(), "F".to_string(), "G".to_string(), "H".to_string(),
                 "I".to_string(), "J".to_string(), "K".to_string(), "L".to_string()]
        }
    }

    /// Get all user instances
    pub fn get_instances(&self) -> Vec<String> {
        self.positions.iter().cloned().collect()
    }
}

impl std::fmt::Debug for DodecadicStructure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let schema_name = self.schema.as_ref()
            .map(|s| s.get_schema_name())
            .unwrap_or("No Schema");
        
        f.debug_struct("DodecadicStructure")
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
    use crate::schemas::BennettDodecadSchema;

    #[test]
    fn test_dodecadic_structure_creation() {
        let structure = DodecadicStructure::new_with_positions(
            "Test Dodecad",
            "Self-Rule", "Control", "Innovation", "Design", "Uniqueness", "Framework",
            "Recurrence", "Possibility", "Existence", "Connection", "Opposition", "Completeness"
        );
        
        assert_eq!(structure.name, "Test Dodecad");
        assert_eq!(structure.positions[0], "Self-Rule");
        assert_eq!(structure.positions[11], "Completeness");
        
        // Should have default connectives (12 choose 2 = 66)
        assert!(structure.has_connectives());
        assert_eq!(structure.connectives_count(), 66);
    }

    #[test]
    fn test_schema_application() {
        let mut structure = DodecadicStructure::new("Test");
        let schema = Box::new(BennettDodecadSchema);
        
        structure.apply_schema(schema);
        assert!(structure.schema.is_some());
        assert_eq!(structure.schema.as_ref().unwrap().get_schema_name(), "Bennett's Dodecad");
    }

    #[test]
    fn test_connective_operations() {
        let mut structure = DodecadicStructure::new_with_positions(
            "Test", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L"
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
        let mut structure = DodecadicStructure::new("Test");
        let schema = Box::new(BennettDodecadSchema);
        structure.apply_schema(schema);
        structure.initialize_default_connectives();
        
        // Should use schema-defined connective labels
        assert_eq!(structure.get_connective(0, 1).unwrap(), "Autocracy <> Domination");
        assert_eq!(structure.get_connective(2, 3).unwrap(), "Creativity <> Pattern");
    }

    #[test]
    fn test_jgb_schema() {
        let mut structure = DodecadicStructure::new("JGB Test");
        structure.apply_schema(Box::new(BennettDodecadSchema));
        structure.initialize_default_connectives();
        
        // Should use Bennett's schema connective labels
        assert_eq!(structure.get_connective(0, 1).unwrap(), "Autocracy <> Domination");
        assert_eq!(structure.get_connective(10, 11).unwrap(), "Polarity <> Wholeness");
    }
} 