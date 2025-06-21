use std::collections::HashMap;
use uuid::Uuid;
use crate::{SystematicStructure, error::{Result, SystematicsError}};
use systematics_library::{System, TriadicSystem};

/// A triadic structure representing the fundamental three-fold pattern in Bennett's systematic framework.
/// 
/// The triad represents the first structure of genuine complexity, embodying the dynamic interaction
/// between Will, Function, and Being. This structure maintains the three-layer semantic approach:
/// 
/// 1. **Positional Coordinates**: Semantic indexes (0, 1, 2) 
/// 2. **Canonical Terms**: From schema ("Will", "Function", "Being")
/// 3. **User Instances**: User-provided terms that map to canonical terms
#[derive(Debug, Clone)]
pub struct Triad {
    // Core identity
    id: String,
    name: String,
    
    // User's instances for each positional coordinate (triad has 3 positions)
    user_expressions: [String; 3],
    
    // Connective relationships between instances (from_index, to_index) -> relationship
    connectives: HashMap<(usize, usize), String>,
    
    // System definition
    system: TriadicSystem,
}

impl Triad {
    /// Create a new triad with the given name and user expressions
    pub fn new(name: String, first_instance: String, second_instance: String, third_instance: String) -> Self {
        let connectives = HashMap::new();
        
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            user_expressions: [first_instance, second_instance, third_instance],
            connectives,
            system: TriadicSystem,
        }
    }
    
    // -------------------------------------------------------------------------
    // Content Access Methods
    // -------------------------------------------------------------------------
    
    /// Get the first user expression (maps to "Will")
    pub fn first_instance(&self) -> &str {
        &self.user_expressions[0]
    }
    
    /// Get the second user expression (maps to "Being")
    pub fn second_instance(&self) -> &str {
        &self.user_expressions[1]
    }
    
    /// Get the third user expression (maps to "Function")
    pub fn third_instance(&self) -> &str {
        &self.user_expressions[2]
    }
    
    /// Get all user expressions as a tuple
    pub fn instances_tuple(&self) -> (&str, &str, &str) {
        (&self.user_expressions[0], &self.user_expressions[1], &self.user_expressions[2])
    }
    
    /// Get the number of positional coordinates in this structure
    pub fn position_count(&self) -> usize {
        3
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
    
    /// Map a user expression to its positional coordinate
    /// Returns the 0-based index for the given user expression
    pub fn instance_to_position(&self, instance: &str) -> Option<usize> {
        self.user_expressions.iter().position(|inst| inst == instance)
    }
    
    /// Map a positional coordinate to its user expression
    /// Returns the user expression for the given 0-based position index
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
    /// Returns the user expression for the given 0-based position index
    pub fn position_to_user_term(&self, position: usize) -> Option<&str> {
        self.instance_from_position(position)
    }
    
    /// Map a user term to its position (alias for instance_to_position)
    /// Returns the 0-based index for the given user expression
    pub fn position_from_user_term(&self, user_term: &str) -> Option<usize> {
        self.instance_to_position(user_term)
    }
    
    /// Get connective relationship between two instances
    pub fn get_connective(&self, from_index: usize, to_index: usize) -> Option<&String> {
        self.connectives.get(&(from_index, to_index))
    }
    
    /// Set connective relationship between two instances
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
    
    fn definition_type(&self) -> &str {
        "triad"
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
        // Validate name
        if self.name.trim().is_empty() {
            return Err(SystematicsError::StructureValidation {
                reason: "Triad name cannot be empty".to_string(),
            });
        }
        
        // Validate all three instances are not empty
        let canonical_names = ["Will", "Being", "Function"];
        for (i, instance) in self.user_expressions.iter().enumerate() {
            if instance.trim().is_empty() {
                return Err(SystematicsError::StructureValidation {
                    reason: format!("Instance {} ({}) cannot be empty", i + 1, canonical_names[i]),
                });
            }
        }
        
        // Validate instance lengths
        for (i, instance) in self.user_expressions.iter().enumerate() {
            if instance.len() > 100 {
                return Err(SystematicsError::StructureValidation {
                    reason: format!("Instance {} is too long (max 100 characters)", i + 1),
                });
            }
        }
        
        // Validate instances contain only allowed characters
        for (i, instance) in self.user_expressions.iter().enumerate() {
            if !instance.chars().all(|c| c.is_alphanumeric() || c.is_whitespace() || ".,!?'-()".contains(c)) {
                return Err(SystematicsError::StructureValidation {
                    reason: format!("Instance {} contains invalid characters", i + 1),
                });
            }
        }
        
        // Validate instances are all different
        for i in 0..3 {
            for j in (i + 1)..3 {
                if self.user_expressions[i].trim().to_lowercase() == self.user_expressions[j].trim().to_lowercase() {
                    return Err(SystematicsError::StructureValidation {
                        reason: format!("Instances {} and {} should be different to represent distinct aspects", i + 1, j + 1),
                    });
                }
            }
        }
        
        Ok(())
    }
    
    fn display(&self) {
        let header = "=== Triadic Structure ===";
        println!("\n{}", header);
        println!("Name: {}", self.name());
        println!("Impulses:");
        println!("  - {}", self.first_instance());
        println!("  - {}", self.second_instance());
        println!("  - {}", self.third_instance());
        
        // Show key relationships if they exist
        if !self.connectives.is_empty() {
            println!("Connectives:");
            let mut shown_pairs = std::collections::HashSet::new();
            let mut display_items = Vec::new();
            
            // Collect all unique pairs first
            for ((from, to), relationship) in &self.connectives {
                let pair = if from < to { (*from, *to) } else { (*to, *from) };
                if !shown_pairs.contains(&pair) {
                    shown_pairs.insert(pair);
                    let left_term = &self.user_expressions[pair.0];
                    let right_term = &self.user_expressions[pair.1];
                    display_items.push((left_term, relationship, right_term));
                }
            }
            
            // Calculate column widths
            let max_left_len = display_items.iter().map(|(left, _, _)| left.len()).max().unwrap_or(0);
            let max_rel_len = display_items.iter().map(|(_, rel, _)| rel.len()).max().unwrap_or(0);
            
            // Display with proper column alignment
            for (left_term, relationship, right_term) in display_items {
                println!("  {:>left_width$} <--{:^rel_width$}--> {}", 
                    left_term,
                    relationship,
                    right_term,
                    left_width = max_left_len,
                    rel_width = max_rel_len);
            }
        }
        
        println!();
        println!("Metadata");
        println!("ID: {}", &self.id[..8]); // Short ID for readability
        println!("{}", "=".repeat(header.len()));
    }
    

}

