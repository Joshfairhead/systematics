use std::collections::HashMap;
use uuid::Uuid;
use crate::{SystematicStructure, error::{Result, SystematicsError}};
use systematics_library::{System, HeptadicSystem};

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
    instances: [String; 7],
    
    // User-defined attributes
    attributes: Vec<String>,
    
    // Connective relationships between terms (from_index, to_index) -> relationship
    connectives: HashMap<(usize, usize), String>,
    
    // Schema reference
    system: HeptadicSystem,
}

impl Heptad {
    /// Create a new heptad with default empty terms
    pub fn new(name: String) -> Self {
        let connectives = HashMap::new();
        
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            instances: [
                String::new(), String::new(), String::new(), String::new(),
                String::new(), String::new(), String::new()
            ],
            attributes: Vec::new(),
            connectives,
            system: HeptadicSystem,
        }
    }
    
    /// Get user instance by index (0-6)
    /// 
    /// Returns the user-provided instance for the given position index.
    /// This is the user-provided data, not the canonical term character from the system.
    /// 
    /// # Arguments
    /// * `index` - Position index (0-6 for heptad)
    /// 
    /// # Returns
    /// * `Some(&str)` - The user instance at the given position
    /// * `None` - If the index is out of bounds
    pub fn get_user_instance(&self, index: usize) -> Option<&str> {
        self.instances.get(index).map(|s| s.as_str())
    }
    
    /// Get the first user instance (maps to "Insight")
    pub fn first_user_instance(&self) -> &str {
        &self.instances[0]
    }
    
    /// Get the second user instance (maps to "Research")
    pub fn second_user_instance(&self) -> &str {
        &self.instances[1]
    }
    
    /// Get the third user instance (maps to "Design")
    pub fn third_user_instance(&self) -> &str {
        &self.instances[2]
    }
    
    /// Get the fourth user instance (maps to "Synthesis")
    pub fn fourth_user_instance(&self) -> &str {
        &self.instances[3]
    }
    
    /// Get the fifth user instance (maps to "Application")
    pub fn fifth_user_instance(&self) -> &str {
        &self.instances[4]
    }
    
    /// Get the sixth user instance (maps to "Delivery")
    pub fn sixth_user_instance(&self) -> &str {
        &self.instances[5]
    }
    
    /// Get the seventh user instance (maps to "Value")
    pub fn seventh_user_instance(&self) -> &str {
        &self.instances[6]
    }

    /// Get all user instances as a tuple
    pub fn instances_tuple(&self) -> (&str, &str, &str, &str, &str, &str, &str) {
        (&self.instances[0], &self.instances[1], &self.instances[2], 
         &self.instances[3], &self.instances[4], &self.instances[5], 
         &self.instances[6])
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
    
    /// Get the number of positional coordinates in this structure
    pub fn position_count(&self) -> usize {
        7
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

impl SystematicStructure for Heptad {
    const TERM_COUNT: usize = 7;
    
    fn id(&self) -> &str {
        &self.id
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn structure_type(&self) -> &str {
        "heptad"
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
                reason: "Heptad name cannot be empty".to_string(),
            });
        }
        
        // Validate all seven terms are not empty
        let term_names = ["Insight", "Research", "Design", "Synthesis", "Application", "Delivery", "Value"];
        for (i, term) in self.instances.iter().enumerate() {
            if term.trim().is_empty() {
                return Err(SystematicsError::StructureValidation {
                    reason: format!("Term {} ({}) cannot be empty", i + 1, term_names[i]),
                });
            }
        }
        
        // Validate term lengths and characters
        for (i, term) in self.instances.iter().enumerate() {
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
                if self.instances[i].trim().to_lowercase() == self.instances[j].trim().to_lowercase() {
                    return Err(SystematicsError::StructureValidation {
                        reason: format!("Terms {} and {} should be different to represent distinct aspects", i + 1, j + 1),
                    });
                }
            }
        }
        
        Ok(())
    }
    
    fn display(&self) {
        let header = "=== Heptadic Structure ===";
        println!("\n{}", header);
        println!("Name: {}", self.name());
        println!("{}:", self.term_designation());
        println!("  - {}", self.instances[0]);
        println!("  - {}", self.instances[1]);
        println!("  - {}", self.instances[2]);
        println!("  - {}", self.instances[3]);
        println!("  - {}", self.instances[4]);
        println!("  - {}", self.instances[5]);
        println!("  - {}", self.instances[6]);
        
        if !self.attributes.is_empty() {
            println!("Attributes: {}", self.attributes.join(", "));
        }
        
        if !self.connectives.is_empty() {
            println!("Connectives:");
            for ((from, to), relationship) in &self.connectives {
                println!("  {} → {}: {}", 
                    self.instances[*from], 
                    self.instances[*to], 
                    relationship);
            }
        }
        
        println!();
        println!("Metadata");
        println!("ID: {}", &self.id[..8]); // Short ID for readability
        println!("{}", "=".repeat(header.len()));
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
            instances: self.terms,
            attributes: self.attributes,
            connectives: self.connectives.unwrap_or_else(HashMap::new),
            system: HeptadicSystem,
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
        assert_eq!(heptad.instance_from_position(0), Some("Insight1"));
        assert_eq!(heptad.instance_from_position(6), Some("Value1"));
    }

    #[test]
    fn test_term_characters() {
        let heptad = HeptadBuilder::new()
            .name("Test".to_string())
            .terms("A".to_string(), "B".to_string(), "C".to_string(), "D".to_string(), "E".to_string(), "F".to_string(), "G".to_string())
            .build()
            .unwrap();
        
        let characters = heptad.term_characters();
        assert_eq!(characters.len(), 7);
        assert_eq!(characters[0], "Insight");
        assert_eq!(characters[1], "Research");
        assert_eq!(characters[6], "Value");
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
        assert_eq!(heptad.user_instance_index().len(), 7);
        assert!(heptad.validate().is_ok());
    }
} 