use std::collections::HashMap;
use uuid::Uuid;
use crate::{SystematicStructure, schemas::{Schema, OctadSchema}, error::{Result, SystematicsError}};

/// Octad: Eight-term systematic structure representing octadic relationships
/// 
/// Based on Bennett's systematic structures, the octad represents eight-fold
/// relationships representing organizational wholeness and completion
#[derive(Debug, Clone)]
pub struct Octad {
    // Unique identifier for this structure instance
    id: String,
    
    // User-defined name for this structure
    name: String,
    
    // User's terms for each index position (octad has 8 term indices)
    user_term_index: [String; 8],
    
    // User-defined attributes
    attributes: Vec<String>,
    
    // Connective relationships between terms (from_index, to_index) -> relationship
    connectives: HashMap<(usize, usize), String>,
    
    // Schema reference
    schema: OctadSchema,
}

impl Octad {
    /// Create a new octad with default empty terms
    pub fn new(name: String) -> Self {
        let connectives = HashMap::new();
        
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            user_term_index: [
                String::new(), String::new(), String::new(), String::new(),
                String::new(), String::new(), String::new(), String::new()
            ],
            attributes: Vec::new(),
            connectives,
            schema: OctadSchema,
        }
    }
    
    /// Get term by index (0-7)
    pub fn get_term(&self, index: usize) -> Option<&str> {
        self.user_term_index.get(index).map(|s| s.as_str())
    }
    
    /// Add an attribute to the octad
    pub fn add_attribute(&mut self, attribute: String) {
        if !self.attributes.contains(&attribute) {
            self.attributes.push(attribute);
        }
    }
    
    /// Remove an attribute from the octad
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
        if from_index < 8 && to_index < 8 && from_index != to_index {
            self.connectives.insert((from_index, to_index), relationship);
        }
    }
    
    /// Get all connectives
    pub fn connectives(&self) -> &HashMap<(usize, usize), String> {
        &self.connectives
    }
}

impl SystematicStructure for Octad {
    const TERM_COUNT: usize = 8;
    
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
                reason: "Octad name cannot be empty".to_string(),
            });
        }
        
        // Validate all eight terms are not empty
        for (i, term) in self.user_term_index.iter().enumerate() {
            if term.trim().is_empty() {
                return Err(SystematicsError::StructureValidation {
                    reason: format!("Term {} cannot be empty", i + 1),
                });
            }
        }
        
        // Validate term lengths and characters
        for (i, term) in self.user_term_index.iter().enumerate() {
            if term.len() > 100 {
                return Err(SystematicsError::StructureValidation {
                    reason: format!("Term {} is too long (max 100 characters)", i + 1),
                });
            }
            
            if !term.chars().all(|c| c.is_alphanumeric() || c.is_whitespace() || ".,!?'-()".contains(c)) {
                return Err(SystematicsError::StructureValidation {
                    reason: format!("Term {} contains invalid characters", i + 1),
                });
            }
        }
        
        // Validate terms are all different
        for i in 0..8 {
            for j in (i + 1)..8 {
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
        println!("Type: Octad ({} terms)", Self::TERM_COUNT);
        print!("Terms: ");
        for (i, term) in self.user_term_index.iter().enumerate() {
            if i > 0 { print!(" → "); }
            print!("{}", term);
        }
        println!();
        
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

/// Builder for creating Octad instances
pub struct OctadBuilder {
    name: String,
    terms: [String; 8],
    attributes: Vec<String>,
    connectives: Option<HashMap<(usize, usize), String>>,
}

impl OctadBuilder {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            terms: [
                String::new(), String::new(), String::new(), String::new(),
                String::new(), String::new(), String::new(), String::new()
            ],
            attributes: Vec::new(),
            connectives: None,
        }
    }
    
    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }
    
    pub fn terms(mut self, t1: String, t2: String, t3: String, t4: String, t5: String, t6: String, t7: String, t8: String) -> Self {
        self.terms = [t1, t2, t3, t4, t5, t6, t7, t8];
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
    
    pub fn build(self) -> Result<Octad> {
        let octad = Octad {
            id: Uuid::new_v4().to_string(),
            name: self.name,
            user_term_index: self.terms,
            attributes: self.attributes,
            connectives: self.connectives.unwrap_or_else(HashMap::new),
            schema: OctadSchema,
        };
        
        // Validate the built octad
        octad.validate()?;
        
        Ok(octad)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_octad_creation() {
        let octad = OctadBuilder::new()
            .name("Test Octad".to_string())
            .terms(
                "T1".to_string(), "T2".to_string(), "T3".to_string(), "T4".to_string(),
                "T5".to_string(), "T6".to_string(), "T7".to_string(), "T8".to_string()
            )
            .build()
            .unwrap();
        
        assert_eq!(octad.name(), "Test Octad");
        assert_eq!(octad.get_term(0), Some("T1"));
        assert_eq!(octad.get_term(7), Some("T8"));
    }

    #[test]
    fn test_canonical_terms() {
        let octad = OctadBuilder::new()
            .name("Test".to_string())
            .terms("A".to_string(), "B".to_string(), "C".to_string(), "D".to_string(), "E".to_string(), "F".to_string(), "G".to_string(), "H".to_string())
            .build()
            .unwrap();
        
        let canonical = octad.canonical_terms();
        assert_eq!(canonical.len(), 8);
        assert_eq!(canonical[0], "Smallest Significant Holon");
        assert_eq!(canonical[1], "Critical Functions");
    }

    #[test]
    fn test_trait_compliance() {
        let octad = OctadBuilder::new()
            .name("Test".to_string())
            .terms("A".to_string(), "B".to_string(), "C".to_string(), "D".to_string(), "E".to_string(), "F".to_string(), "G".to_string(), "H".to_string())
            .build()
            .unwrap();
        
        assert_eq!(Octad::TERM_COUNT, 8);
        assert!(!octad.id().is_empty());
        assert_eq!(octad.name(), "Test");
        assert_eq!(octad.user_terms().len(), 8);
        assert!(octad.validate().is_ok());
    }
} 