/// Builder for creating Triad structures with improved naming
pub struct TriadBuilder {
    name: Option<String>,
    first_instance: Option<String>,
    second_instance: Option<String>,
    third_instance: Option<String>,
    connectives: Option<HashMap<(usize, usize), String>>,
}

impl TriadBuilder {
    pub fn new() -> Self {
        Self {
            name: None,
            first_instance: None,
            second_instance: None,
            third_instance: None,
            connectives: None,
        }
    }
    
    /// Set the name for the triad
    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }
    
    /// Set the first instance (maps to "Will")
    pub fn first_instance<S: Into<String>>(mut self, instance: S) -> Self {
        self.first_instance = Some(instance.into());
        self
    }
    
    /// Set the second instance (maps to "Being") 
    pub fn second_instance<S: Into<String>>(mut self, instance: S) -> Self {
        self.second_instance = Some(instance.into());
        self
    }
    
    /// Set the third instance (maps to "Function")
    pub fn third_instance<S: Into<String>>(mut self, instance: S) -> Self {
        self.third_instance = Some(instance.into());
        self
    }
    
    /// Set all instances at once
    pub fn user_expressions<S1: Into<String>, S2: Into<String>, S3: Into<String>>(
        mut self, 
        first: S1, 
        second: S2, 
        third: S3
    ) -> Self {
        self.first_instance = Some(first.into());
        self.second_instance = Some(second.into());
        self.third_instance = Some(third.into());
        self
    }
    
    /// Set custom connectives
    pub fn connectives(mut self, connectives: HashMap<(usize, usize), String>) -> Self {
        self.connectives = Some(connectives);
        self
    }
    
    /// Build the triad
    pub fn build(self) -> Result<Triad> {
        let name = self.name.unwrap_or_else(|| "Unnamed Triad".to_string());
        let first_instance = self.first_instance.ok_or_else(|| SystematicsError::Builder {
            reason: "Triad requires a first instance".to_string(),
        })?;
        let second_instance = self.second_instance.ok_or_else(|| SystematicsError::Builder {
            reason: "Triad requires a second instance".to_string(),
        })?;
        let third_instance = self.third_instance.ok_or_else(|| SystematicsError::Builder {
            reason: "Triad requires a third instance".to_string(),
        })?;
        
        let triad = Triad::new(name, first_instance, second_instance, third_instance);
        
        if let Some(_custom_connectives) = self.connectives {
            // Custom connectives would be applied here if needed
        }
        
        triad.validate()?;
        Ok(triad)
    }
}

