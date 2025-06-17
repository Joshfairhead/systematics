use crate::{SystematicStructure, error::{Result, SystematicsError}};
use systematics_library::{System, MonadicSystem};
use uuid::Uuid;
use std::collections::HashMap;

// =============================================================================
// Monad Structure Definition
// =============================================================================

/// A monadic structure - the simplest systematic structure with one term
/// 
/// Represents Bennett's fundamental unit of systematic organization.
/// Contains a single term with optional attributes and follows the
/// canonical monad schema.
#[derive(Debug, Clone)]
pub struct Monad {
    // Core identity
    id: String,
    name: String,
    
    // User's terms for each index position (monad has 1 term index)
    user_term_index: [String; 1],
    
    // User-defined attributes
    attributes: Vec<String>,
    
    // Connectives (empty for monad but maintains pattern for higher structures)
    connectives: HashMap<(usize, usize), String>,
    
    // System definition
    system: MonadicSystem,
}

// =============================================================================
// Core Implementation
// =============================================================================

impl Monad {
    /// Create a new monad with the given name
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            user_term_index: [String::new()],
            attributes: Vec::new(),
            connectives: HashMap::new(),
            system: MonadicSystem,
        }
    }
    
    // -------------------------------------------------------------------------
    // Content Access Methods
    // -------------------------------------------------------------------------
    
    /// Get the monad's term value
    pub fn term(&self) -> &str {
        &self.user_term_index[0]
    }
    
    /// Get all attributes
    pub fn attributes(&self) -> &[String] {
        &self.attributes
    }
    
    // -------------------------------------------------------------------------
    // Attribute Management Methods
    // -------------------------------------------------------------------------
    
    /// Add a single attribute
    pub fn add_attribute(&mut self, attribute: String) {
        self.attributes.push(attribute);
    }
    
    /// Remove an attribute by value
    pub fn remove_attribute(&mut self, attribute: &str) {
        self.attributes.retain(|attr| attr != attribute);
    }
    
    /// Check if monad has a specific attribute
    pub fn has_attribute(&self, attribute: &str) -> bool {
        self.attributes.contains(&attribute.to_string())
    }
    
    /// Get connectives (empty for monad but maintains pattern)
    pub fn connectives(&self) -> &HashMap<(usize, usize), String> {
        &self.connectives
    }
}

// =============================================================================
// SystematicStructure Trait Implementation
// =============================================================================

impl SystematicStructure for Monad {
    const TERM_COUNT: usize = 1;
    
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
        "monad"
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
    
    // -------------------------------------------------------------------------
    // Content Access
    // -------------------------------------------------------------------------
    
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
                reason: "Monad name cannot be empty".to_string(),
            });
        }
        
        // Validate term is not empty
        if self.user_term_index[0].trim().is_empty() {
            return Err(SystematicsError::StructureValidation {
                reason: "Monad term cannot be empty".to_string(),
            });
        }
        
        // Validate term length
        if self.user_term_index[0].len() > 100 {
            return Err(SystematicsError::StructureValidation {
                reason: "Monad term is too long (max 100 characters)".to_string(),
            });
        }
        
        // Validate term contains only allowed characters
        if !self.user_term_index[0].chars().all(|c| c.is_alphanumeric() || c.is_whitespace() || ".,!?'-()".contains(c)) {
            return Err(SystematicsError::StructureValidation {
                reason: "Monad term contains invalid characters".to_string(),
            });
        }
        
        Ok(())
    }
    
    // -------------------------------------------------------------------------
    // Display & Output
    // -------------------------------------------------------------------------
    
    fn display(&self) {
        let header = "=== Monadic Structure ===";
        println!("\n{}", header);
        
        // Special monad logic: if name and term are the same, only show name
        if self.name == self.term() {
            println!("Name: {}", self.name);
        } else {
            println!("Name: {}", self.name);
            println!("{}: {}", self.term_designation(), self.term());
        }
        
        if !self.attributes.is_empty() {
            println!("Attributes:");
            for attr in &self.attributes {
                println!("  - {}", attr);
            }
        } else {
            println!("Attributes: None");
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

/// Builder for creating Monad structures
pub struct MonadBuilder {
    name: Option<String>,
    term: Option<String>,
    attributes: Vec<String>,
}

impl MonadBuilder {
    pub fn new() -> Self {
        Self {
            name: None,
            term: None,
            attributes: Vec::new(),
        }
    }
    
    /// Set the monad name
    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }
    
    /// Set the monad term
    pub fn term<S: Into<String>>(mut self, term: S) -> Self {
        self.term = Some(term.into());
        self
    }
    
    /// Add a single attribute
    pub fn attribute<S: Into<String>>(mut self, attribute: S) -> Self {
        self.attributes.push(attribute.into());
        self
    }
    
    /// Add multiple attributes
    pub fn attributes<I, S>(mut self, attributes: I) -> Self 
    where 
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.attributes.extend(attributes.into_iter().map(|s| s.into()));
        self
    }
    
    /// Build the monad structure
    pub fn build(self) -> Result<Monad> {
        let name = self.name.unwrap_or_else(|| "Unnamed Monad".to_string());
        let term = self.term.ok_or_else(|| SystematicsError::Builder {
            reason: "Monad requires a term".to_string(),
        })?;
        
        let mut monad = Monad::new(name);
        monad.user_term_index[0] = term;
        monad.attributes = self.attributes;
            
        monad.validate()?;
        Ok(monad)
    }
}

impl Default for MonadBuilder {
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
    fn test_monad_creation() {
        let monad = MonadBuilder::new()
            .name("Test Monad")
            .term("Unity")
            .attribute("infinite")
            .attribute("eternal")
            .build()
            .unwrap();
            
        assert_eq!(monad.name(), "Test Monad");
        assert_eq!(monad.term(), "Unity");
        assert_eq!(monad.attributes().len(), 2);
        assert!(monad.validate().is_ok());
    }
    
    #[test]
    fn test_monad_validation() {
        let result = MonadBuilder::new()
            .name("Invalid Monad")
            .term("")
            .build();
            
        assert!(result.is_err());
    }
    
    #[test]
    fn test_attribute_management() {
        let mut monad = MonadBuilder::new()
            .name("Test")
            .term("Unity")
            .build()
            .unwrap();
            
        assert_eq!(monad.attributes().len(), 0);
        
        monad.add_attribute("infinite".to_string());
        assert_eq!(monad.attributes().len(), 1);
        assert!(monad.has_attribute("infinite"));
        
        monad.remove_attribute("infinite");
        assert_eq!(monad.attributes().len(), 0);
        assert!(!monad.has_attribute("infinite"));
    }
    
    #[test]
    fn test_term_characters() {
        let monad = MonadBuilder::new()
            .name("Test")
            .term("Unity")
            .build()
            .unwrap();
            
        let characters = monad.term_characters();
        assert_eq!(characters, vec!["Unity"]);
    }
    
    #[test]
    fn test_trait_compliance() {
        let monad = MonadBuilder::new()
            .name("Test")
            .term("Absolute")
            .build()
            .unwrap();
            
        // Test SystematicStructure trait methods
        assert_eq!(Monad::TERM_COUNT, 1);
        assert!(!monad.id().is_empty());
        assert_eq!(monad.name(), "Test");
        assert_eq!(monad.user_terms().len(), 1);
        assert_eq!(monad.user_terms()[0], "Absolute");
        assert_eq!(monad.term_characters(), vec!["Unity"]);
        assert!(monad.validate().is_ok());
    }
} 