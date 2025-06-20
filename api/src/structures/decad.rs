use std::collections::HashMap;
use uuid::Uuid;
use crate::{SystematicStructure, error::{Result, SystematicsError}};
use systematics_library::{System, DecadicSystem};

/// Decad: Ten-term systematic structure representing decadic relationships
/// 
/// Based on Bennett's systematic structures, the decad represents ten-fold
/// relationships representing completion and systematic wholeness
#[derive(Debug, Clone)]
pub struct Decad {
    // Unique identifier for this structure instance
    id: String,
    
    // User-defined name for this structure
    name: String,
    
    // User's terms for each index position (decad has 10 term indices)
    user_instance_index: [String; 10],
    
    // User-defined attributes
    attributes: Vec<String>,
    
    // Connective relationships between terms (from_index, to_index) -> relationship
    connectives: HashMap<(usize, usize), String>,
    
    // Schema reference
    system: DecadicSystem,
}

impl Decad {
    /// Create a new decad with default empty terms
    pub fn new(name: String) -> Self {
        let connectives = HashMap::new();
        
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            user_instance_index: [
                String::new(), String::new(), String::new(), String::new(),
                String::new(), String::new(), String::new(), String::new(),
                String::new(), String::new()
            ],
            attributes: Vec::new(),
            connectives,
            system: DecadicSystem,
        }
    }
    
    /// Get user instance by index (0-9)
    /// 
    /// Returns the user-provided instance for the given position index.
    /// This is the user-provided data, not the canonical term character from the system.
    /// 
    /// # Arguments
    /// * `index` - Position index (0-9 for decad)
    /// 
    /// # Returns
    /// * `Some(&str)` - The user instance at the given position
    /// * `None` - If the index is out of bounds
    pub fn get_user_instance(&self, index: usize) -> Option<&str> {
        self.user_instance_index.get(index).map(|s| s.as_str())
    }
    
    /// Get all user instances as a tuple
    pub fn instances_tuple(&self) -> (&str, &str, &str, &str, &str, &str, &str, &str, &str, &str) {
        (&self.user_instance_index[0], &self.user_instance_index[1], &self.user_instance_index[2], 
         &self.user_instance_index[3], &self.user_instance_index[4], &self.user_instance_index[5], 
         &self.user_instance_index[6], &self.user_instance_index[7], &self.user_instance_index[8], 
         &self.user_instance_index[9])
    }
    
    /// Add an attribute to the decad
    pub fn add_attribute(&mut self, attribute: String) {
        if !self.attributes.contains(&attribute) {
            self.attributes.push(attribute);
        }
    }
    
    /// Remove an attribute from the decad
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
        if from_index < 10 && to_index < 10 && from_index != to_index {
            self.connectives.insert((from_index, to_index), relationship);
        }
    }
    
    /// Get all connectives
    pub fn connectives(&self) -> &HashMap<(usize, usize), String> {
        &self.connectives
    }
    
    /// Get the number of positional coordinates in this structure
    pub fn position_count(&self) -> usize {
        10
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
        self.user_instance_index.iter().position(|inst| inst == instance)
    }
    
    /// Map a positional coordinate to its user instance
    /// Returns the user instance for the given 0-based position index
    pub fn instance_from_position(&self, position: usize) -> Option<&str> {
        self.user_instance_index.get(position).map(|s| s.as_str())
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

impl SystematicStructure for Decad {
    const TERM_COUNT: usize = 10;
    
    fn id(&self) -> &str {
        &self.id
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn structure_type(&self) -> &str {
        "decad"
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
    
    fn user_instance_index(&self) -> &[String] {
        &self.user_instance_index
    }
    
    fn first_order_connectives_type(&self) -> &str {
        self.system.first_order_connectives_type()
    }
    
    fn connectives_traits(&self) -> &std::collections::HashMap<(usize, usize), String> {
        &self.connectives
    }
    
    fn system(&self) -> &dyn systematics_library::System {
        &self.system
    }
    
    fn validate(&self) -> Result<()> {
        // Validate name is not empty
        if self.name.trim().is_empty() {
            return Err(SystematicsError::StructureValidation {
                reason: "Decad name cannot be empty".to_string(),
            });
        }
        
        // Validate all ten terms are not empty
        for (i, term) in self.user_instance_index.iter().enumerate() {
            if term.trim().is_empty() {
                return Err(SystematicsError::StructureValidation {
                    reason: format!("Term at position {} cannot be empty", i + 1),
                });
            }
        }
        
        // Check for duplicate terms
        for i in 0..10 {
            for j in (i + 1)..10 {
                if !self.user_instance_index[i].trim().is_empty() && 
                   self.user_instance_index[i].trim() == self.user_instance_index[j].trim() {
                    return Err(SystematicsError::StructureValidation {
                        reason: format!("Duplicate term '{}' found at positions {} and {}", 
                                       self.user_instance_index[i].trim(), i + 1, j + 1),
                    });
                }
            }
        }
        
        Ok(())
    }
    
    fn display(&self) {
        println!("=== Decad: {} ===", self.name);
        println!("ID: {}", self.id);
        println!("Structure Type: {}", self.structure_type());
        println!("Coherence Attribute: {}", self.coherence_attribute());
        println!("Term Designation: {}", self.term_designation());
        
        println!("\n--- Canonical Terms ---");
        for (i, term_char) in self.term_characters().iter().enumerate() {
            println!("Position {}: {}", i + 1, term_char);
        }
        
        println!("\n--- User Terms ---");
        for (i, user_term) in self.user_instance_index.iter().enumerate() {
            if !user_term.is_empty() {
                println!("Position {}: {}", i + 1, user_term);
            }
        }
        
        if !self.attributes.is_empty() {
            println!("\n--- Attributes ---");
            for attr in &self.attributes {
                println!("• {}", attr);
            }
        }
        
        if !self.connectives.is_empty() {
            println!("\n--- {} ---", self.first_order_connectives_type());
            for ((from, to), relationship) in &self.connectives {
                println!("{} → {}: {}", from + 1, to + 1, relationship);
            }
        }
    }
}

/// Builder for creating Decad instances with validation
pub struct DecadBuilder {
    name: String,
    terms: [String; 10],
    attributes: Vec<String>,
    connectives: Option<HashMap<(usize, usize), String>>,
}

impl DecadBuilder {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            terms: [
                String::new(), String::new(), String::new(), String::new(),
                String::new(), String::new(), String::new(), String::new(),
                String::new(), String::new()
            ],
            attributes: Vec::new(),
            connectives: None,
        }
    }
    
    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }
    
    pub fn terms(mut self, terms: [String; 10]) -> Self {
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
    
    pub fn build(self) -> Result<Decad> {
        let mut decad = Decad {
            id: Uuid::new_v4().to_string(),
            name: self.name,
            user_instance_index: self.terms,
            attributes: self.attributes,
            connectives: self.connectives.unwrap_or_default(),
            system: DecadicSystem,
        };
        
        decad.validate()?;
        Ok(decad)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decad_creation() {
        let decad = Decad::new("Test Decad".to_string());
        assert_eq!(decad.name(), "Test Decad");
        assert_eq!(decad.structure_type(), "decad");
        assert_eq!(decad.position_count(), 10);
        assert_eq!(decad.coherence_attribute(), "Completion");
        assert_eq!(decad.term_designation(), "Principles");
        assert_eq!(decad.first_order_connectives_type(), "Progressions");
    }

    #[test]
    fn test_term_characters() {
        let decad = Decad::new("Test".to_string());
        let term_chars = decad.term_characters();
        assert_eq!(term_chars.len(), 10);
        assert_eq!(term_chars[0], "Unity");
        assert_eq!(term_chars[9], "Perfection");
    }

    #[test]
    fn test_trait_compliance() {
        let decad = Decad::new("Test".to_string());
        assert_eq!(Decad::TERM_COUNT, 10);
        
        // Test SystematicStructure trait methods
        assert_eq!(decad.user_instance_index().len(), 10);
    }
} 