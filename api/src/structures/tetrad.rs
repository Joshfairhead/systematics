use std::collections::HashMap;
use uuid::Uuid;
use crate::{SystematicStructure, schemas::{Schema, TetradSchema}, error::{Result, SystematicsError}};

/// Tetrad: Four-term systematic structure representing tetradic relationships
/// 
/// Based on Bennett's systematic structures, the tetrad represents four-fold
/// relationships with canonical terms: Ground, Ideal, Instrumental, Directive
#[derive(Debug, Clone)]
pub struct Tetrad {
    // Unique identifier for this structure instance
    id: String,
    
    // User-defined name for this structure
    name: String,
    
    // User's terms for each index position (tetrad has 4 term indices)
    user_term_index: [String; 4],
    
    // Connective relationships between terms (from_index, to_index) -> relationship
    connectives: HashMap<(usize, usize), String>,
    
    // Schema reference
    schema: TetradSchema,
}

impl Tetrad {
    /// Create a new tetrad with the given name and terms
    pub fn new(name: String, ground: String, ideal: String, instrumental: String, directive: String) -> Self {
        let connectives = HashMap::new();
        
        // TODO: Add proper Bennett framework connective relationships
        
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            user_term_index: [ground, ideal, instrumental, directive],
            connectives,
            schema: TetradSchema,
        }
    }
    
    /// Get the first term (Ground)
    pub fn first_term(&self) -> &str {
        &self.user_term_index[0]
    }
    
    /// Get the second term (Ideal)
    pub fn second_term(&self) -> &str {
        &self.user_term_index[1]
    }
    
    /// Get the third term (Instrumental)
    pub fn third_term(&self) -> &str {
        &self.user_term_index[2]
    }
    
    /// Get the fourth term (Directive)
    pub fn fourth_term(&self) -> &str {
        &self.user_term_index[3]
    }
    
    /// Get all terms as a tuple
    pub fn terms_tuple(&self) -> (&str, &str, &str, &str) {
        (&self.user_term_index[0], &self.user_term_index[1], &self.user_term_index[2], &self.user_term_index[3])
    }
    
    /// Get connective relationship between two terms
    pub fn get_connective(&self, from_index: usize, to_index: usize) -> Option<&String> {
        self.connectives.get(&(from_index, to_index))
    }
    
    /// Set connective relationship between two terms
    pub fn set_connective(&mut self, from_index: usize, to_index: usize, relationship: String) {
        if from_index < 4 && to_index < 4 && from_index != to_index {
            self.connectives.insert((from_index, to_index), relationship);
        }
    }
    
    /// Get all connectives
    pub fn connectives(&self) -> &HashMap<(usize, usize), String> {
        &self.connectives
    }
    
    /// Get the number of positional coordinates in this structure
    pub fn position_count(&self) -> usize {
        4
    }
    
        /// Map a term character to its positional coordinate
    /// Returns the 0-based index for the given term character
    pub fn term_character_to_position(&self, term_character: &str) -> Option<usize> {
        let term_characters = self.schema.term_characters();
        term_characters.iter().position(|&term| term == term_character)
    }

    /// Map a positional coordinate to its term character
    /// Returns the term character for the given 0-based position index
    pub fn term_character_from_position(&self, position: usize) -> Option<&str> {
        let term_characters = self.schema.term_characters();
        term_characters.get(position).copied()
    }
    
    /// Map a user instance to its positional coordinate
    /// Returns the 0-based index for the given user instance
    pub fn instance_to_position(&self, instance: &str) -> Option<usize> {
        self.user_term_index.iter().position(|inst| inst == instance)
    }
    
    /// Map a positional coordinate to its user instance
    /// Returns the user instance for the given 0-based position index
    pub fn instance_from_position(&self, position: usize) -> Option<&str> {
        self.user_term_index.get(position).map(|s| s.as_str())
    }
    
        /// Map a position to its term character (alias for term_character_from_position)
    /// Returns the term character for the given 0-based position index
    pub fn position_to_term_character(&self, position: usize) -> Option<&str> {
        self.term_character_from_position(position)
    }

    /// Map a term character to its position (alias for term_character_to_position)
    /// Returns the 0-based index for the given term character
    pub fn position_from_term_character(&self, term_character: &str) -> Option<usize> {
        self.term_character_to_position(term_character)
    }
    
    /// Map a position to its user term (alias for instance_from_position)
    /// Returns the user instance for the given 0-based position index
    pub fn position_to_user_term(&self, position: usize) -> Option<&str> {
        self.instance_from_position(position)
    }
    
    /// Map a user term to its position (alias for instance_to_position)
    /// Returns the 0-based index for the given user instance
    pub fn position_from_user_term(&self, user_term: &str) -> Option<usize> {
        self.instance_to_position(user_term)
    }
}

