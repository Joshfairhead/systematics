use std::collections::HashMap;
use uuid::Uuid;
use crate::{SystematicStructure, schemas::{Schema, TriadSchema}, error::{Result, SystematicsError}};

/// Triad: Three-term systematic structure representing triadic relationships
/// 
/// Based on Bennett's systematic structures, the triad represents three-fold
/// relationships with canonical terms: Will, Function, Being
#[derive(Debug, Clone)]
pub struct Triad {
    // Unique identifier for this structure instance
    id: String,
    
    // User-defined name for this structure
    name: String,
    
    // User's terms for each index position (triad has 3 term indices)
    user_term_index: [String; 3],
    
    // User-defined attributes
    attributes: Vec<String>,
    
    // Connective relationships between terms (from_index, to_index) -> relationship
    connectives: HashMap<(usize, usize), String>,
    
    // Schema reference
    schema: TriadSchema,
}

impl Triad {
    /// Create a new triad with default empty terms
    pub fn new(name: String) -> Self {
        let connectives = HashMap::new();
        
        // TODO: Add proper Bennett framework connective relationships
        // connectives.insert((0, 1), "directs".to_string());     // Will directs Function
        // connectives.insert((1, 2), "manifests".to_string());  // Function manifests Being  
        // connectives.insert((2, 0), "informs".to_string());    // Being informs Will
        
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            user_term_index: [String::new(), String::new(), String::new()],
            attributes: Vec::new(),
            connectives,
            schema: TriadSchema,
        }
    }
    
    /// Get the first term (Will)
    pub fn first_term(&self) -> &str {
        &self.user_term_index[0]
    }
    
    /// Get the second term (Function)
    pub fn second_term(&self) -> &str {
        &self.user_term_index[1]
    }
    
    /// Get the third term (Being)
    pub fn third_term(&self) -> &str {
        &self.user_term_index[2]
    }
    
    /// Get all terms as a tuple
    pub fn terms_tuple(&self) -> (&str, &str, &str) {
        (&self.user_term_index[0], &self.user_term_index[1], &self.user_term_index[2])
    }
    
    /// Add an attribute to the triad
    pub fn add_attribute(&mut self, attribute: String) {
        if !self.attributes.contains(&attribute) {
            self.attributes.push(attribute);
        }
    }
    
    /// Remove an attribute from the triad
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
        if from_index < 3 && to_index < 3 && from_index != to_index {
            self.connectives.insert((from_index, to_index), relationship);
        }
    }
    
    /// Get all connectives
    pub fn connectives(&self) -> &HashMap<(usize, usize), String> {
        &self.connectives
    }
}

