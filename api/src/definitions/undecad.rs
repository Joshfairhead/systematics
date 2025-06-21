use std::collections::HashMap;
use uuid::Uuid;
use crate::{SystematicStructure, error::{Result, SystematicsError}};
use systematics_library::{System, UndecadicSystem};

/// Undecad: Eleven-term systematic structure representing undecadic relationships
/// 
/// Based on Bennett's systematic structures, the undecad represents eleven-fold
/// relationships representing transition and transcendence
#[derive(Debug, Clone)]
pub struct Undecad {
    // Unique identifier for this structure instance
    id: String,
    
    // User-defined name for this structure
    name: String,
    
    // User's terms for each index position (undecad has 11 term indices)
    user_expressions: [String; 11],
    
    // User-defined attributes
    attributes: Vec<String>,
    
    // Connective relationships between terms (from_index, to_index) -> relationship
    connectives: HashMap<(usize, usize), String>,
    
    // Schema reference
    system: UndecadicSystem,
}

impl Undecad {
    /// Create a new undecad with default empty terms
    pub fn new(name: String) -> Self {
        let connectives = HashMap::new();
        
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            user_expressions: [
                String::new(), String::new(), String::new(), String::new(),
                String::new(), String::new(), String::new(), String::new(),
                String::new(), String::new(), String::new()
            ],
            attributes: Vec::new(),
            connectives,
            system: UndecadicSystem,
        }
    }
    
    /// Get user instance by index (0-10)
    /// 
    /// Returns the user-provided instance for the given position index.
    /// This is the user-provided data, not the canonical term character from the system.
    /// 
    /// # Arguments
    /// * `index` - Position index (0-10 for undecad)
    /// 
    /// # Returns
    /// * `Some(&str)` - The user instance at the given position
    /// * `None` - If the index is out of bounds
    pub fn get_user_instance(&self, index: usize) -> Option<&str> {
        self.user_expressions.get(index).map(|s| s.as_str())
    }
    
    /// Get all user instances as a tuple
    pub fn instances_tuple(&self) -> (&str, &str, &str, &str, &str, &str, &str, &str, &str, &str, &str) {
        (&self.user_expressions[0], &self.user_expressions[1], &self.user_expressions[2], 
         &self.user_expressions[3], &self.user_expressions[4], &self.user_expressions[5], 
         &self.user_expressions[6], &self.user_expressions[7], &self.user_expressions[8], 
         &self.user_expressions[9], &self.user_expressions[10])
    }
    
    /// Add an attribute to the undecad
    pub fn add_attribute(&mut self, attribute: String) {
        if !self.attributes.contains(&attribute) {
            self.attributes.push(attribute);
        }
    }
    
    /// Remove an attribute from the undecad
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
        if from_index < 11 && to_index < 11 && from_index != to_index {
            self.connectives.insert((from_index, to_index), relationship);
        }
    }
    
    /// Get all connectives
    pub fn connectives(&self) -> &HashMap<(usize, usize), String> {
        &self.connectives
    }
    
    /// Get the number of positional coordinates in this structure
    pub fn position_count(&self) -> usize {
        11
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
        self.user_expressions.iter().position(|inst| inst == instance)
    }
    
    /// Map a positional coordinate to its user instance
    /// Returns the user instance for the given 0-based position index
    pub fn instance_from_position(&self, position: usize) -> Option<&str> {
        self.user_expressions.get(position).map(|s| s.as_str())
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

impl SystematicStructure for Undecad {
    const TERM_COUNT: usize = 11;
    
    fn id(&self) -> &str {
        &self.id
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn definition_type(&self) -> &str {
        "undecad"
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
    
    fn user_expressions(&self) -> &[String] {
        &self.user_expressions
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
                reason: "Undecad name cannot be empty".to_string(),
            });
        }
        
        // Validate all eleven terms are not empty
        for (i, term) in self.user_expressions.iter().enumerate() {
            if term.trim().is_empty() {
                return Err(SystematicsError::StructureValidation {
                    reason: format!("Term at position {} cannot be empty", i + 1),
                });
            }
        }
        
        // Check for duplicate terms
        for i in 0..11 {
            for j in (i + 1)..11 {
                if !self.user_expressions[i].trim().is_empty() && 
                   self.user_expressions[i].trim() == self.user_expressions[j].trim() {
                    return Err(SystematicsError::StructureValidation {
                        reason: format!("Duplicate term '{}' found at positions {} and {}", 
                                       self.user_expressions[i].trim(), i + 1, j + 1),
                    });
                }
            }
        }
        
        Ok(())
    }
    
    fn display(&self) {
        println!("=== Undecad: {} ===", self.name);
        println!("ID: {}", self.id);
        println!("Structure Type: {}", self.definition_type());
        println!("Coherence Attribute: {}", self.coherence_attribute());
        println!("Term Designation: {}", self.term_designation());
        
        println!("\n--- Canonical Terms ---");
        for (i, term_char) in self.term_characters().iter().enumerate() {
            println!("Position {}: {}", i + 1, term_char);
        }
        
        println!("\n--- User Terms ---");
        for (i, user_term) in self.user_expressions.iter().enumerate() {
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

/// Builder for creating Undecad instances with validation
pub struct UndecadBuilder {
    name: String,
    terms: [String; 11],
    attributes: Vec<String>,
    connectives: Option<HashMap<(usize, usize), String>>,
}

impl UndecadBuilder {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            terms: [
                String::new(), String::new(), String::new(), String::new(),
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
    
    pub fn terms(mut self, terms: [String; 11]) -> Self {
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
    
    pub fn build(self) -> Result<Undecad> {
        let undecad = Undecad {
            id: Uuid::new_v4().to_string(),
            name: self.name,
            user_expressions: self.terms,
            attributes: self.attributes,
            connectives: self.connectives.unwrap_or_default(),
            system: UndecadicSystem,
        };
        
        undecad.validate()?;
        Ok(undecad)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_undecad_creation() {
        let undecad = Undecad::new("Test Undecad".to_string());
        assert_eq!(undecad.name(), "Test Undecad");
        assert_eq!(undecad.definition_type(), "undecad");
        assert_eq!(undecad.position_count(), 11);
        assert_eq!(undecad.coherence_attribute(), "Articulate Symmetry");
        assert_eq!(undecad.term_designation(), "Elements");
        assert_eq!(undecad.first_order_connectives_type(), "Connectives");
    }

    #[test]
    fn test_term_characters() {
        let undecad = Undecad::new("Test".to_string());
        let term_chars = undecad.term_characters();
        assert_eq!(term_chars.len(), 11);
        assert_eq!(term_chars[0], "Term 1");
        assert_eq!(term_chars[10], "Term 11");
    }

    #[test]
    fn test_trait_compliance() {
        let undecad = Undecad::new("Test".to_string());
        assert_eq!(Undecad::TERM_COUNT, 11);
        
        // Test SystematicStructure trait methods
        assert_eq!(undecad.user_expressions().len(), 11);
    }
} 