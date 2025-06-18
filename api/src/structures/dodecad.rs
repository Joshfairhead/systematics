use std::collections::HashMap;
use uuid::Uuid;
use crate::{SystematicStructure, error::{Result, SystematicsError}};
use systematics_library::{System, DodecadicSystem};

/// Dodecad: Twelve-term systematic structure representing dodecadic relationships
/// 
/// Based on Bennett's systematic structures, the dodecad represents twelve-fold
/// relationships representing the complete spectrum of systematic wholeness
#[derive(Debug, Clone)]
pub struct Dodecad {
    // Unique identifier for this structure instance
    id: String,
    
    // User-defined name for this structure
    name: String,
    
    // User's terms for each index position (dodecad has 12 term indices)
    user_term_index: [String; 12],
    
    // User-defined attributes
    attributes: Vec<String>,
    
    // Connective relationships between terms (from_index, to_index) -> relationship
    connectives: HashMap<(usize, usize), String>,
    
    // Schema reference
    system: DodecadicSystem,
}

impl Dodecad {
    /// Create a new dodecad with default empty terms
    pub fn new(name: String) -> Self {
        let connectives = HashMap::new();
        
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            user_term_index: [
                String::new(), String::new(), String::new(), String::new(),
                String::new(), String::new(), String::new(), String::new(),
                String::new(), String::new(), String::new(), String::new()
            ],
            attributes: Vec::new(),
            connectives,
            system: DodecadicSystem,
        }
    }
    
    /// Get term by index (0-11)
    pub fn get_term(&self, index: usize) -> Option<&str> {
        self.user_term_index.get(index).map(|s| s.as_str())
    }
    
    /// Add an attribute to the dodecad
    pub fn add_attribute(&mut self, attribute: String) {
        if !self.attributes.contains(&attribute) {
            self.attributes.push(attribute);
        }
    }
    
    /// Remove an attribute from the dodecad
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
        if from_index < 12 && to_index < 12 && from_index != to_index {
            self.connectives.insert((from_index, to_index), relationship);
        }
    }
    
    /// Get all connectives
    pub fn connectives(&self) -> &HashMap<(usize, usize), String> {
        &self.connectives
    }
    
    /// Get the number of positional coordinates in this structure
    pub fn position_count(&self) -> usize {
        12
    }
    
    /// Map a term character to its positional coordinate
    /// Returns the 0-based index for the given term character
    pub fn term_character_to_position(&self, term_character: &str) -> Option<usize> {
        let term_characters = self.system.term_characters();
        term_characters.iter().position(|&term| term == term_character)
    }
    
    /// Map a positional coordinate to its term character
    /// Returns the term character for the given 0-based position index
    pub fn term_character_from_position(&self, position: usize) -> Option<&str> {
        let term_characters = self.system.term_characters();
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

impl SystematicStructure for Dodecad {
    const TERM_COUNT: usize = 12;
    
    fn id(&self) -> &str {
        &self.id
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn structure_type(&self) -> &str {
        "dodecad"
    }
    
    fn terms(&self) -> &[String] {
        &self.user_term_index
    }
    
    fn coherence_attribute(&self) -> &str {
        self.system.coherence_attribute()
    }
    
    fn term_designation(&self) -> &str {
        self.system.term_designation()
    }
    
    fn source(&self) -> &str {
        self.system.source()
    }
    
    fn term_characters(&self) -> Vec<String> {
        self.system.term_characters().iter().map(|s| s.to_string()).collect()
    }
    
    fn user_terms(&self) -> &[String] {
        &self.user_term_index
    }
    
    fn first_order_connectives_name(&self) -> &str {
        self.system.first_order_connectives_name()
    }
    
    fn connectives(&self) -> &std::collections::HashMap<(usize, usize), String> {
        &self.connectives
    }
    
    fn system(&self) -> &dyn systematics_library::System {
        &self.system
    }
    
    fn validate(&self) -> Result<()> {
        // Validate name is not empty
        if self.name.trim().is_empty() {
            return Err(SystematicsError::StructureValidation {
                reason: "Dodecad name cannot be empty".to_string(),
            });
        }
        
        // Validate all twelve terms are not empty
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
        for i in 0..12 {
            for j in (i + 1)..12 {
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
        let header = "=== Dodecadic Structure ===";
        println!("\n{}", header);
        println!("Name: {}", self.name());
        println!("{}:", self.term_designation());
        println!("  - {}", self.user_term_index[0]);
        println!("  - {}", self.user_term_index[1]);
        println!("  - {}", self.user_term_index[2]);
        println!("  - {}", self.user_term_index[3]);
        println!("  - {}", self.user_term_index[4]);
        println!("  - {}", self.user_term_index[5]);
        println!("  - {}", self.user_term_index[6]);
        println!("  - {}", self.user_term_index[7]);
        println!("  - {}", self.user_term_index[8]);
        println!("  - {}", self.user_term_index[9]);
        println!("  - {}", self.user_term_index[10]);
        println!("  - {}", self.user_term_index[11]);
        
        if !self.attributes.is_empty() {
            println!("Attributes: {}", self.attributes.join(", "));
        }
        
        if !self.connectives.is_empty() {
            println!("Connectives: {} relationships defined", self.connectives.len());
            // Only show first few connectives due to volume (66 total possible)
            let mut count = 0;
            for ((from, to), relationship) in &self.connectives {
                if count < 5 {
                    println!("  {} → {}: {}", 
                        self.user_term_index[*from], 
                        self.user_term_index[*to], 
                        relationship);
                    count += 1;
                } else {
                    println!("  ... and {} more connectives", self.connectives.len() - 5);
                    break;
                }
            }
        }
        
        println!();
        println!("Metadata");
        println!("ID: {}", &self.id[..8]); // Short ID for readability
        println!("{}", "=".repeat(header.len()));
    }
}

/// Builder for creating Dodecad instances
pub struct DodecadBuilder {
    name: String,
    terms: [String; 12],
    attributes: Vec<String>,
    connectives: Option<HashMap<(usize, usize), String>>,
}

impl DodecadBuilder {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            terms: [
                String::new(), String::new(), String::new(), String::new(),
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
    
    pub fn terms(mut self, terms: [String; 12]) -> Self {
        self.terms = terms;
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
    
    pub fn build(self) -> Result<Dodecad> {
        let dodecad = Dodecad {
            id: Uuid::new_v4().to_string(),
            name: self.name,
            user_term_index: self.terms,
            attributes: self.attributes,
            connectives: self.connectives.unwrap_or_else(HashMap::new),
            system: DodecadicSystem,
        };
        
        // Validate the built dodecad
        dodecad.validate()?;
        
        Ok(dodecad)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dodecad_creation() {
        let terms = [
            "T1".to_string(), "T2".to_string(), "T3".to_string(), "T4".to_string(),
            "T5".to_string(), "T6".to_string(), "T7".to_string(), "T8".to_string(),
            "T9".to_string(), "T10".to_string(), "T11".to_string(), "T12".to_string()
        ];
        
        let dodecad = DodecadBuilder::new()
            .name("Test Dodecad".to_string())
            .terms(terms)
            .build()
            .unwrap();
        
        assert_eq!(dodecad.name(), "Test Dodecad");
        assert_eq!(dodecad.get_term(0), Some("T1"));
        assert_eq!(dodecad.get_term(11), Some("T12"));
    }

    #[test]
    fn test_term_characters() {
        let terms = [
            "A".to_string(), "B".to_string(), "C".to_string(), "D".to_string(),
            "E".to_string(), "F".to_string(), "G".to_string(), "H".to_string(),
            "I".to_string(), "J".to_string(), "K".to_string(), "L".to_string()
        ];
        
        let dodecad = DodecadBuilder::new()
            .name("Test".to_string())
            .terms(terms)
            .build()
            .unwrap();
        
        let characters = dodecad.term_characters();
        assert_eq!(characters.len(), 12);
        assert_eq!(characters[0], "Autocracy");
        assert_eq!(characters[1], "Domination");
        assert_eq!(characters[11], "Wholeness");
    }

    #[test]
    fn test_trait_compliance() {
        let terms = [
            "A".to_string(), "B".to_string(), "C".to_string(), "D".to_string(),
            "E".to_string(), "F".to_string(), "G".to_string(), "H".to_string(),
            "I".to_string(), "J".to_string(), "K".to_string(), "L".to_string()
        ];
        
        let dodecad = DodecadBuilder::new()
            .name("Test".to_string())
            .terms(terms)
            .build()
            .unwrap();
        
        assert_eq!(Dodecad::TERM_COUNT, 12);
        assert!(!dodecad.id().is_empty());
        assert_eq!(dodecad.name(), "Test");
        assert_eq!(dodecad.user_terms().len(), 12);
        assert!(dodecad.validate().is_ok());
    }
} 