impl Default for TriadBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triad_creation() {
        let triad = TriadBuilder::new()
            .name("Test Triad")
            .user_expressions("Will", "Being", "Function")
            .build()
            .unwrap();
        
        assert_eq!(triad.name(), "Test Triad");
        assert_eq!(triad.first_instance(), "Will");
        assert_eq!(triad.second_instance(), "Being");
        assert_eq!(triad.third_instance(), "Function");
        assert_eq!(triad.user_expressions_tuple(), ("Will", "Being", "Function"));
        assert!(triad.validate().is_ok());
    }

    #[test]
    fn test_term_characters() {
        let triad = TriadBuilder::new()
            .name("Test Triad")
            .user_expressions("A", "B", "C")
            .build()
            .unwrap();
        
        let characters = triad.term_characters();
        assert_eq!(characters, vec!["Will", "Being", "Function"]);
    }

    #[test]
    fn test_positional_coordinate_mapping() {
        let triad = TriadBuilder::new()
            .name("Test")
            .user_expressions("MyWill", "MyBeing", "MyFunction")
            .build()
            .unwrap();
        
        // Test term character to position mapping
        assert_eq!(triad.term_character_to_position("Will"), Some(0));
        assert_eq!(triad.term_character_to_position("Being"), Some(1));
        assert_eq!(triad.term_character_to_position("Function"), Some(2));
        assert_eq!(triad.term_character_to_position("Invalid"), None);
        
        // Test position to term character mapping
        assert_eq!(triad.term_character_from_position(0), Some("Will"));
        assert_eq!(triad.term_character_from_position(1), Some("Being"));
        assert_eq!(triad.term_character_from_position(2), Some("Function"));
        assert_eq!(triad.term_character_from_position(3), None);
        
        // Test position count
        assert_eq!(triad.position_count(), 3);
        
        // Test user expression to position mapping
        assert_eq!(triad.instance_to_position("MyWill"), Some(0));
        assert_eq!(triad.instance_to_position("MyBeing"), Some(1));
        assert_eq!(triad.instance_to_position("MyFunction"), Some(2));
        assert_eq!(triad.instance_to_position("Invalid"), None);
        
        // Test position to user expression mapping
        assert_eq!(triad.instance_from_position(0), Some("MyWill"));
        assert_eq!(triad.instance_from_position(1), Some("MyBeing"));
        assert_eq!(triad.instance_from_position(2), Some("MyFunction"));
        assert_eq!(triad.instance_from_position(3), None);
    }

    #[test]
    fn test_connective_relationships() {
        let triad = TriadBuilder::new()
            .name("Test Triad")
            .user_expressions("Will", "Function", "Being")
            .build()
            .unwrap();
        
        // Test connectives access (currently empty)
        assert_eq!(triad.connectives().len(), 0);
    }

    #[test]
    fn test_triad_validation() {
        // Valid triad
        let valid_triad = TriadBuilder::new()
            .name("Valid")
            .user_expressions("A", "B", "C")
            .build();
        assert!(valid_triad.is_ok());
        
        // Empty instance should fail
        let invalid_triad = TriadBuilder::new()
            .name("Invalid")
            .user_expressions("A", "", "C")
            .build();
        assert!(invalid_triad.is_err());
        
        // Duplicate instances should fail
        let duplicate_triad = TriadBuilder::new()
            .name("Duplicate")
            .user_expressions("Same", "Same", "Different")
            .build();
        assert!(duplicate_triad.is_err());
    }

    #[test]
    fn test_triad_instances_method() {
        let triad = TriadBuilder::new()
            .name("Test")
            .user_expressions("One", "Two", "Three")
            .build()
            .unwrap();
        
        let (first, second, third) = triad.user_expressions_tuple();
        assert_eq!(first, "One");
        assert_eq!(second, "Two");
        assert_eq!(third, "Three");
    }

    #[test]
    fn test_trait_compliance() {
        let triad = TriadBuilder::new()
            .name("Test")
            .user_expressions("A", "B", "C")
            .build()
            .unwrap();
        
        // Test SystematicStructure trait methods
        assert_eq!(Triad::TERM_COUNT, 3);
        assert!(!triad.id().is_empty());
        assert_eq!(triad.name(), "Test");
        assert_eq!(triad.user_expressions().len(), 3);
        assert_eq!(triad.user_expressions()[0], "A");
        assert_eq!(triad.user_expressions()[1], "B");
        assert_eq!(triad.user_expressions()[2], "C");
        assert_eq!(triad.term_characters(), vec!["Will", "Being", "Function"]);
        assert!(triad.validate().is_ok());
    }
    
    #[test]
    fn test_position_alias_methods() {
        let triad = TriadBuilder::new()
            .name("Test")
            .user_expressions("MyWill", "MyBeing", "MyFunction")
            .build()
            .unwrap();
        
        // Test term character position aliases
        assert_eq!(triad.position_to_term_character(0), Some("Will"));
        assert_eq!(triad.position_to_term_character(1), Some("Being"));
        assert_eq!(triad.position_to_term_character(2), Some("Function"));
        assert_eq!(triad.position_to_term_character(3), None);
        
        assert_eq!(triad.position_from_term_character("Will"), Some(0));
        assert_eq!(triad.position_from_term_character("Being"), Some(1));
        assert_eq!(triad.position_from_term_character("Function"), Some(2));
        assert_eq!(triad.position_from_term_character("NonExistent"), None);
        
        // Test user term position aliases
        assert_eq!(triad.position_to_user_term(0), Some("MyWill"));
        assert_eq!(triad.position_to_user_term(1), Some("MyBeing"));
        assert_eq!(triad.position_to_user_term(2), Some("MyFunction"));
        assert_eq!(triad.position_to_user_term(3), None);
        
        assert_eq!(triad.position_from_user_term("MyWill"), Some(0));
        assert_eq!(triad.position_from_user_term("MyBeing"), Some(1));
        assert_eq!(triad.position_from_user_term("MyFunction"), Some(2));
        assert_eq!(triad.position_from_user_term("Unknown"), None);
        
        // Verify aliases return same results as original methods
        assert_eq!(triad.position_to_term_character(0), triad.term_character_from_position(0));
        assert_eq!(triad.position_from_term_character("Will"), triad.term_character_to_position("Will"));
        assert_eq!(triad.position_to_user_term(0), triad.instance_from_position(0));
        assert_eq!(triad.position_from_user_term("MyWill"), triad.instance_to_position("MyWill"));
    }
} 