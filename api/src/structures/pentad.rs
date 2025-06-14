use std::collections::HashMap;
use uuid::Uuid;
use crate::{SystematicStructure, schemas::{Schema, PentadSchema}, error::{Result, SystematicsError}};

/// Pentad: Five-term systematic structure representing pentadic relationships
/// 
/// Based on Bennett's systematic structures, the pentad represents five-fold
/// relationships with canonical terms: Quintessence, Higher Potential, Lower Potential, Purpose, Source
#[derive(Debug, Clone)]
pub struct Pentad {
    // Unique identifier for this structure instance
    id: String,
    
    // User-defined name for this structure
    name: String,
    
    // User's terms for each index position (pentad has 5 term indices)
    user_term_index: [String; 5],
    
    // User-defined attributes
    attributes: Vec<String>,
    
    // Connective relationships between terms (from_index, to_index) -> relationship
    connectives: HashMap<(usize, usize), String>,
    
    // Schema reference
    schema: PentadSchema,
}

impl Pentad {
    /// Create a new pentad with default empty terms
    pub fn new(name: String) -> Self {
        let connectives = HashMap::new();
        
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            user_term_index: [String::new(), String::new(), String::new(), String::new(), String::new()],
            attributes: Vec::new(),
            connectives,
            schema: PentadSchema,
        }
    }
    
    /// Get the first term (Quintessence)
    pub fn first_term(&self) -> &str {
        &self.user_term_index[0]
    }
    
    /// Get the second term (Higher Potential)
    pub fn second_term(&self) -> &str {
        &self.user_term_index[1]
    }
    
    /// Get the third term (Lower Potential)
    pub fn third_term(&self) -> &str {
        &self.user_term_index[2]
    }
    
    /// Get the fourth term (Purpose)
    pub fn fourth_term(&self) -> &str {
        &self.user_term_index[3]
    }
    
    /// Get the fifth term (Source)
    pub fn fifth_term(&self) -> &str {
        &self.user_term_index[4]
    }
    
    /// Get all terms as a tuple
    pub fn terms_tuple(&self) -> (&str, &str, &str, &str, &str) {
        (&self.user_term_index[0], &self.user_term_index[1], &self.user_term_index[2], &self.user_term_index[3], &self.user_term_index[4])
    }
    
    /// Add an attribute to the pentad
    pub fn add_attribute(&mut self, attribute: String) {
        if !self.attributes.contains(&attribute) {
            self.attributes.push(attribute);
        }
    }
    
    /// Remove an attribute from the pentad
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
        if from_index < 5 && to_index < 5 && from_index != to_index {
            self.connectives.insert((from_index, to_index), relationship);
        }
    }
    
    /// Get all connectives
    pub fn connectives(&self) -> &HashMap<(usize, usize), String> {
        &self.connectives
    }
}

