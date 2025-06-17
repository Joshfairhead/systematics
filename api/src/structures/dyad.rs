// Placeholder for Dyad structure - will be implemented similar to Monad
use crate::{SystematicStructure, error::{Result, SystematicsError}};
use systematics_library::{System, DyadicSystem};
use uuid::Uuid;
use std::collections::HashMap;

// =============================================================================
// Dyad Structure Definition
// =============================================================================

/// A dyadic structure representing the fundamental duality in Bennett's systematic framework.
/// 
/// The dyad represents the first level of differentiation from unity, embodying the 
/// essential polarity between Essence and Existence. This structure maintains the three-layer
/// semantic approach:
/// 
/// 1. **Positional Coordinates**: Semantic indexes (0, 1) 
/// 2. **Canonical Terms**: From schema ("Essence", "Existence")
/// 3. **User Instances**: User-provided terms that map to canonical terms
#[derive(Debug, Clone)]
pub struct Dyad {
    // Core identity
    id: String,
    name: String,
    
    // User's instances for each positional coordinate (dyad has 2 positions)  
    user_instances: [String; 2],
    

    
    // Connective relationships (consistent with larger structures)
    connectives: HashMap<(usize, usize), String>,
    
    // System definition
    system: DyadicSystem,
}

// =============================================================================
// Core Implementation
// =============================================================================

impl Dyad {
    /// Create a new dyad with the given name and user instances
    pub fn new(name: String, first_instance: String, second_instance: String) -> Self {
        let connectives = HashMap::new();
        // TODO: Add proper Bennett framework connective relationships
        // connectives.insert((0, 1), "manifests as".to_string());
        
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            user_instances: [first_instance, second_instance],
            connectives,
            system: DyadicSystem,
        }
    }
    
    // -------------------------------------------------------------------------
    // Content Access Methods
    // -------------------------------------------------------------------------
    
    /// Get the first user instance (maps to "Essence")
    pub fn first_instance(&self) -> &str {
        &self.user_instances[0]
    }
    
    /// Get the second user instance (maps to "Existence")
    pub fn second_instance(&self) -> &str {
        &self.user_instances[1]
    }
    
    /// Get both user instances as a tuple
    pub fn instances_tuple(&self) -> (&str, &str) {
        (&self.user_instances[0], &self.user_instances[1])
    }
    
    /// Get the number of positional coordinates in this structure
    pub fn position_count(&self) -> usize {
        2
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
        self.user_instances.iter().position(|inst| inst == instance)
    }
    
    /// Map a positional coordinate to its user instance
    /// Returns the user instance for the given 0-based position index
    pub fn instance_from_position(&self, position: usize) -> Option<&str> {
        self.user_instances.get(position).map(|s| s.as_str())
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
    
    /// Get the connective relationship
    pub fn connective(&self) -> Option<&String> {
        self.connectives.get(&(0, 1))
    }
    
    /// Get a specific connective relationship between two positions
    pub fn get_connective(&self, from_index: usize, to_index: usize) -> Option<&String> {
        self.connectives.get(&(from_index, to_index))
    }
    
    /// Set a connective relationship between two positions
    pub fn set_connective(&mut self, from_index: usize, to_index: usize, relationship: String) {
        if from_index < 2 && to_index < 2 && from_index != to_index {
            self.connectives.insert((from_index, to_index), relationship);
        }
    }
    
    /// Get all connectives
    pub fn connectives(&self) -> &HashMap<(usize, usize), String> {
        &self.connectives
    }
}

// =============================================================================
// SystematicStructure Trait Implementation
// =============================================================================

impl SystematicStructure for Dyad {
    const TERM_COUNT: usize = 2;
    
    // -------------------------------------------------------------------------
    // Core Identity & Structure
    // -------------------------------------------------------------------------
    