impl SystematicStructure for Triad {
    const TERM_COUNT: usize = 3;
    
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
                reason: "Triad name cannot be empty".to_string(),
            });
        }
        
        // Validate all three terms are not empty
        if self.user_term_index[0].trim().is_empty() {
            return Err(SystematicsError::StructureValidation {
                reason: "First term (Will) cannot be empty".to_string(),
            });
        }
        
        if self.user_term_index[1].trim().is_empty() {
            return Err(SystematicsError::StructureValidation {
                reason: "Second term (Function) cannot be empty".to_string(),
            });
        }
        
        if self.user_term_index[2].trim().is_empty() {
            return Err(SystematicsError::StructureValidation {
                reason: "Third term (Being) cannot be empty".to_string(),
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
        
        // Validate terms are all different (triad should represent distinct aspects)
        if self.user_term_index[0].trim().to_lowercase() == self.user_term_index[1].trim().to_lowercase() ||
           self.user_term_index[0].trim().to_lowercase() == self.user_term_index[2].trim().to_lowercase() ||
           self.user_term_index[1].trim().to_lowercase() == self.user_term_index[2].trim().to_lowercase() {
            return Err(SystematicsError::StructureValidation {
                reason: "All triad terms should be different to represent distinct aspects".to_string(),
            });
        }
        
        Ok(())
    }
    
    fn display(&self) {
        println!("=== {} ===", self.name);
        println!("Type: Triad ({} terms)", Self::TERM_COUNT);
        println!("Terms: {} → {} → {}", 
                 self.user_term_index[0], 
                 self.user_term_index[1], 
                 self.user_term_index[2]);
        
        // Show key relationships
        if let Some(rel1) = self.get_connective(0, 1) {
            println!("Relationship: {} {} {}", 
                    self.user_term_index[0], rel1, self.user_term_index[1]);
        }
        if let Some(rel2) = self.get_connective(1, 2) {
            println!("             {} {} {}", 
                    self.user_term_index[1], rel2, self.user_term_index[2]);
        }
        if let Some(rel3) = self.get_connective(2, 0) {
            println!("             {} {} {}", 
                    self.user_term_index[2], rel3, self.user_term_index[0]);
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

/// Builder for creating Triad structures
pub struct TriadBuilder {
    name: String,
    terms: [String; 3],
    attributes: Vec<String>,
    connectives: Option<HashMap<(usize, usize), String>>,
}

impl TriadBuilder {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            terms: [String::new(), String::new(), String::new()],
            attributes: Vec::new(),
            connectives: None,
        }
    }
    
    /// Set the name for the triad
    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }
    
    /// Set the terms for the triad
    pub fn terms(mut self, first: String, second: String, third: String) -> Self {
        self.terms = [first, second, third];
        self
    }
    
    /// Add attributes to the triad
    pub fn attributes(mut self, attributes: Vec<String>) -> Self {
        self.attributes = attributes;
        self
    }
    
    /// Set custom connectives
    pub fn connectives(mut self, connectives: HashMap<(usize, usize), String>) -> Self {
        self.connectives = Some(connectives);
        self
    }
    
    /// Build the triad
    pub fn build(self) -> Result<Triad> {
        let mut triad = Triad::new(self.name);
        triad.user_term_index = self.terms;
        triad.attributes = self.attributes;
        
        if let Some(custom_connectives) = self.connectives {
            triad.connectives = custom_connectives;
        }
        
        // Validate before returning
        triad.validate()?;
        Ok(triad)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triad_creation() {
        let triad = TriadBuilder::new()
            .name("Test Triad".to_string())
            .terms("Intention".to_string(), "Action".to_string(), "Result".to_string())
            .build()
            .unwrap();
        
        assert_eq!(triad.name(), "Test Triad");
        assert_eq!(triad.first_term(), "Intention");
        assert_eq!(triad.second_term(), "Action");
        assert_eq!(triad.third_term(), "Result");
        assert_eq!(triad.terms_tuple(), ("Intention", "Action", "Result"));
    }

    #[test]
    fn test_canonical_terms() {
        let triad = TriadBuilder::new()
            .name("Test Triad".to_string())
            .terms("A".to_string(), "B".to_string(), "C".to_string())
            .build()
            .unwrap();
        
        let canonical = triad.canonical_terms();
        assert_eq!(canonical, vec!["Will", "Function", "Being"]);
    }

    #[test]
    fn test_attribute_management() {
        let mut triad = TriadBuilder::new()
            .name("Test Triad".to_string())
            .terms("A".to_string(), "B".to_string(), "C".to_string())
            .build()
            .unwrap();
        
        triad.add_attribute("dynamic".to_string());
        triad.add_attribute("structured".to_string());
        assert_eq!(triad.attributes().len(), 2);
        
        triad.remove_attribute("dynamic");
        assert_eq!(triad.attributes().len(), 1);
        assert_eq!(triad.attributes()[0], "structured");
    }

    #[test]
    fn test_connective_relationships() {
        let triad = TriadBuilder::new()
            .name("Test Triad".to_string())
            .terms("Will".to_string(), "Function".to_string(), "Being".to_string())
            .build()
            .unwrap();
        
        // No default connectives (commented out fabricated relationships)
        assert_eq!(triad.get_connective(0, 1), None);
        assert_eq!(triad.get_connective(1, 2), None);
        assert_eq!(triad.get_connective(2, 0), None);
    }

    #[test]
    fn test_triad_validation() {
        // Valid triad
        let valid_triad = TriadBuilder::new()
            .name("Valid".to_string())
            .terms("A".to_string(), "B".to_string(), "C".to_string())
            .build();
        assert!(valid_triad.is_ok());
        
        // Empty term should fail
        let invalid_triad = TriadBuilder::new()
            .name("Invalid".to_string())
            .terms("A".to_string(), "".to_string(), "C".to_string())
            .build();
        assert!(invalid_triad.is_err());
        
        // Duplicate terms should fail
        let duplicate_triad = TriadBuilder::new()
            .name("Duplicate".to_string())
            .terms("Same".to_string(), "Same".to_string(), "Different".to_string())
            .build();
        assert!(duplicate_triad.is_err());
    }

    #[test]
    fn test_triad_terms_method() {
        let triad = TriadBuilder::new()
            .name("Test".to_string())
            .terms("One".to_string(), "Two".to_string(), "Three".to_string())
            .build()
            .unwrap();
        
        let user_terms = triad.user_terms();
        assert_eq!(user_terms.len(), 3);
        assert_eq!(user_terms[0], "One");
        assert_eq!(user_terms[1], "Two");
        assert_eq!(user_terms[2], "Three");
    }

    #[test]
    fn test_trait_compliance() {
        let triad = TriadBuilder::new()
            .name("Test".to_string())
            .terms("A".to_string(), "B".to_string(), "C".to_string())
            .build()
            .unwrap();
        
        assert_eq!(Triad::TERM_COUNT, 3);
        assert!(!triad.id().is_empty());
        assert_eq!(triad.name(), "Test");
        assert!(triad.validate().is_ok());
    }
} 