impl SystematicStructure for Pentad {
    const TERM_COUNT: usize = 5;
    
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
                reason: "Pentad name cannot be empty".to_string(),
            });
        }
        
        // Validate all five terms are not empty
        let term_names = ["Quintessence", "Higher Potential", "Lower Potential", "Purpose", "Source"];
        for (i, term) in self.user_term_index.iter().enumerate() {
            if term.trim().is_empty() {
                return Err(SystematicsError::StructureValidation {
                    reason: format!("{} term ({}) cannot be empty", 
                        match i {
                            0 => "First",
                            1 => "Second", 
                            2 => "Third",
                            3 => "Fourth",
                            4 => "Fifth",
                            _ => "Unknown"
                        }, 
                        term_names[i]
                    ),
                });
            }
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
        
        // Validate terms are all different (pentad should represent distinct aspects)
        for i in 0..5 {
            for j in (i + 1)..5 {
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
        println!("Type: Pentad ({} terms)", Self::TERM_COUNT);
        println!("Terms: {} → {} → {} → {} → {}", 
                 self.user_term_index[0], 
                 self.user_term_index[1], 
                 self.user_term_index[2], 
                 self.user_term_index[3], 
                 self.user_term_index[4]);
        
        if !self.attributes.is_empty() {
            println!("Attributes: {}", self.attributes.join(", "));
        }
        
        if !self.connectives.is_empty() {
            println!("Connectives:");
            for ((from, to), relationship) in &self.connectives {
                println!("  {} → {}: {}", 
                    self.user_term_index[*from], 
                    self.user_term_index[*to], 
                    relationship);
            }
        }
    }
}

/// Builder for creating Pentad instances
pub struct PentadBuilder {
    name: String,
    terms: [String; 5],
    attributes: Vec<String>,
    connectives: Option<HashMap<(usize, usize), String>>,
}

impl PentadBuilder {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            terms: [String::new(), String::new(), String::new(), String::new(), String::new()],
            attributes: Vec::new(),
            connectives: None,
        }
    }
    
    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }
    
    pub fn terms(mut self, first: String, second: String, third: String, fourth: String, fifth: String) -> Self {
        self.terms = [first, second, third, fourth, fifth];
        self
    }
    
    pub fn attributes(mut self, attributes: Vec<String>) -> Self {
        self.attributes = attributes;
        self
    }
    
    pub fn connectives(mut self, connectives: HashMap<(usize, usize), String>) -> Self {
        self.connectives = Some(connectives);
        self
    }
    
    pub fn build(self) -> Result<Pentad> {
        let pentad = Pentad {
            id: Uuid::new_v4().to_string(),
            name: self.name,
            user_term_index: self.terms,
            attributes: self.attributes,
            connectives: self.connectives.unwrap_or_else(HashMap::new),
            schema: PentadSchema,
        };
        
        // Validate the built pentad
        pentad.validate()?;
        
        Ok(pentad)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pentad_creation() {
        let pentad = PentadBuilder::new()
            .name("Test Pentad".to_string())
            .terms(
                "Essence".to_string(),
                "Higher Aspect".to_string(), 
                "Lower Aspect".to_string(),
                "Goal".to_string(),
                "Origin".to_string()
            )
            .build()
            .unwrap();
        
        assert_eq!(pentad.name(), "Test Pentad");
        assert_eq!(pentad.first_term(), "Essence");
        assert_eq!(pentad.second_term(), "Higher Aspect");
        assert_eq!(pentad.third_term(), "Lower Aspect");
        assert_eq!(pentad.fourth_term(), "Goal");
        assert_eq!(pentad.fifth_term(), "Origin");
    }

    #[test]
    fn test_canonical_terms() {
        let pentad = PentadBuilder::new()
            .name("Test".to_string())
            .terms("A".to_string(), "B".to_string(), "C".to_string(), "D".to_string(), "E".to_string())
            .build()
            .unwrap();
        
        let canonical = pentad.canonical_terms();
        assert_eq!(canonical.len(), 5);
        assert_eq!(canonical[0], "Quintessence");
        assert_eq!(canonical[1], "Higher Potential");
        assert_eq!(canonical[2], "Lower Potential");
        assert_eq!(canonical[3], "Purpose");
        assert_eq!(canonical[4], "Source");
    }

    #[test]
    fn test_attribute_management() {
        let mut pentad = PentadBuilder::new()
            .name("Test".to_string())
            .terms("A".to_string(), "B".to_string(), "C".to_string(), "D".to_string(), "E".to_string())
            .build()
            .unwrap();
        
        pentad.add_attribute("Test Attribute".to_string());
        assert_eq!(pentad.attributes().len(), 1);
        assert_eq!(pentad.attributes()[0], "Test Attribute");
        
        pentad.remove_attribute("Test Attribute");
        assert_eq!(pentad.attributes().len(), 0);
    }

    #[test]
    fn test_connective_relationships() {
        let mut pentad = PentadBuilder::new()
            .name("Test".to_string())
            .terms("A".to_string(), "B".to_string(), "C".to_string(), "D".to_string(), "E".to_string())
            .build()
            .unwrap();
        
        pentad.set_connective(0, 1, "test relationship".to_string());
        assert_eq!(pentad.get_connective(0, 1), Some(&"test relationship".to_string()));
        assert_eq!(pentad.connectives().len(), 1);
    }

    #[test]
    fn test_pentad_validation() {
        // Test empty name
        let result = PentadBuilder::new()
            .name("".to_string())
            .terms("A".to_string(), "B".to_string(), "C".to_string(), "D".to_string(), "E".to_string())
            .build();
        assert!(result.is_err());
        
        // Test empty term
        let result = PentadBuilder::new()
            .name("Test".to_string())
            .terms("".to_string(), "B".to_string(), "C".to_string(), "D".to_string(), "E".to_string())
            .build();
        assert!(result.is_err());
        
        // Test duplicate terms
        let result = PentadBuilder::new()
            .name("Test".to_string())
            .terms("A".to_string(), "A".to_string(), "C".to_string(), "D".to_string(), "E".to_string())
            .build();
        assert!(result.is_err());
        
        // Test valid pentad
        let result = PentadBuilder::new()
            .name("Valid".to_string())
            .terms("A".to_string(), "B".to_string(), "C".to_string(), "D".to_string(), "E".to_string())
            .build();
        assert!(result.is_ok());
    }

    #[test]
    fn test_pentad_terms_method() {
        let pentad = PentadBuilder::new()
            .name("Test".to_string())
            .terms("First".to_string(), "Second".to_string(), "Third".to_string(), "Fourth".to_string(), "Fifth".to_string())
            .build()
            .unwrap();
        
        let terms = pentad.user_terms();
        assert_eq!(terms.len(), 5);
        assert_eq!(terms[0], "First");
        assert_eq!(terms[1], "Second");
        assert_eq!(terms[2], "Third");
        assert_eq!(terms[3], "Fourth");
        assert_eq!(terms[4], "Fifth");
        
        let tuple = pentad.terms_tuple();
        assert_eq!(tuple, ("First", "Second", "Third", "Fourth", "Fifth"));
    }

    #[test]
    fn test_trait_compliance() {
        let pentad = PentadBuilder::new()
            .name("Test".to_string())
            .terms("A".to_string(), "B".to_string(), "C".to_string(), "D".to_string(), "E".to_string())
            .build()
            .unwrap();
        
        // Test SystematicStructure trait
        assert_eq!(Pentad::TERM_COUNT, 5);
        assert!(!pentad.id().is_empty());
        assert_eq!(pentad.name(), "Test");
        assert_eq!(pentad.user_terms().len(), 5);
        assert!(pentad.validate().is_ok());
    }
} 