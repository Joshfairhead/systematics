// Placeholder for Dyad structure - will be implemented similar to Monad
use crate::{
    SystematicStructure, 
    schemas::{Schema, DyadSchema}, 
    error::{Result, SystematicsError}
};
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
    
    // Schema definition
    schema: DyadSchema,
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
            schema: DyadSchema,
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
    
    /// Map a canonical term to its positional coordinate
    /// Returns the 0-based index for the given canonical term
    pub fn canonical_term_to_position(&self, canonical_term: &str) -> Option<usize> {
        let canonical_terms = self.schema.canonical_terms();
        canonical_terms.iter().position(|&term| term == canonical_term)
    }
    
    /// Map a positional coordinate to its canonical term
    /// Returns the canonical term for the given 0-based position index
    pub fn canonical_term_from_position(&self, position: usize) -> Option<&str> {
        let canonical_terms = self.schema.canonical_terms();
        canonical_terms.get(position).copied()
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
    
    /// Map a position to its canonical term (alias for canonical_term_from_position)
    /// Returns the canonical term for the given 0-based position index
    pub fn position_to_canonical_term(&self, position: usize) -> Option<&str> {
        self.canonical_term_from_position(position)
    }
    
    /// Map a canonical term to its position (alias for canonical_term_to_position)
    /// Returns the 0-based index for the given canonical term
    pub fn position_from_canonical_term(&self, canonical_term: &str) -> Option<usize> {
        self.canonical_term_to_position(canonical_term)
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
    
    // -------------------------------------------------------------------------
    // Content Access
    // -------------------------------------------------------------------------
    
    fn canonical_terms(&self) -> Vec<String> {
        self.schema.canonical_terms().iter().map(|s| s.to_string()).collect()
    }
    
    fn user_terms(&self) -> &[String] {
        &self.user_instances
    }
    
    // -------------------------------------------------------------------------
    // Schema & Structure
    // -------------------------------------------------------------------------
    
    fn schema(&self) -> &dyn Schema {
        &self.schema
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
        println!("\n=== {} ===", self.name);
        println!("Type: Dyad (2 positions)");
        println!("Instances: {} ↔ {}", self.first_instance(), self.second_instance());
        if let Some(connective) = self.connective() {
            println!("Relationship: {} {} {}", self.first_instance(), connective, self.second_instance());
        }
        

        
        println!("Schema: {}", self.schema.name());
        println!("ID: {}", &self.id[..8]); // Short ID for readability
        println!("{}", "=".repeat(self.name.len() + 8));
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
        
        // Test canonical term to position mapping
        assert_eq!(dyad.canonical_term_to_position("Essence"), Some(0));
        assert_eq!(dyad.canonical_term_to_position("Existence"), Some(1));
        assert_eq!(dyad.canonical_term_to_position("Invalid"), None);
        
        // Test position to canonical term mapping
        assert_eq!(dyad.canonical_term_from_position(0), Some("Essence"));
        assert_eq!(dyad.canonical_term_from_position(1), Some("Existence"));
        assert_eq!(dyad.canonical_term_from_position(2), None);
        
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
    fn test_canonical_terms() {
        let dyad = DyadBuilder::new()
            .name("Test")
            .instances("Spirit", "Matter")
            .build()
            .unwrap();
            
        let canonical = dyad.canonical_terms();
        assert_eq!(canonical, vec!["Essence", "Existence"]);
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
        assert_eq!(dyad.canonical_terms(), vec!["Essence", "Existence"]);
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
        
        // Test canonical term position aliases
        assert_eq!(dyad.position_to_canonical_term(0), Some("Essence"));
        assert_eq!(dyad.position_to_canonical_term(1), Some("Existence"));
        assert_eq!(dyad.position_to_canonical_term(2), None);
        
        assert_eq!(dyad.position_from_canonical_term("Essence"), Some(0));
        assert_eq!(dyad.position_from_canonical_term("Existence"), Some(1));
        assert_eq!(dyad.position_from_canonical_term("NonExistent"), None);
        
        // Test user term position aliases
        assert_eq!(dyad.position_to_user_term(0), Some("Spirit"));
        assert_eq!(dyad.position_to_user_term(1), Some("Matter"));
        assert_eq!(dyad.position_to_user_term(2), None);
        
        assert_eq!(dyad.position_from_user_term("Spirit"), Some(0));
        assert_eq!(dyad.position_from_user_term("Matter"), Some(1));
        assert_eq!(dyad.position_from_user_term("Unknown"), None);
        
        // Verify aliases return same results as original methods
        assert_eq!(dyad.position_to_canonical_term(0), dyad.canonical_term_from_position(0));
        assert_eq!(dyad.position_from_canonical_term("Essence"), dyad.canonical_term_to_position("Essence"));
        assert_eq!(dyad.position_to_user_term(0), dyad.instance_from_position(0));
        assert_eq!(dyad.position_from_user_term("Spirit"), dyad.instance_to_position("Spirit"));
    }
} 