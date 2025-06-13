use std::io::{self, Write};
use std::collections::HashMap;
use crate::schemas::{StructureSchema, select_monad_schema};

pub struct MonadicStructure {
    pub name: String,
    // Single position instance (A)
    pub positions: [String; 1],
    // Additional user attributes for the monad
    pub attributes: Vec<String>, // List of attribute descriptions
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
            attributes: Vec::new(),
            connectives: HashMap::new(),
            schema: None,
        }
    }

    /// Apply a schema to this structure (builder pattern) - must be done first
    pub fn with_schema(mut self, schema: Box<dyn StructureSchema>) -> Self {
        if schema.get_position_count() != 1 {
            panic!("Schema must support exactly 1 position for MonadicStructure");
        }
        self.schema = Some(schema);
        self
    }

    /// Set content by semantic label (requires schema to be applied first)
    pub fn with_content(mut self, semantic_label: &str, content: &str) -> Self {
        if let Some(ref schema) = self.schema {
            let labels = schema.get_canonical_labels();
            if let Some(position) = labels.iter().position(|&label| label == semantic_label) {
                self.positions[position] = content.to_string();
                self.initialize_default_connectives();
            } else {
                panic!("Unknown semantic label '{}'. Available labels: {:?}", semantic_label, labels);
            }
        } else {
            panic!("Schema must be applied before adding content. Use .with_schema() first.");
        }
        self
    }

    /// Set content by numeric position (1-based) - for direct access
    pub fn with_position(mut self, position: usize, content: &str) -> Self {
        if position < 1 || position > self.positions.len() {
            panic!("Position {} out of bounds for monad (valid range: 1-{})", position, self.positions.len());
        }
        self.positions[position - 1] = content.to_string();
        self.initialize_default_connectives();
        self
    }

    /// Add an attribute (builder pattern)
    pub fn with_attribute(mut self, attribute: &str) -> Self {
        self.attributes.push(attribute.to_string());
        self
    }

    /// Creates a new MonadicStructure with specific position value (legacy method)
    #[deprecated(note = "Use new().with_schema().with_content() or new().with_position() instead")]
    pub fn new_with_positions(name: &str, pos_1: &str) -> Self {
        Self::new(name).with_position(1, pos_1)
    }

    /// Apply a schema to this structure (mutable method)
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

    /// Add an attribute to the monad
    pub fn add_attribute(&mut self, attribute: &str) {
        self.attributes.push(attribute.to_string());
    }

    /// Remove an attribute by value
    pub fn remove_attribute(&mut self, attribute: &str) {
        self.attributes.retain(|attr| attr != attribute);
    }

    /// Check if monad has a specific attribute
    pub fn has_attribute(&self, attribute: &str) -> bool {
        self.attributes.contains(&attribute.to_string())
    }

    /// Get all attributes
    pub fn get_attributes(&self) -> &Vec<String> {
        &self.attributes
    }

    /// Interactive creation method with schema selection
    pub fn create_interactive() -> Result<Self, Box<dyn std::error::Error>> {
        println!("\n--- Creating a Monad ---");
        
        // Schema selection and display first
        let schema = select_monad_schema();
        println!("Schema: {}", schema.get_schema_name());
        
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

        // Get monad name after schema display
        let name = get_optional_input("Enter a name for your Monad (or press Enter for 'Unnamed Monad'): ", "Unnamed Monad")?;
        let labels = schema.get_canonical_labels();
        let positions = [labels[0].to_string()]; // Use the canonical label directly

        // Create structure with schema
        let mut structure = MonadicStructure {
            name,
            positions,
            attributes: Vec::new(),
            connectives: HashMap::new(),
            schema: Some(schema),
        };

        // Add attributes (default to yes)
        println!("\nAdd attributes to your Monad (press Enter to start, or 'n' to skip): ");
        let add_attributes = get_optional_input("", "y")?;
        
        if !add_attributes.to_lowercase().starts_with('n') {
            println!("Enter attributes one by one (press Enter on empty line to finish):");
            loop {
                print!("Attribute: ");
                use std::io::{self, Write};
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let attribute = input.trim();
                
                if attribute.is_empty() {
                    break;
                }
                
                structure.add_attribute(attribute);
            }
            
            if !structure.attributes.is_empty() {
                println!("Added {} attributes", structure.attributes.len());
            }
        }

        // Display the created structure
        structure.display();
        
        Ok(structure)
    }

    /// Display structure details
    pub fn display(&self) {
        println!("\n--- Monad Details ---");
        println!("Name: {}", self.name);
        
        // Display position with three-layer architecture
        if let Some(ref schema) = self.schema {
            let labels = schema.get_canonical_labels();
            println!("{} (1): {}", labels[0], self.positions[0]);
        } else {
            println!("Position 1: {}", self.positions[0]);
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
            vec!["1".to_string()]  // Base layer uses 1-based numbers
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
            .field("attributes", &self.attributes)
            .field("connectives", &self.connectives)
            .field("schema", &schema_name)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schemas::schema1_monad::BennettMonadSchema;

    #[test]
    fn test_monadic_structure_creation() {
        let structure = MonadicStructure::new("Test Monad")
            .with_position(1, "Unity Instance");
        
        assert_eq!(structure.name, "Test Monad");
        assert_eq!(structure.positions[0], "Unity Instance");
        
        // Should have no connectives
        assert_eq!(structure.connectives.len(), 0);
    }

    #[test]
    fn test_schema_application() {
        // Test builder pattern
        let structure = MonadicStructure::new("Test")
            .with_schema(Box::new(BennettMonadSchema));
        
        assert!(structure.schema.is_some());
        assert_eq!(structure.schema.as_ref().unwrap().get_schema_name(), "Bennett's Monad");
        
        // Test mutable method still works
        let mut structure2 = MonadicStructure::new("Test2");
        structure2.apply_schema(Box::new(BennettMonadSchema));
        assert!(structure2.schema.is_some());
    }

    #[test]
    fn test_canonical_terms() {
        let structure = MonadicStructure::new("Test")
            .with_schema(Box::new(BennettMonadSchema));
        
        let terms = structure.get_canonical_terms();
        assert_eq!(terms, vec!["Unity"]);
    }

    #[test]
    fn test_instances() {
        let structure = MonadicStructure::new("Test")
            .with_position(1, "Unity Instance");
        
        let instances = structure.get_instances();
        assert_eq!(instances, vec!["Unity Instance"]);
    }

    #[test]
    fn test_attribute_management() {
        let mut structure = MonadicStructure::new("Test Monad");
        
        // Test adding attributes
        structure.add_attribute("infinite");
        structure.add_attribute("diverse");
        structure.add_attribute("eternal");
        
        assert!(structure.has_attribute("infinite"));
        assert!(structure.has_attribute("diverse"));
        assert!(structure.has_attribute("eternal"));
        assert!(!structure.has_attribute("nonexistent"));
        
        // Test attribute count
        assert_eq!(structure.get_attributes().len(), 3);
        
        // Test removing attribute
        structure.remove_attribute("diverse");
        assert!(!structure.has_attribute("diverse"));
        assert_eq!(structure.get_attributes().len(), 2);
        
        // Test getting all attributes
        let attributes = structure.get_attributes();
        assert!(attributes.contains(&"infinite".to_string()));
        assert!(attributes.contains(&"eternal".to_string()));
        assert!(!attributes.contains(&"diverse".to_string()));
    }

    #[test]
    fn test_empty_attributes() {
        let structure = MonadicStructure::new("Test");
        assert_eq!(structure.get_attributes().len(), 0);
        assert!(!structure.has_attribute("anything"));
    }

    #[test]
    fn test_builder_pattern_with_schema() {
        // Test the new semantic workflow: schema first, then content by semantic label
        let structure = MonadicStructure::new("Complete Monad")
            .with_schema(Box::new(BennettMonadSchema))
            .with_content("Unity", "Universal Unity")
            .with_attribute("infinite")
            .with_attribute("eternal")
            .with_attribute("indivisible");
        
        assert_eq!(structure.name, "Complete Monad");
        assert_eq!(structure.positions[0], "Universal Unity");
        assert!(structure.schema.is_some());
        assert_eq!(structure.schema.as_ref().unwrap().get_schema_name(), "Bennett's Monad");
        assert_eq!(structure.attributes.len(), 3);
        assert!(structure.has_attribute("infinite"));
        assert!(structure.has_attribute("eternal"));
        assert!(structure.has_attribute("indivisible"));
    }

    #[test]
    fn test_builder_pattern_direct_position() {
        // Test direct position access (without schema)
        let structure = MonadicStructure::new("Direct Monad")
            .with_position(1, "Direct Unity")
            .with_attribute("simple");
        
        assert_eq!(structure.name, "Direct Monad");
        assert_eq!(structure.positions[0], "Direct Unity");
        assert!(structure.schema.is_none());
        assert_eq!(structure.attributes.len(), 1);
        assert!(structure.has_attribute("simple"));
    }

    #[test]
    fn test_semantic_content_validation() {
        // Test that content requires schema
        let result = std::panic::catch_unwind(|| {
            MonadicStructure::new("Test")
                .with_content("Unity", "Should fail");
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_semantic_label() {
        // Test invalid semantic label
        let result = std::panic::catch_unwind(|| {
            MonadicStructure::new("Test")
                .with_schema(Box::new(BennettMonadSchema))
                .with_content("InvalidLabel", "Should fail");
        });
        assert!(result.is_err());
    }
} 