impl SystematicStructure for Tetrad {
    const TERM_COUNT: usize = 4;
    
    fn id(&self) -> &str {
        &self.id
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn coherence_attribute(&self) -> &str {
        self.schema.coherence_attribute()
    }
    
    fn term_designation(&self) -> &str {
        self.schema.term_designation()
    }
    
    fn term_characters(&self) -> Vec<String> {
        self.schema.term_characters().iter().map(|s| s.to_string()).collect()
    }
    
    fn first_order_connectives_name(&self) -> &str {
        self.schema.first_order_connectives_name()
    }
    
    fn user_terms(&self) -> &[String] {
        &self.user_term_index
    }
    
    fn schema(&self) -> &dyn Schema {
        &self.schema
    }
    
    fn validate(&self) -> Result<()> {
        // Validate name is not empty
        if self.name.trim().is_empty() {
            return Err(SystematicsError::StructureValidation {
                reason: "Tetrad name cannot be empty".to_string(),
            });
        }
        
        // Validate all four terms are not empty
        if self.user_term_index[0].trim().is_empty() {
            return Err(SystematicsError::StructureValidation {
                reason: "First term (Ground) cannot be empty".to_string(),
            });
        }
        
        if self.user_term_index[1].trim().is_empty() {
            return Err(SystematicsError::StructureValidation {
                reason: "Second term (Ideal) cannot be empty".to_string(),
            });
        }
        
        if self.user_term_index[2].trim().is_empty() {
            return Err(SystematicsError::StructureValidation {
                reason: "Third term (Instrumental) cannot be empty".to_string(),
            });
        }
        
        if self.user_term_index[3].trim().is_empty() {
            return Err(SystematicsError::StructureValidation {
                reason: "Fourth term (Directive) cannot be empty".to_string(),
            });
        }
        
        // Validate term lengths
        for (i, term) in self.user_term_index.iter().enumerate() {
            if term.len() > 100 {
                return Err(SystematicsError::StructureValidation {
                    reason: format!("Term {} is too long (max 100 characters)", i + 1),
                });
            }
        }
        
        // Validate terms contain only allowed characters
        for (i, term) in self.user_term_index.iter().enumerate() {
            if !term.chars().all(|c| c.is_alphanumeric() || c.is_whitespace() || ".,!?'-()".contains(c)) {
                return Err(SystematicsError::StructureValidation {
                    reason: format!("Term {} contains invalid characters", i + 1),
                });
            }
        }
        
        // Validate terms are all different (tetrad should represent distinct aspects)
        for i in 0..4 {
            for j in (i + 1)..4 {
                if self.user_term_index[i].trim().to_lowercase() == self.user_term_index[j].trim().to_lowercase() {
                    return Err(SystematicsError::StructureValidation {
                        reason: format!("Terms {} and {} should be different to represent distinct aspects", i + 1, j + 1),
                    });
                }
            }
        }
        
        Ok(())
    }
    
    fn display(&self) {
        println!("\n=== {} ===", self.name);
        println!("Type: Tetrad (4 terms)");
        println!("Terms: {} → {} → {} → {}", 
                 self.user_term_index[0], 
                 self.user_term_index[1], 
                 self.user_term_index[2],
                 self.user_term_index[3]);
        
        // Show key relationships if any exist
        let mut relationships = Vec::new();
        for i in 0..4 {
            for j in 0..4 {
                if i != j {
                    if let Some(rel) = self.get_connective(i, j) {
                        relationships.push(format!("{} {} {}", 
                            self.user_term_index[i], rel, self.user_term_index[j]));
                    }
                }
            }
        }
        
        if !relationships.is_empty() {
            println!("Relationships:");
            for (i, rel) in relationships.iter().enumerate() {
                if i == 0 {
                    println!("  {}", rel);
                } else {
                    println!("  {}", rel);
                }
            }
        }
        
        println!("Schema: {}", self.schema.name());
        println!("ID: {}", &self.id[..8]);
        println!("{}", "=".repeat(self.name.len() + 8));
        println!();
    }
}

/// Builder for creating Tetrad structures
pub struct TetradBuilder {
    name: Option<String>,
    ground: Option<String>,
    ideal: Option<String>,
    instrumental: Option<String>,
    directive: Option<String>,
}

impl TetradBuilder {
    pub fn new() -> Self {
        Self {
            name: None,
            ground: None,
            ideal: None,
            instrumental: None,
            directive: None,
        }
    }
    
    /// Set the name for the tetrad
    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }
    