    fn id(&self) -> &str {
        &self.id
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn structure_type(&self) -> &str {
        "dyad"
    }
    
    fn terms(&self) -> &[String] {
        &self.user_instances
    }
    
    fn coherence_attribute(&self) -> &str {
        self.system.coherence_attribute()
    }
    
    fn term_designation(&self) -> &str {
        self.system.term_designation()
    }
    
    // -------------------------------------------------------------------------
    // Content Access
    // -------------------------------------------------------------------------
    
    fn term_characters(&self) -> Vec<String> {
        self.system.term_characters().iter().map(|s| s.to_string()).collect()
    }
    
    fn user_terms(&self) -> &[String] {
        &self.user_instances
    }
    
    fn first_order_connectives_name(&self) -> &str {
        self.system.first_order_connectives_name()
    }
    
    fn connectives(&self) -> &std::collections::HashMap<(usize, usize), String> {
        &self.connectives
    }
    
    // -------------------------------------------------------------------------
    // Schema & Structure
    // -------------------------------------------------------------------------
    
    fn system(&self) -> &dyn systematics_library::System {
        &self.system
    }
    
    // -------------------------------------------------------------------------
    // Validation & Integrity
    // -------------------------------------------------------------------------
    
    fn validate(&self) -> Result<()> {
        // Validate name
        if self.name.trim().is_empty() {
            return Err(SystematicsError::StructureValidation {
                reason: "Dyad name cannot be empty".to_string(),
            });
        }
        
        // Validate first instance is not empty
        if self.user_instances[0].trim().is_empty() {
            return Err(SystematicsError::StructureValidation {
                reason: "First instance (Essence) cannot be empty".to_string(),
            });
        }
        
        // Validate second instance is not empty
        if self.user_instances[1].trim().is_empty() {
            return Err(SystematicsError::StructureValidation {
                reason: "Second instance (Existence) cannot be empty".to_string(),
            });
        }
        
        // Validate instance lengths
        for (i, instance) in self.user_instances.iter().enumerate() {
            if instance.len() > 100 {
                return Err(SystematicsError::StructureValidation {
                    reason: format!("Instance {} is too long (max 100 characters)", i + 1),
                });
            }
        }
        
        // Validate instances contain only allowed characters
        for (i, instance) in self.user_instances.iter().enumerate() {
            if !instance.chars().all(|c| c.is_alphanumeric() || c.is_whitespace() || ".,!?'-()".contains(c)) {
                return Err(SystematicsError::StructureValidation {
                    reason: format!("Instance {} contains invalid characters", i + 1),
                });
            }
        }
        
        // Validate instances are different (dyad should represent duality)
        if self.user_instances[0].trim().to_lowercase() == self.user_instances[1].trim().to_lowercase() {
            return Err(SystematicsError::StructureValidation {
                reason: "Dyad instances should be different to represent duality".to_string(),
            });
        }
        
        Ok(())
    }
    
    // -------------------------------------------------------------------------
    // Display & Output
    // -------------------------------------------------------------------------
    
    fn display(&self) {
        let header = "=== Dyadic Structure ===";
        println!("\n{}", header);
        println!("Name: {}", self.name());
        println!("Poles: {} ↔ {}", self.first_instance(), self.second_instance());
        
        // Show connectives if they exist
        if !self.connectives.is_empty() {
            println!("Connectives:");
            let mut shown_pairs = std::collections::HashSet::new();
            let mut display_items = Vec::new();
            
            // Collect all unique pairs first
            for ((from, to), relationship) in &self.connectives {
                let pair = if from < to { (*from, *to) } else { (*to, *from) };
                if !shown_pairs.contains(&pair) {
                    shown_pairs.insert(pair);
                    let left_term = &self.user_instances[pair.0];
                    let right_term = &self.user_instances[pair.1];
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
    
    // -------------------------------------------------------------------------
    // Serialization (Optional Feature)
    // -------------------------------------------------------------------------
    
    #[cfg(feature = "serde_support")]
    fn to_json(&self) -> Result<String> {
        serde_json::to_string(self).map_err(|e| SystematicsError::Serialization(e.to_string()))
    }
    
    #[cfg(feature = "serde_support")]
    fn from_json(json: &str) -> Result<Self> {
        serde_json::from_str(json).map_err(|e| SystematicsError::Deserialization(e.to_string()))
    }
}

// =============================================================================
// Builder Pattern
// =============================================================================

/// Builder for creating Dyad structures with improved naming
pub struct DyadBuilder {
    name: Option<String>,
    first_instance: Option<String>,
    second_instance: Option<String>,
}

impl DyadBuilder {
    pub fn new() -> Self {
        Self {
            name: None,
            first_instance: None,
            second_instance: None,
        }
    }
    
    /// Set the dyad name
    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }
    
    /// Set the first instance (maps to "Essence")
    pub fn first_instance<S: Into<String>>(mut self, instance: S) -> Self {
        self.first_instance = Some(instance.into());
        self
    }
    
    /// Set the second instance (maps to "Existence")
    pub fn second_instance<S: Into<String>>(mut self, instance: S) -> Self {
        self.second_instance = Some(instance.into());
        self
    }
    
    /// Set both instances at once
    pub fn instances<S1: Into<String>, S2: Into<String>>(mut self, first: S1, second: S2) -> Self {
        self.first_instance = Some(first.into());
        self.second_instance = Some(second.into());
        self
    }
    
    /// Legacy method for backward compatibility - maps to instances()
    pub fn terms<S1: Into<String>, S2: Into<String>>(self, first: S1, second: S2) -> Self {
        self.instances(first, second)
    }

    /// Build the dyad structure
    pub fn build(self) -> Result<Dyad> {
        let name = self.name.unwrap_or_else(|| "Unnamed Dyad".to_string());
        
        let first_instance = self.first_instance.ok_or_else(|| SystematicsError::Builder {
            reason: "Dyad requires a first instance (Essence)".to_string(),
        })?;
        
        let second_instance = self.second_instance.ok_or_else(|| SystematicsError::Builder {
            reason: "Dyad requires a second instance (Existence)".to_string(),
        })?;
        
        let dyad = Dyad::new(name, first_instance, second_instance);
            
        dyad.validate()?;
        Ok(dyad)
    }
}

impl Default for DyadBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_dyad_creation() {
        let dyad = DyadBuilder::new()
            .name("Test Dyad")
            .instances("Spirit", "Matter")
            .build()
            .unwrap();
            
        assert_eq!(dyad.name(), "Test Dyad");
        assert_eq!(dyad.first_instance(), "Spirit");
        assert_eq!(dyad.second_instance(), "Matter");
        assert!(dyad.validate().is_ok());
    }
    
    #[test]
    fn test_dyad_instances_method() {
        let dyad = DyadBuilder::new()
            .name("Test")
            .instances("Essence", "Existence")
            .build()
            .unwrap();
            
        let (first, second) = dyad.instances_tuple();
        assert_eq!(first, "Essence");
        assert_eq!(second, "Existence");
    }
    
    #[test]
    fn test_positional_coordinate_mapping() {
        let dyad = DyadBuilder::new()
            .name("Test")
            .instances("MyEssence", "MyExistence")
            .build()
            .unwrap();
        
        // Test term character to position mapping
        assert_eq!(dyad.term_character_to_position("Essence"), Some(0));
        assert_eq!(dyad.term_character_to_position("Existence"), Some(1));
        assert_eq!(dyad.term_character_to_position("Invalid"), None);
        
        // Test position to term character mapping
        assert_eq!(dyad.term_character_from_position(0), Some("Essence"));
        assert_eq!(dyad.term_character_from_position(1), Some("Existence"));
        assert_eq!(dyad.term_character_from_position(2), None);
        
        // Test position count
        assert_eq!(dyad.position_count(), 2);
        
        // Test user instance to position mapping
        assert_eq!(dyad.instance_to_position("MyEssence"), Some(0));
        assert_eq!(dyad.instance_to_position("MyExistence"), Some(1));
        assert_eq!(dyad.instance_to_position("Invalid"), None);
        
        // Test position to user instance mapping
        assert_eq!(dyad.instance_from_position(0), Some("MyEssence"));
        assert_eq!(dyad.instance_from_position(1), Some("MyExistence"));
        assert_eq!(dyad.instance_from_position(2), None);
    }
    
    #[test]
    fn test_dyad_validation() {
        // Test missing first instance
        let result = DyadBuilder::new()
            .name("Invalid Dyad")
            .second_instance("Existence")
            .build();
        assert!(result.is_err());
        
        // Test missing second instance
        let result = DyadBuilder::new()
            .name("Invalid Dyad")
            .first_instance("Essence")
            .build();
        assert!(result.is_err());
        
        // Test identical instances (should represent duality)
        let result = DyadBuilder::new()
            .name("Invalid Dyad")
            .instances("Same", "Same")
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_term_characters() {
        let dyad = DyadBuilder::new()
            .name("Test")
            .instances("Spirit", "Matter")
            .build()
            .unwrap();
            
        let characters = dyad.term_characters();
        assert_eq!(characters, vec!["Essence", "Existence"]);
    }
    
    #[test]
    fn test_connective_relationship() {
        let dyad = DyadBuilder::new()
            .name("Test")
            .instances("Spirit", "Matter")
            .build()
            .unwrap();
            
        // No default connective (commented out fabricated relationships)
        assert_eq!(dyad.connective(), None);
    }
    
    #[test]
    fn test_trait_compliance() {
        let dyad = DyadBuilder::new()
            .name("Test")
            .instances("Essence", "Existence")
            .build()
            .unwrap();
            
        // Test SystematicStructure trait methods
        assert_eq!(Dyad::TERM_COUNT, 2);
        assert!(!dyad.id().is_empty());
        assert_eq!(dyad.name(), "Test");
        assert_eq!(dyad.user_terms().len(), 2);
        assert_eq!(dyad.user_terms()[0], "Essence");
        assert_eq!(dyad.user_terms()[1], "Existence");
        assert_eq!(dyad.term_characters(), vec!["Essence", "Existence"]);
        assert!(dyad.validate().is_ok());
    }
    
    #[test]
    fn test_legacy_terms_method() {
        // Test backward compatibility with terms() method
        let dyad = DyadBuilder::new()
            .name("Test")
            .terms("Spirit", "Matter")
            .build()
            .unwrap();
            
        assert_eq!(dyad.first_instance(), "Spirit");
        assert_eq!(dyad.second_instance(), "Matter");
    }
    
    #[test]
    fn test_position_alias_methods() {
        let dyad = DyadBuilder::new()
            .name("Test")
            .instances("Spirit", "Matter")
            .build()
            .unwrap();
        
        // Test term character position aliases
        assert_eq!(dyad.position_to_term_character(0), Some("Essence"));
        assert_eq!(dyad.position_to_term_character(1), Some("Existence"));
        assert_eq!(dyad.position_to_term_character(2), None);
        
        assert_eq!(dyad.position_from_term_character("Essence"), Some(0));
        assert_eq!(dyad.position_from_term_character("Existence"), Some(1));
        assert_eq!(dyad.position_from_term_character("NonExistent"), None);
        
        // Test user term position aliases
        assert_eq!(dyad.position_to_user_term(0), Some("Spirit"));
        assert_eq!(dyad.position_to_user_term(1), Some("Matter"));
        assert_eq!(dyad.position_to_user_term(2), None);
        
        assert_eq!(dyad.position_from_user_term("Spirit"), Some(0));
        assert_eq!(dyad.position_from_user_term("Matter"), Some(1));
        assert_eq!(dyad.position_from_user_term("Unknown"), None);
        
        // Verify aliases return same results as original methods
        assert_eq!(dyad.position_to_term_character(0), dyad.term_character_from_position(0));
        assert_eq!(dyad.position_from_term_character("Essence"), dyad.term_character_to_position("Essence"));
        assert_eq!(dyad.position_to_user_term(0), dyad.instance_from_position(0));
        assert_eq!(dyad.position_from_user_term("Spirit"), dyad.instance_to_position("Spirit"));
    }
} 