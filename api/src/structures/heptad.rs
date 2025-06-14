use std::collections::HashMap;
use uuid::Uuid;
use crate::{SystematicStructure, schemas::{Schema, HeptadSchema}, error::{Result, SystematicsError}};

/// Heptad: Seven-term systematic structure representing heptadic relationships
/// 
/// Based on Bennett's systematic structures, the heptad represents seven-fold
/// relationships with canonical terms: Insight, Research, Design, Synthesis, Application, Delivery, Value
#[derive(Debug, Clone)]
pub struct Heptad {
    // Unique identifier for this structure instance
    id: String,
    
    // User-defined name for this structure
    name: String,
    
    // User's terms for each index position (heptad has 7 term indices)
    user_term_index: [String; 7],
    
    // User-defined attributes
    attributes: Vec<String>,
    
    // Connective relationships between terms (from_index, to_index) -> relationship
    connectives: HashMap<(usize, usize), String>,
    
    // Schema reference
    schema: HeptadSchema,
}

impl Heptad {
    /// Create a new heptad with default empty terms
    pub fn new(name: String) -> Self {
        let connectives = HashMap::new();
        
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            user_term_index: [
                String::new(), String::new(), String::new(), String::new(),
                String::new(), String::new(), String::new()
            ],
            attributes: Vec::new(),
            connectives,
            schema: HeptadSchema,
        }
    }
    
    /// Get term by index (0-6)
    pub fn get_term(&self, index: usize) -> Option<&str> {
        self.user_term_index.get(index).map(|s| s.as_str())
    }
    
    /// Add an attribute to the heptad
    pub fn add_attribute(&mut self, attribute: String) {
        if !self.attributes.contains(&attribute) {
            self.attributes.push(attribute);
        }
    }
    
    /// Remove an attribute from the heptad
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
        if from_index < 7 && to_index < 7 && from_index != to_index {
            self.connectives.insert((from_index, to_index), relationship);
        }
    }
    
    /// Get all connectives
    pub fn connectives(&self) -> &HashMap<(usize, usize), String> {
        &self.connectives
    }
}

impl SystematicStructure for Heptad {
    const TERM_COUNT: usize = 7;
    
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
                reason: "Heptad name cannot be empty".to_string(),
            });
        }
        
        // Validate all seven terms are not empty
        let term_names = ["Insight", "Research", "Design", "Synthesis", "Application", "Delivery", "Value"];
        for (i, term) in self.user_term_index.iter().enumerate() {
            if term.trim().is_empty() {
                return Err(SystematicsError::StructureValidation {
                    reason: format!("Term {} ({}) cannot be empty", i + 1, term_names[i]),
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
        for i in 0..7 {
            for j in (i + 1)..7 {
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
        println!("Type: Heptad ({} terms)", Self::TERM_COUNT);
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

/// Builder for creating Heptad instances
pub struct HeptadBuilder {
    name: String,
    terms: [String; 7],
    attributes: Vec<String>,
    connectives: Option<HashMap<(usize, usize), String>>,
}

impl HeptadBuilder {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            terms: [
                String::new(), String::new(), String::new(), String::new(),
                String::new(), String::new(), String::new()
            ],
            attributes: Vec::new(),
            connectives: None,
        }
    }
    
    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }
    
    pub fn terms(mut self, t1: String, t2: String, t3: String, t4: String, t5: String, t6: String, t7: String) -> Self {
        self.terms = [t1, t2, t3, t4, t5, t6, t7];
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
    
    pub fn build(self) -> Result<Heptad> {
        let heptad = Heptad {
            id: Uuid::new_v4().to_string(),
            name: self.name,
            user_term_index: self.terms,
            attributes: self.attributes,
            connectives: self.connectives.unwrap_or_else(HashMap::new),
            schema: HeptadSchema,
        };
        
        // Validate the built heptad
        heptad.validate()?;
        
        Ok(heptad)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heptad_creation() {
        let heptad = HeptadBuilder::new()
            .name("Test Heptad".to_string())
            .terms(
                "Insight1".to_string(), "Research1".to_string(), "Design1".to_string(),
                "Synthesis1".to_string(), "Application1".to_string(), "Delivery1".to_string(), "Value1".to_string()
            )
            .build()
            .unwrap();
        
        assert_eq!(heptad.name(), "Test Heptad");
        assert_eq!(heptad.get_term(0), Some("Insight1"));
        assert_eq!(heptad.get_term(6), Some("Value1"));
    }

    #[test]
    fn test_canonical_terms() {
        let heptad = HeptadBuilder::new()
            .name("Test".to_string())
            .terms("A".to_string(), "B".to_string(), "C".to_string(), "D".to_string(), "E".to_string(), "F".to_string(), "G".to_string())
            .build()
            .unwrap();
        
        let canonical = heptad.canonical_terms();
        assert_eq!(canonical.len(), 7);
        assert_eq!(canonical[0], "Insight");
        assert_eq!(canonical[1], "Research");
        assert_eq!(canonical[6], "Value");
    }

    #[test]
    fn test_trait_compliance() {
        let heptad = HeptadBuilder::new()
            .name("Test".to_string())
            .terms("A".to_string(), "B".to_string(), "C".to_string(), "D".to_string(), "E".to_string(), "F".to_string(), "G".to_string())
            .build()
            .unwrap();
        
        assert_eq!(Heptad::TERM_COUNT, 7);
        assert!(!heptad.id().is_empty());
        assert_eq!(heptad.name(), "Test");
        assert_eq!(heptad.user_terms().len(), 7);
        assert!(heptad.validate().is_ok());
    }
} 