    /// Set the terms for the tetrad
    pub fn terms<S1: Into<String>, S2: Into<String>, S3: Into<String>, S4: Into<String>>(
        mut self, 
        ground: S1, 
        ideal: S2, 
        instrumental: S3, 
        directive: S4
    ) -> Self {
        self.ground = Some(ground.into());
        self.ideal = Some(ideal.into());
        self.instrumental = Some(instrumental.into());
        self.directive = Some(directive.into());
        self
    }
    
    /// Set custom connectives (placeholder for future use)
    pub fn connectives(self, _connectives: HashMap<(usize, usize), String>) -> Self {
        // This method is kept for compatibility but doesn't modify anything yet
        self
    }
    
    /// Build the tetrad
    pub fn build(self) -> Result<Tetrad> {
        let name = self.name.unwrap_or_else(|| "Unnamed Tetrad".to_string());
        let ground = self.ground.ok_or_else(|| SystematicsError::Builder {
            reason: "Tetrad requires a ground term".to_string(),
        })?;
        let ideal = self.ideal.ok_or_else(|| SystematicsError::Builder {
            reason: "Tetrad requires an ideal term".to_string(),
        })?;
        let instrumental = self.instrumental.ok_or_else(|| SystematicsError::Builder {
            reason: "Tetrad requires an instrumental term".to_string(),
        })?;
        let directive = self.directive.ok_or_else(|| SystematicsError::Builder {
            reason: "Tetrad requires a directive term".to_string(),
        })?;
        
        let tetrad = Tetrad::new(name, ground, ideal, instrumental, directive);
        
        tetrad.validate()?;
        Ok(tetrad)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tetrad_creation() {
        let tetrad = TetradBuilder::new()
            .name("Test Tetrad")
            .terms("Foundation", "Vision", "Method", "Guide")
            .build()
            .unwrap();
        
        assert_eq!(tetrad.name(), "Test Tetrad");
        assert_eq!(tetrad.first_term(), "Foundation");
        assert_eq!(tetrad.second_term(), "Vision");
        assert_eq!(tetrad.third_term(), "Method");
        assert_eq!(tetrad.fourth_term(), "Guide");
        assert_eq!(tetrad.terms_tuple(), ("Foundation", "Vision", "Method", "Guide"));
        assert!(tetrad.validate().is_ok());
    }

    #[test]
    fn test_term_characters() {
        let tetrad = TetradBuilder::new()
            .name("Test Tetrad")
            .terms("A", "B", "C", "D")
            .build()
            .unwrap();
        
        let characters = tetrad.term_characters();
        assert_eq!(characters, vec!["Ground", "Ideal", "Instrumental", "Directive"]);
    }

    #[test]
    fn test_connective_relationships() {
        let tetrad = TetradBuilder::new()
            .name("Test Tetrad")
            .terms("Ground", "Ideal", "Instrumental", "Directive")
            .build()
            .unwrap();
        
        // No default connectives (proper Bennett framework relationships not implemented yet)
        assert_eq!(tetrad.get_connective(0, 1), None);
        assert_eq!(tetrad.get_connective(1, 2), None);
        assert_eq!(tetrad.get_connective(2, 3), None);
        assert_eq!(tetrad.get_connective(3, 0), None);
    }

    #[test]
    fn test_tetrad_validation() {
        // Valid tetrad
        let valid_tetrad = TetradBuilder::new()
            .name("Valid")
            .terms("A", "B", "C", "D")
            .build();
        assert!(valid_tetrad.is_ok());
        
        // Empty term should fail
        let invalid_tetrad = TetradBuilder::new()
            .name("Invalid")
            .terms("A", "", "C", "D")
            .build();
        assert!(invalid_tetrad.is_err());
        
        // Duplicate terms should fail
        let duplicate_tetrad = TetradBuilder::new()
            .name("Duplicate")
            .terms("Same", "Same", "Different", "Another")
            .build();
        assert!(duplicate_tetrad.is_err());
    }

    #[test]
    fn test_tetrad_terms_method() {
        let tetrad = TetradBuilder::new()
            .name("Test")
            .terms("One", "Two", "Three", "Four")
            .build()
            .unwrap();
        
        let user_terms = tetrad.user_terms();
        assert_eq!(user_terms.len(), 4);
        assert_eq!(user_terms[0], "One");
        assert_eq!(user_terms[1], "Two");
        assert_eq!(user_terms[2], "Three");
        assert_eq!(user_terms[3], "Four");
    }

    #[test]
    fn test_trait_compliance() {
        let tetrad = TetradBuilder::new()
            .name("Test")
            .terms("A", "B", "C", "D")
            .build()
            .unwrap();
        
        assert_eq!(Tetrad::TERM_COUNT, 4);
        assert!(!tetrad.id().is_empty());
        assert_eq!(tetrad.name(), "Test");
        assert!(tetrad.validate().is_ok());
    }
    
    #[test]
    fn test_positional_coordinate_mapping() {
        let tetrad = TetradBuilder::new()
            .name("Test")
            .terms("MyGround", "MyIdeal", "MyInstrumental", "MyDirective")
            .build()
            .unwrap();
        
        // Test term character to position mapping
        assert_eq!(tetrad.term_character_to_position("Ground"), Some(0));
        assert_eq!(tetrad.term_character_to_position("Ideal"), Some(1));
        assert_eq!(tetrad.term_character_to_position("Instrumental"), Some(2));
        assert_eq!(tetrad.term_character_to_position("Directive"), Some(3));
        assert_eq!(tetrad.term_character_to_position("Invalid"), None);
        
        // Test position to term character mapping
        assert_eq!(tetrad.term_character_from_position(0), Some("Ground"));
        assert_eq!(tetrad.term_character_from_position(1), Some("Ideal"));
        assert_eq!(tetrad.term_character_from_position(2), Some("Instrumental"));
        assert_eq!(tetrad.term_character_from_position(3), Some("Directive"));
        assert_eq!(tetrad.term_character_from_position(4), None);
        
        // Test position count
        assert_eq!(tetrad.position_count(), 4);
        
        // Test user instance to position mapping
        assert_eq!(tetrad.instance_to_position("MyGround"), Some(0));
        assert_eq!(tetrad.instance_to_position("MyIdeal"), Some(1));
        assert_eq!(tetrad.instance_to_position("MyInstrumental"), Some(2));
        assert_eq!(tetrad.instance_to_position("MyDirective"), Some(3));
        assert_eq!(tetrad.instance_to_position("Invalid"), None);
        
        // Test position to user instance mapping
        assert_eq!(tetrad.instance_from_position(0), Some("MyGround"));
        assert_eq!(tetrad.instance_from_position(1), Some("MyIdeal"));
        assert_eq!(tetrad.instance_from_position(2), Some("MyInstrumental"));
        assert_eq!(tetrad.instance_from_position(3), Some("MyDirective"));
        assert_eq!(tetrad.instance_from_position(4), None);
    }
    
    #[test]
    fn test_position_alias_methods() {
        let tetrad = TetradBuilder::new()
            .name("Test")
            .terms("MyGround", "MyIdeal", "MyInstrumental", "MyDirective")
            .build()
            .unwrap();
        
        // Test term character position aliases
        assert_eq!(tetrad.position_to_term_character(0), Some("Ground"));
        assert_eq!(tetrad.position_to_term_character(1), Some("Ideal"));
        assert_eq!(tetrad.position_to_term_character(2), Some("Instrumental"));
        assert_eq!(tetrad.position_to_term_character(3), Some("Directive"));
        assert_eq!(tetrad.position_to_term_character(4), None);
        
        assert_eq!(tetrad.position_from_term_character("Ground"), Some(0));
        assert_eq!(tetrad.position_from_term_character("Ideal"), Some(1));
        assert_eq!(tetrad.position_from_term_character("Instrumental"), Some(2));
        assert_eq!(tetrad.position_from_term_character("Directive"), Some(3));
        assert_eq!(tetrad.position_from_term_character("NonExistent"), None);
        
        // Test user term position aliases
        assert_eq!(tetrad.position_to_user_term(0), Some("MyGround"));
        assert_eq!(tetrad.position_to_user_term(1), Some("MyIdeal"));
        assert_eq!(tetrad.position_to_user_term(2), Some("MyInstrumental"));
        assert_eq!(tetrad.position_to_user_term(3), Some("MyDirective"));
        assert_eq!(tetrad.position_to_user_term(4), None);
        
        assert_eq!(tetrad.position_from_user_term("MyGround"), Some(0));
        assert_eq!(tetrad.position_from_user_term("MyIdeal"), Some(1));
        assert_eq!(tetrad.position_from_user_term("MyInstrumental"), Some(2));
        assert_eq!(tetrad.position_from_user_term("MyDirective"), Some(3));
        assert_eq!(tetrad.position_from_user_term("Unknown"), None);
        
        // Verify aliases return same results as original methods
        assert_eq!(tetrad.position_to_term_character(0), tetrad.term_character_from_position(0));
        assert_eq!(tetrad.position_from_term_character("Ground"), tetrad.term_character_to_position("Ground"));
        assert_eq!(tetrad.position_to_user_term(0), tetrad.instance_from_position(0));
        assert_eq!(tetrad.position_from_user_term("MyGround"), tetrad.instance_to_position("MyGround"));
    }
} 