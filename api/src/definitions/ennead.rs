use std::collections::HashMap;
use uuid::Uuid;
use crate::{SystematicStructure, error::{Result, SystematicsError}};
use systematics_library::{System, EnneadicSystem};

/// Ennead: Nine-term systematic structure representing enneadic relationships
/// 
/// Based on Bennett's systematic structures, the ennead represents nine-fold
/// relationships representing transformation and personality patterns
#[derive(Debug, Clone)]
pub struct Ennead {
    // Unique identifier for this structure instance
    id: String,
    
    // User-defined name for this structure
    name: String,
    
    // User's terms for each index position (ennead has 9 term indices)
    instances: [String; 9],
    
    // User-defined attributes
    attributes: Vec<String>,
    
    // Connective relationships between terms (from_index, to_index) -> relationship
    connectives: HashMap<(usize, usize), String>,
    
    // Schema reference
    system: EnneadicSystem,
}

impl Ennead {
    /// Create a new ennead with default empty terms
    pub fn new(name: String) -> Self {
        let connectives = HashMap::new();
        
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            instances: [
                String::new(), String::new(), String::new(), String::new(),
                String::new(), String::new(), String::new(), String::new(),
                String::new()
            ],
            attributes: Vec::new(),
            connectives,
            system: EnneadicSystem,
        }
    }
    
    /// Get user instance by index (0-8)
    /// 
    /// Returns the user-provided instance for the given position index.
    /// This is the user-provided data, not the canonical term character from the system.
    /// 
    /// # Arguments
    /// * `index` - Position index (0-8 for ennead)
    /// 
    /// # Returns
    /// * `Some(&str)` - The user instance at the given position
    /// * `None` - If the index is out of bounds
    pub fn get_user_instance(&self, index: usize) -> Option<&str> {
        self.instances.get(index).map(|s| s.as_str())
    }
    
    /// Get all user instances as a tuple
    pub fn instances_tuple(&self) -> (&str, &str, &str, &str, &str, &str, &str, &str, &str) {
        (&self.instances[0], &self.instances[1], &self.instances[2], 
         &self.instances[3], &self.instances[4], &self.instances[5], 
         &self.instances[6], &self.instances[7], &self.instances[8])
    }
    
    /// Add an attribute to the ennead
    pub fn add_attribute(&mut self, attribute: String) {
        if !self.attributes.contains(&attribute) {
            self.attributes.push(attribute);
        }
    }
    
    /// Remove an attribute from the ennead
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
        if from_index < 9 && to_index < 9 && from_index != to_index {
            self.connectives.insert((from_index, to_index), relationship);
        }
    }
    
    /// Get all connectives
    pub fn connectives(&self) -> &HashMap<(usize, usize), String> {
        &self.connectives
    }
    
    /// Get the number of positional coordinates in this structure
    pub fn position_count(&self) -> usize {
        9
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
        self.instances.iter().position(|inst| inst == instance)
    }
    
    /// Map a positional coordinate to its user instance
    /// Returns the user instance for the given 0-based position index
    pub fn instance_from_position(&self, position: usize) -> Option<&str> {
        self.instances.get(position).map(|s| s.as_str())
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

impl SystematicStructure for Ennead {
    const TERM_COUNT: usize = 9;
    
    fn id(&self) -> &str {
        &self.id
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn definition_type(&self) -> &str {
        "ennead"
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
        &self.instances
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
                reason: "Ennead name cannot be empty".to_string(),
            });
        }
        
        // Validate all nine terms are not empty
        for (i, term) in self.instances.iter().enumerate() {
            if term.trim().is_empty() {
                return Err(SystematicsError::StructureValidation {
                    reason: format!("Term at position {} cannot be empty", i + 1),
                });
            }
        }
        
        // Check for duplicate terms
        for i in 0..9 {
            for j in (i + 1)..9 {
                if !self.instances[i].trim().is_empty() && 
                   self.instances[i].trim() == self.instances[j].trim() {
                    return Err(SystematicsError::StructureValidation {
                        reason: format!("Duplicate term '{}' found at positions {} and {}", 
                                       self.instances[i].trim(), i + 1, j + 1),
                    });
                }
            }
        }
        
        Ok(())
    }
    
    fn display(&self) {
        println!("=== Ennead: {} ===", self.name);
        println!("ID: {}", self.id);
        println!("Structure Type: {}", self.definition_type());
        println!("Coherence Attribute: {}", self.coherence_attribute());
        println!("Term Designation: {}", self.term_designation());
        
        println!("\n--- Canonical Terms ---");
        for (i, term_char) in self.term_characters().iter().enumerate() {
            println!("Position {}: {}", i + 1, term_char);
        }
        
        println!("\n--- User Terms ---");
        for (i, user_term) in self.instances.iter().enumerate() {
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

/// Builder for creating Ennead instances with validation
pub struct EnneadBuilder {
    name: String,
    terms: [String; 9],
    attributes: Vec<String>,
    connectives: Option<HashMap<(usize, usize), String>>,
}

impl EnneadBuilder {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            terms: [
                String::new(), String::new(), String::new(), String::new(),
                String::new(), String::new(), String::new(), String::new(),
                String::new()
            ],
            attributes: Vec::new(),
            connectives: None,
        }
    }
    
    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }
    
    pub fn terms(mut self, t1: String, t2: String, t3: String, t4: String, t5: String, t6: String, t7: String, t8: String, t9: String) -> Self {
        self.terms = [t1, t2, t3, t4, t5, t6, t7, t8, t9];
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
    
    pub fn build(self) -> Result<Ennead> {
        let ennead = Ennead {
            id: Uuid::new_v4().to_string(),
            name: self.name,
            instances: self.terms,
            attributes: self.attributes,
            connectives: self.connectives.unwrap_or_default(),
            system: EnneadicSystem,
        };
        
        ennead.validate()?;
        Ok(ennead)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ennead_creation() {
        let ennead = Ennead::new("Test Ennead".to_string());
        assert_eq!(ennead.name(), "Test Ennead");
        assert_eq!(ennead.definition_type(), "ennead");
        assert_eq!(ennead.position_count(), 9);
        assert_eq!(ennead.coherence_attribute(), "Transformation");
        assert_eq!(ennead.term_designation(), "Elements");
        assert_eq!(ennead.first_order_connectives_type(), "Connectives");
    }

    #[test]
    fn test_term_characters() {
        let ennead = Ennead::new("Test".to_string());
        let term_chars = ennead.term_characters();
        assert_eq!(term_chars.len(), 9);
        assert_eq!(term_chars[0], "Term 1");
        assert_eq!(term_chars[8], "Term 9");
    }

    #[test]
    fn test_trait_compliance() {
        let ennead = Ennead::new("Test".to_string());
        assert_eq!(Ennead::TERM_COUNT, 9);
        
        // Test SystematicStructure trait methods
        assert_eq!(ennead.user_instance_index().len(), 9);
    }
} 