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
    
    // User-defined attributes
    attributes: Vec<String>,
    
    // Connective relationships between terms (from_index, to_index) -> relationship
    connectives: HashMap<(usize, usize), String>,
    
    // Schema reference
    schema: TetradSchema,
}

impl Tetrad {
    /// Create a new tetrad with default empty terms
    pub fn new(name: String) -> Self {
        let connectives = HashMap::new();
        
        // TODO: Add proper Bennett framework connective relationships
        
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            user_term_index: [String::new(), String::new(), String::new(), String::new()],
            attributes: Vec::new(),
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
    
    /// Add an attribute to the tetrad
    pub fn add_attribute(&mut self, attribute: String) {
        if !self.attributes.contains(&attribute) {
            self.attributes.push(attribute);
        }
    }
    
    /// Remove an attribute from the tetrad
    pub fn remove_attribute(&mut self, attribute: &str) {
        self.attributes.retain(|attr| attr != attribute);
    }
    
    /// Get attributes
    pub fn attributes(&self) -> &[String] {
        &self.attributes
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
}

impl SystematicStructure for Tetrad {
    const TERM_COUNT: usize = 4;
    
    fn id(&self) -> &str {
        &self.id
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn canonical_terms(&self) -> Vec<String> {
        self.schema.canonical_terms().iter().map(|s| s.to_string()).collect()
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
        println!("=== {} ===", self.name);
        println!("Type: Tetrad ({} terms)", Self::TERM_COUNT);
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
        
        if !self.attributes.is_empty() {
            println!("Attributes: {}", self.attributes.join(", "));
        }
        println!("Schema: {}", self.schema.name());
        println!("ID: {}", &self.id[..8]);
        println!("{}", "=".repeat(self.name.len() + 8));
        println!();
    }
}

/// Builder for creating Tetrad structures
pub struct TetradBuilder {
    name: String,
    terms: [String; 4],
    attributes: Vec<String>,
    connectives: Option<HashMap<(usize, usize), String>>,
}

impl TetradBuilder {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            terms: [String::new(), String::new(), String::new(), String::new()],
            attributes: Vec::new(),
            connectives: None,
        }
    }
    
    /// Set the name for the tetrad
    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }
    
    /// Set the terms for the tetrad
    pub fn terms(mut self, first: String, second: String, third: String, fourth: String) -> Self {
        self.terms = [first, second, third, fourth];
        self
    }
    
    /// Add attributes to the tetrad
    pub fn attributes(mut self, attributes: Vec<String>) -> Self {
        self.attributes = attributes;
        self
    }
    
    /// Set custom connectives
    pub fn connectives(mut self, connectives: HashMap<(usize, usize), String>) -> Self {
        self.connectives = Some(connectives);
        self
    }
    
    /// Build the tetrad
    pub fn build(self) -> Result<Tetrad> {
        let mut tetrad = Tetrad::new(self.name);
        tetrad.user_term_index = self.terms;
        tetrad.attributes = self.attributes;
        
        if let Some(custom_connectives) = self.connectives {
            tetrad.connectives = custom_connectives;
        }
        
        // Validate before returning
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
            .name("Test Tetrad".to_string())
            .terms("Origin".to_string(), "Path".to_string(), "Drive".to_string(), "Governance".to_string())
            .build()
            .unwrap();
        
        assert_eq!(tetrad.name(), "Test Tetrad");
        assert_eq!(tetrad.first_term(), "Origin");
        assert_eq!(tetrad.second_term(), "Path");
        assert_eq!(tetrad.third_term(), "Drive");
        assert_eq!(tetrad.fourth_term(), "Governance");
        assert_eq!(tetrad.terms_tuple(), ("Origin", "Path", "Drive", "Governance"));
    }

    #[test]
    fn test_canonical_terms() {
        let tetrad = TetradBuilder::new()
            .name("Test Tetrad".to_string())
            .terms("A".to_string(), "B".to_string(), "C".to_string(), "D".to_string())
            .build()
            .unwrap();
        
        let canonical = tetrad.canonical_terms();
        assert_eq!(canonical, vec!["Ground", "Ideal", "Instrumental", "Directive"]);
    }

    #[test]
    fn test_attribute_management() {
        let mut tetrad = TetradBuilder::new()
            .name("Test Tetrad".to_string())
            .terms("A".to_string(), "B".to_string(), "C".to_string(), "D".to_string())
            .build()
            .unwrap();
        
        tetrad.add_attribute("complex".to_string());
        tetrad.add_attribute("structured".to_string());
        assert_eq!(tetrad.attributes().len(), 2);
        
        tetrad.remove_attribute("complex");
        assert_eq!(tetrad.attributes().len(), 1);
        assert_eq!(tetrad.attributes()[0], "structured");
    }

    #[test]
    fn test_connective_relationships() {
        let tetrad = TetradBuilder::new()
            .name("Test Tetrad".to_string())
            .terms("Ground".to_string(), "Ideal".to_string(), "Instrumental".to_string(), "Directive".to_string())
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
            .name("Valid".to_string())
            .terms("A".to_string(), "B".to_string(), "C".to_string(), "D".to_string())
            .build();
        assert!(valid_tetrad.is_ok());
        
        // Empty term should fail
        let invalid_tetrad = TetradBuilder::new()
            .name("Invalid".to_string())
            .terms("A".to_string(), "".to_string(), "C".to_string(), "D".to_string())
            .build();
        assert!(invalid_tetrad.is_err());
        
        // Duplicate terms should fail
        let duplicate_tetrad = TetradBuilder::new()
            .name("Duplicate".to_string())
            .terms("Same".to_string(), "Same".to_string(), "Different".to_string(), "Another".to_string())
            .build();
        assert!(duplicate_tetrad.is_err());
    }

    #[test]
    fn test_tetrad_terms_method() {
        let tetrad = TetradBuilder::new()
            .name("Test".to_string())
            .terms("One".to_string(), "Two".to_string(), "Three".to_string(), "Four".to_string())
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
            .name("Test".to_string())
            .terms("A".to_string(), "B".to_string(), "C".to_string(), "D".to_string())
            .build()
            .unwrap();
        
        assert_eq!(Tetrad::TERM_COUNT, 4);
        assert!(!tetrad.id().is_empty());
        assert_eq!(tetrad.name(), "Test");
        assert!(tetrad.validate().is_ok());
    }
} 