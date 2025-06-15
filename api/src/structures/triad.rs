use std::collections::HashMap;
use uuid::Uuid;
use crate::{SystematicStructure, schemas::{Schema, TriadSchema}, error::{Result, SystematicsError}};

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
    user_instances: [String; 3],
    
    // Connective relationships between instances (from_index, to_index) -> relationship
    connectives: HashMap<(usize, usize), String>,
    
    // Schema definition
    schema: TriadSchema,
}

impl Triad {
    /// Create a new triad with the given name and user instances
    pub fn new(name: String, first_instance: String, second_instance: String, third_instance: String) -> Self {
        let connectives = HashMap::new();
        
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            user_instances: [first_instance, second_instance, third_instance],
            connectives,
            schema: TriadSchema,
        }
    }
    
    // -------------------------------------------------------------------------
    // Content Access Methods
    // -------------------------------------------------------------------------
    
    /// Get the first user instance (maps to "Will")
    pub fn first_instance(&self) -> &str {
        &self.user_instances[0]
    }
    
    /// Get the second user instance (maps to "Function")
    pub fn second_instance(&self) -> &str {
        &self.user_instances[1]
    }
    
    /// Get the third user instance (maps to "Being")
    pub fn third_instance(&self) -> &str {
        &self.user_instances[2]
    }
    
    /// Get all user instances as a tuple
    pub fn instances_tuple(&self) -> (&str, &str, &str) {
        (&self.user_instances[0], &self.user_instances[1], &self.user_instances[2])
    }
    
    /// Get the number of positional coordinates in this structure
    pub fn position_count(&self) -> usize {
        3
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
    
    fn canonical_terms(&self) -> Vec<String> {
        self.schema.canonical_terms().iter().map(|s| s.to_string()).collect()
    }
    
    fn user_terms(&self) -> &[String] {
        &self.user_instances
    }
    
    fn schema(&self) -> &dyn Schema {
        &self.schema
    }
    
    fn validate(&self) -> Result<()> {
        // Validate name
        if self.name.trim().is_empty() {
            return Err(SystematicsError::StructureValidation {
                reason: "Triad name cannot be empty".to_string(),
            });
        }
        
        // Validate all three instances are not empty
        let canonical_names = ["Will", "Function", "Being"];
        for (i, instance) in self.user_instances.iter().enumerate() {
            if instance.trim().is_empty() {
                return Err(SystematicsError::StructureValidation {
                    reason: format!("Instance {} ({}) cannot be empty", i + 1, canonical_names[i]),
                });
            }
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
        
        // Validate instances are all different
        for i in 0..3 {
            for j in (i + 1)..3 {
                if self.user_instances[i].trim().to_lowercase() == self.user_instances[j].trim().to_lowercase() {
                    return Err(SystematicsError::StructureValidation {
                        reason: format!("Instances {} and {} should be different to represent distinct aspects", i + 1, j + 1),
                    });
                }
            }
        }
        
        Ok(())
    }
    
    fn display(&self) {
        println!("\n=== {} ===", self.name);
        println!("Type: Triad (3 positions)");
        println!("Instances: {} ↔ {} ↔ {}", self.first_instance(), self.second_instance(), self.third_instance());
        
        // Show key relationships
        if !self.connectives.is_empty() {
            println!("Connectives:");
            for ((from, to), relationship) in &self.connectives {
                println!("  {} → {}: {}", 
                    self.user_instances[*from], 
                    self.user_instances[*to], 
                    relationship);
            }
        }
        
        println!("Schema: {}", self.schema.name());
        println!("ID: {}", &self.id[..8]);
        println!("{}", "=".repeat(self.name.len() + 8));
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
    
    /// Set the second instance (maps to "Function") 
    pub fn second_instance<S: Into<String>>(mut self, instance: S) -> Self {
        self.second_instance = Some(instance.into());
        self
    }
    
    /// Set the third instance (maps to "Being")
    pub fn third_instance<S: Into<String>>(mut self, instance: S) -> Self {
        self.third_instance = Some(instance.into());
        self
    }
    
    /// Set all instances at once
    pub fn instances<S1: Into<String>, S2: Into<String>, S3: Into<String>>(
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
    
    /// Legacy method for backward compatibility - maps to instances()
    pub fn terms<S1: Into<String>, S2: Into<String>, S3: Into<String>>(
        self, 
        first: S1, 
        second: S2, 
        third: S3
    ) -> Self {
        self.instances(first, second, third)
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
        
        if let Some(custom_connectives) = self.connectives {
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
            .instances("Will", "Function", "Being")
            .build()
            .unwrap();
        
        assert_eq!(triad.name(), "Test Triad");
        assert_eq!(triad.first_instance(), "Will");
        assert_eq!(triad.second_instance(), "Function");
        assert_eq!(triad.third_instance(), "Being");
        assert_eq!(triad.instances_tuple(), ("Will", "Function", "Being"));
        assert!(triad.validate().is_ok());
    }

    #[test]
    fn test_canonical_terms() {
        let triad = TriadBuilder::new()
            .name("Test Triad")
            .instances("A", "B", "C")
            .build()
            .unwrap();
        
        let canonical = triad.canonical_terms();
        assert_eq!(canonical, vec!["Will", "Function", "Being"]);
    }

    #[test]
    fn test_positional_coordinate_mapping() {
        let triad = TriadBuilder::new()
            .name("Test")
            .instances("MyWill", "MyFunction", "MyBeing")
            .build()
            .unwrap();
        
        // Test canonical term to position mapping
        assert_eq!(triad.canonical_term_to_position("Will"), Some(0));
        assert_eq!(triad.canonical_term_to_position("Function"), Some(1));
        assert_eq!(triad.canonical_term_to_position("Being"), Some(2));
        assert_eq!(triad.canonical_term_to_position("Invalid"), None);
        
        // Test position to canonical term mapping
        assert_eq!(triad.canonical_term_from_position(0), Some("Will"));
        assert_eq!(triad.canonical_term_from_position(1), Some("Function"));
        assert_eq!(triad.canonical_term_from_position(2), Some("Being"));
        assert_eq!(triad.canonical_term_from_position(3), None);
        
        // Test position count
        assert_eq!(triad.position_count(), 3);
        
        // Test user instance to position mapping
        assert_eq!(triad.instance_to_position("MyWill"), Some(0));
        assert_eq!(triad.instance_to_position("MyFunction"), Some(1));
        assert_eq!(triad.instance_to_position("MyBeing"), Some(2));
        assert_eq!(triad.instance_to_position("Invalid"), None);
        
        // Test position to user instance mapping
        assert_eq!(triad.instance_from_position(0), Some("MyWill"));
        assert_eq!(triad.instance_from_position(1), Some("MyFunction"));
        assert_eq!(triad.instance_from_position(2), Some("MyBeing"));
        assert_eq!(triad.instance_from_position(3), None);
    }

    #[test]
    fn test_connective_relationships() {
        let triad = TriadBuilder::new()
            .name("Test Triad")
            .instances("Will", "Function", "Being")
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
            .instances("A", "B", "C")
            .build();
        assert!(valid_triad.is_ok());
        
        // Empty instance should fail
        let invalid_triad = TriadBuilder::new()
            .name("Invalid")
            .instances("A", "", "C")
            .build();
        assert!(invalid_triad.is_err());
        
        // Duplicate instances should fail
        let duplicate_triad = TriadBuilder::new()
            .name("Duplicate")
            .instances("Same", "Same", "Different")
            .build();
        assert!(duplicate_triad.is_err());
    }

    #[test]
    fn test_triad_instances_method() {
        let triad = TriadBuilder::new()
            .name("Test")
            .instances("One", "Two", "Three")
            .build()
            .unwrap();
        
        let (first, second, third) = triad.instances_tuple();
        assert_eq!(first, "One");
        assert_eq!(second, "Two");
        assert_eq!(third, "Three");
    }

    #[test]
    fn test_trait_compliance() {
        let triad = TriadBuilder::new()
            .name("Test")
            .instances("A", "B", "C")
            .build()
            .unwrap();
        
        // Test SystematicStructure trait methods
        assert_eq!(Triad::TERM_COUNT, 3);
        assert!(!triad.id().is_empty());
        assert_eq!(triad.name(), "Test");
        assert_eq!(triad.user_terms().len(), 3);
        assert_eq!(triad.user_terms()[0], "A");
        assert_eq!(triad.user_terms()[1], "B");
        assert_eq!(triad.user_terms()[2], "C");
        assert_eq!(triad.canonical_terms(), vec!["Will", "Function", "Being"]);
        assert!(triad.validate().is_ok());
    }
    
    #[test]
    fn test_legacy_terms_method() {
        // Test backward compatibility with terms() method
        let triad = TriadBuilder::new()
            .name("Test")
            .terms("Spirit", "Mind", "Body")
            .build()
            .unwrap();
            
        assert_eq!(triad.first_instance(), "Spirit");
        assert_eq!(triad.second_instance(), "Mind");
        assert_eq!(triad.third_instance(), "Body");
    }
    
    #[test]
    fn test_position_alias_methods() {
        let triad = TriadBuilder::new()
            .name("Test")
            .instances("MyWill", "MyFunction", "MyBeing")
            .build()
            .unwrap();
        
        // Test canonical term position aliases
        assert_eq!(triad.position_to_canonical_term(0), Some("Will"));
        assert_eq!(triad.position_to_canonical_term(1), Some("Function"));
        assert_eq!(triad.position_to_canonical_term(2), Some("Being"));
        assert_eq!(triad.position_to_canonical_term(3), None);
        
        assert_eq!(triad.position_from_canonical_term("Will"), Some(0));
        assert_eq!(triad.position_from_canonical_term("Function"), Some(1));
        assert_eq!(triad.position_from_canonical_term("Being"), Some(2));
        assert_eq!(triad.position_from_canonical_term("NonExistent"), None);
        
        // Test user term position aliases
        assert_eq!(triad.position_to_user_term(0), Some("MyWill"));
        assert_eq!(triad.position_to_user_term(1), Some("MyFunction"));
        assert_eq!(triad.position_to_user_term(2), Some("MyBeing"));
        assert_eq!(triad.position_to_user_term(3), None);
        
        assert_eq!(triad.position_from_user_term("MyWill"), Some(0));
        assert_eq!(triad.position_from_user_term("MyFunction"), Some(1));
        assert_eq!(triad.position_from_user_term("MyBeing"), Some(2));
        assert_eq!(triad.position_from_user_term("Unknown"), None);
        
        // Verify aliases return same results as original methods
        assert_eq!(triad.position_to_canonical_term(0), triad.canonical_term_from_position(0));
        assert_eq!(triad.position_from_canonical_term("Will"), triad.canonical_term_to_position("Will"));
        assert_eq!(triad.position_to_user_term(0), triad.instance_from_position(0));
        assert_eq!(triad.position_from_user_term("MyWill"), triad.instance_to_position("MyWill"));
    }
} 