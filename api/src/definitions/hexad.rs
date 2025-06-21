use std::collections::HashMap;
use uuid::Uuid;
use crate::{SystematicStructure, error::{Result, SystematicsError}};
use systematics_library::{System, HexadicSystem};

/// Hexad: Six-term systematic structure representing hexadic relationships
/// 
/// Based on Bennett's systematic structures, the hexad represents six-fold
/// relationships with canonical terms: Resources, Values, Options, Criteria, Facts, Priorities
#[derive(Debug, Clone)]
pub struct Hexad {
    // Unique identifier for this structure instance
    id: String,
    
    // User-defined name for this structure
    name: String,
    
    // User's terms for each index position (hexad has 6 term indices)
    instances: [String; 6],
    

    
    // Connective relationships between terms (from_index, to_index) -> relationship
    connectives: HashMap<(usize, usize), String>,
    
    // Schema reference
    system: HexadicSystem,
}

impl Hexad {
    /// Create a new hexad with default empty terms
    pub fn new(name: String) -> Self {
        let connectives = HashMap::new();
        
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            instances: [String::new(), String::new(), String::new(), String::new(), String::new(), String::new()],
            connectives,
            system: HexadicSystem,
        }
    }
    
    /// Get user instance by index (0-5)
    /// 
    /// Returns the user-provided instance for the given position index.
    /// This is the user-provided data, not the canonical term character from the system.
    /// 
    /// # Arguments
    /// * `index` - Position index (0-5 for hexad)
    /// 
    /// # Returns
    /// * `Some(&str)` - The user instance at the given position
    /// * `None` - If the index is out of bounds
    pub fn get_user_instance(&self, index: usize) -> Option<&str> {
        self.instances.get(index).map(|s| s.as_str())
    }
    
    /// Get the first user instance (maps to "Resources")
    pub fn first_user_instance(&self) -> &str {
        &self.instances[0]
    }
    
    /// Get the second user instance (maps to "Values")
    pub fn second_user_instance(&self) -> &str {
        &self.instances[1]
    }
    
    /// Get the third user instance (maps to "Options")
    pub fn third_user_instance(&self) -> &str {
        &self.instances[2]
    }
    
    /// Get the fourth user instance (maps to "Criteria")
    pub fn fourth_user_instance(&self) -> &str {
        &self.instances[3]
    }
    
    /// Get the fifth user instance (maps to "Facts")
    pub fn fifth_user_instance(&self) -> &str {
        &self.instances[4]
    }
    
    /// Get the sixth user instance (maps to "Priorities")
    pub fn sixth_user_instance(&self) -> &str {
        &self.instances[5]
    }

    /// Get all user instances as a tuple
    pub fn instances_tuple(&self) -> (&str, &str, &str, &str, &str, &str) {
        (&self.instances[0], &self.instances[1], &self.instances[2], 
         &self.instances[3], &self.instances[4], &self.instances[5])
    }
    

    
    /// Get connective relationship between two terms
    pub fn get_connective(&self, from_index: usize, to_index: usize) -> Option<&String> {
        self.connectives.get(&(from_index, to_index))
    }
    
    /// Set connective relationship between two terms
    pub fn set_connective(&mut self, from_index: usize, to_index: usize, relationship: String) {
        if from_index < 6 && to_index < 6 && from_index != to_index {
            self.connectives.insert((from_index, to_index), relationship);
        }
    }
    
    /// Get all connectives
    pub fn connectives(&self) -> &HashMap<(usize, usize), String> {
        &self.connectives
    }
    
    /// Get the number of positional coordinates in this structure
    pub fn position_count(&self) -> usize {
        6
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

impl SystematicStructure for Hexad {
    const TERM_COUNT: usize = 6;
    
    fn id(&self) -> &str {
        &self.id
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn definition_type(&self) -> &str {
        "hexad"
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
                reason: "Hexad name cannot be empty".to_string(),
            });
        }
        
        // Validate all six terms are not empty
        let term_names = ["Resources", "Values", "Options", "Criteria", "Facts", "Priorities"];
        for (i, term) in self.instances.iter().enumerate() {
            if term.trim().is_empty() {
                return Err(SystematicsError::StructureValidation {
                    reason: format!("Term {} ({}) cannot be empty", i + 1, term_names[i]),
                });
            }
        }
        
        // Validate term lengths
        for (i, term) in self.instances.iter().enumerate() {
            if term.len() > 100 {
                return Err(SystematicsError::StructureValidation {
                    reason: format!("Term {} is too long (max 100 characters)", i + 1),
                });
            }
        }
        
        // Validate terms contain only allowed characters
        for (i, term) in self.instances.iter().enumerate() {
            if !term.chars().all(|c| c.is_alphanumeric() || c.is_whitespace() || ".,!?'-()".contains(c)) {
                return Err(SystematicsError::StructureValidation {
                    reason: format!("Term {} contains invalid characters", i + 1),
                });
            }
        }
        
        // Validate terms are all different
        for i in 0..6 {
            for j in (i + 1)..6 {
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
        let header = "=== Hexadic Structure ===";
        println!("\n{}", header);
        println!("Name: {}", self.name());
        println!("{}:", self.term_designation());
        println!("  - {}", self.instances[0]);
        println!("  - {}", self.instances[1]);
        println!("  - {}", self.instances[2]);
        println!("  - {}", self.instances[3]);
        println!("  - {}", self.instances[4]);
        println!("  - {}", self.instances[5]);
        
        // Note: Connectives not displayed in CLI for systems beyond pentad due to complexity
        if !self.connectives.is_empty() {
            println!("{}: {} relationships defined", self.first_order_connectives_type(), self.connectives.len());
        }
        
        println!();
        println!("Metadata");
        println!("ID: {}", &self.id[..8]); // Short ID for readability
        println!("{}", "=".repeat(header.len()));
    }
}

/// Builder for creating Hexad instances
pub struct HexadBuilder {
    name: Option<String>,
    terms: Option<[String; 6]>,
    connectives: Option<HashMap<(usize, usize), String>>,
}

impl HexadBuilder {
    pub fn new() -> Self {
        Self {
            name: None,
            terms: None,
            connectives: None,
        }
    }
    
    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }
    
    pub fn terms<S1: Into<String>, S2: Into<String>, S3: Into<String>, S4: Into<String>, S5: Into<String>, S6: Into<String>>(
        mut self, 
        t1: S1, t2: S2, t3: S3, t4: S4, t5: S5, t6: S6
    ) -> Self {
        self.terms = Some([t1.into(), t2.into(), t3.into(), t4.into(), t5.into(), t6.into()]);
        self
    }
    
    pub fn connectives(mut self, connectives: HashMap<(usize, usize), String>) -> Self {
        self.connectives = Some(connectives);
        self
    }
    
    pub fn build(self) -> Result<Hexad> {
        let name = self.name.unwrap_or_else(|| "Unnamed Hexad".to_string());
        let terms = self.terms.ok_or_else(|| SystematicsError::Builder {
            reason: "Hexad requires 6 terms".to_string(),
        })?;
        
        let hexad = Hexad {
            id: Uuid::new_v4().to_string(),
            name,
            instances: terms,
            connectives: self.connectives.unwrap_or_else(HashMap::new),
            system: HexadicSystem,
        };
        
        // Validate the built hexad
        hexad.validate()?;
        
        Ok(hexad)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hexad_creation() {
        let hexad = HexadBuilder::new()
            .name("Test Hexad".to_string())
            .terms(
                "Resource1".to_string(), "Value1".to_string(), "Option1".to_string(),
                "Criteria1".to_string(), "Fact1".to_string(), "Priority1".to_string()
            )
            .build()
            .unwrap();
        
        assert_eq!(hexad.name(), "Test Hexad");
        assert_eq!(hexad.instance_from_position(0), Some("Resource1"));
        assert_eq!(hexad.instance_from_position(5), Some("Priority1"));
    }

    #[test]
    fn test_term_characters() {
        let hexad = HexadBuilder::new()
            .name("Test".to_string())
            .terms("A".to_string(), "B".to_string(), "C".to_string(), "D".to_string(), "E".to_string(), "F".to_string())
            .build()
            .unwrap();
        
        let characters = hexad.term_characters();
        assert_eq!(characters.len(), 6);
        assert_eq!(characters[0], "Resources");
        assert_eq!(characters[1], "Values");
        assert_eq!(characters[2], "Options");
        assert_eq!(characters[3], "Criteria");
        assert_eq!(characters[4], "Facts");
        assert_eq!(characters[5], "Priorities");
    }

    #[test]
    fn test_trait_compliance() {
        let hexad = HexadBuilder::new()
            .name("Test".to_string())
            .terms("A".to_string(), "B".to_string(), "C".to_string(), "D".to_string(), "E".to_string(), "F".to_string())
            .build()
            .unwrap();
        
        assert_eq!(Hexad::TERM_COUNT, 6);
        assert!(!hexad.id().is_empty());
        assert_eq!(hexad.name(), "Test");
        assert_eq!(hexad.user_instance_index().len(), 6);
        assert!(hexad.validate().is_ok());
    }
} 