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

/// A dyadic structure - the fundamental duality with two terms
/// 
/// Represents Bennett's basic polarity structure with Essence and Existence.
/// Contains two terms with optional attributes and follows the
/// canonical dyad schema.
#[derive(Debug, Clone)]
pub struct Dyad {
    // Core identity
    id: String,
    name: String,
    
    // User's terms for each index position (dyad has 2 term indices)  
    user_term_index: [String; 2],
    
    // User-defined attributes
    attributes: Vec<String>,
    
    // Connective relationships (consistent with larger structures)
    connectives: HashMap<(usize, usize), String>,
    
    // Schema definition
    schema: DyadSchema,
}

// =============================================================================
// Core Implementation
// =============================================================================

impl Dyad {
    /// Create a new dyad with the given name and terms
    pub fn new(name: String, first_term: String, second_term: String) -> Self {
        let connectives = HashMap::new();
        // TODO: Add proper Bennett framework connective relationships
        // connectives.insert((0, 1), "manifests as".to_string());
        
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            user_term_index: [first_term, second_term],
            attributes: Vec::new(),
            connectives,
            schema: DyadSchema,
        }
    }
    
    // -------------------------------------------------------------------------
    // Content Access Methods
    // -------------------------------------------------------------------------
    
    /// Get the first term (Essence)
    pub fn first_term(&self) -> &str {
        &self.user_term_index[0]
    }
    
    /// Get the second term (Existence)
    pub fn second_term(&self) -> &str {
        &self.user_term_index[1]
    }
    
    /// Get both terms as a tuple
    pub fn terms_tuple(&self) -> (&str, &str) {
        (&self.user_term_index[0], &self.user_term_index[1])
    }
    
    /// Get all attributes
    pub fn attributes(&self) -> &[String] {
        &self.attributes
    }
    
    /// Get the connective relationship
    pub fn connective(&self) -> Option<&String> {
        self.connectives.get(&(0, 1))
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
    
    /// Check if dyad has a specific attribute
    pub fn has_attribute(&self, attribute: &str) -> bool {
        self.attributes.contains(&attribute.to_string())
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
        &self.user_term_index
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
        
        // Validate first term is not empty
        if self.user_term_index[0].trim().is_empty() {
            return Err(SystematicsError::StructureValidation {
                reason: "First term (Essence) cannot be empty".to_string(),
            });
        }
        
        // Validate second term is not empty
        if self.user_term_index[1].trim().is_empty() {
            return Err(SystematicsError::StructureValidation {
                reason: "Second term (Existence) cannot be empty".to_string(),
            });
        }
        
        // Validate term lengths
        for (i, term) in self.user_term_index.iter().enumerate() {
            if term.len() > 100 {
                return Err(SystematicsError::StructureValidation {
                    reason: format!("Term {} is too long (max 100 characters)", i + 1),
                });
            }
        }
        
        // Validate terms contain only allowed characters
        for (i, term) in self.user_term_index.iter().enumerate() {
            if !term.chars().all(|c| c.is_alphanumeric() || c.is_whitespace() || ".,!?'-()".contains(c)) {
                return Err(SystematicsError::StructureValidation {
                    reason: format!("Term {} contains invalid characters", i + 1),
                });
            }
        }
        
        // Validate terms are different (dyad should represent duality)
        if self.user_term_index[0].trim().to_lowercase() == self.user_term_index[1].trim().to_lowercase() {
            return Err(SystematicsError::StructureValidation {
                reason: "Dyad terms should be different to represent duality".to_string(),
            });
        }
        
        Ok(())
    }
    
    // -------------------------------------------------------------------------
    // Display & Output
    // -------------------------------------------------------------------------
    
    fn display(&self) {
        println!("\n=== {} ===", self.name);
        println!("Type: Dyad (2 terms)");
        println!("Terms: {} ↔ {}", self.first_term(), self.second_term());
        if let Some(connective) = self.connective() {
            println!("Relationship: {} {} {}", self.first_term(), connective, self.second_term());
        }
        
        if !self.attributes.is_empty() {
            println!("Attributes: {}", self.attributes.join(", "));
        } else {
            println!("Attributes: None");
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

/// Builder for creating Dyad structures
pub struct DyadBuilder {
    name: Option<String>,
    first_term: Option<String>,
    second_term: Option<String>,
    attributes: Vec<String>,
}

impl DyadBuilder {
    pub fn new() -> Self {
        Self {
            name: None,
            first_term: None,
            second_term: None,
            attributes: Vec::new(),
        }
    }
    
    /// Set the dyad name
    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }
    
    /// Set the first term (Essence)
    pub fn first_term<S: Into<String>>(mut self, term: S) -> Self {
        self.first_term = Some(term.into());
        self
    }
    
    /// Set the second term (Existence)
    pub fn second_term<S: Into<String>>(mut self, term: S) -> Self {
        self.second_term = Some(term.into());
        self
    }
    
    /// Set both terms at once
    pub fn terms<S1: Into<String>, S2: Into<String>>(mut self, first: S1, second: S2) -> Self {
        self.first_term = Some(first.into());
        self.second_term = Some(second.into());
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
    
    /// Build the dyad structure
    pub fn build(self) -> Result<Dyad> {
        let name = self.name.unwrap_or_else(|| "Unnamed Dyad".to_string());
        
        let first_term = self.first_term.ok_or_else(|| SystematicsError::Builder {
            reason: "Dyad requires a first term (Essence)".to_string(),
        })?;
        
        let second_term = self.second_term.ok_or_else(|| SystematicsError::Builder {
            reason: "Dyad requires a second term (Existence)".to_string(),
        })?;
        
        let mut dyad = Dyad::new(name, first_term, second_term);
        dyad.attributes = self.attributes;
            
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
            .first_term("Spirit")
            .second_term("Matter")
            .attribute("fundamental")
            .attribute("universal")
            .build()
            .unwrap();
            
        assert_eq!(dyad.name(), "Test Dyad");
        assert_eq!(dyad.first_term(), "Spirit");
        assert_eq!(dyad.second_term(), "Matter");
        assert_eq!(dyad.attributes().len(), 2);
        assert!(dyad.validate().is_ok());
    }
    
    #[test]
    fn test_dyad_terms_method() {
        let dyad = DyadBuilder::new()
            .name("Test")
            .terms("Essence", "Existence")
            .build()
            .unwrap();
            
        let (first, second) = dyad.terms_tuple();
        assert_eq!(first, "Essence");
        assert_eq!(second, "Existence");
    }
    
    #[test]
    fn test_dyad_validation() {
        // Test missing first term
        let result = DyadBuilder::new()
            .name("Invalid Dyad")
            .second_term("Existence")
            .build();
        assert!(result.is_err());
        
        // Test missing second term
        let result = DyadBuilder::new()
            .name("Invalid Dyad")
            .first_term("Essence")
            .build();
        assert!(result.is_err());
        
        // Test identical terms (should represent duality)
        let result = DyadBuilder::new()
            .name("Invalid Dyad")
            .terms("Same", "Same")
            .build();
        assert!(result.is_err());
    }
    
    #[test]
    fn test_attribute_management() {
        let mut dyad = DyadBuilder::new()
            .name("Test")
            .terms("Spirit", "Matter")
            .build()
            .unwrap();
            
        assert_eq!(dyad.attributes().len(), 0);
        
        dyad.add_attribute("eternal".to_string());
        assert_eq!(dyad.attributes().len(), 1);
        assert!(dyad.has_attribute("eternal"));
        
        dyad.remove_attribute("eternal");
        assert_eq!(dyad.attributes().len(), 0);
        assert!(!dyad.has_attribute("eternal"));
    }
    
    #[test]
    fn test_canonical_terms() {
        let dyad = DyadBuilder::new()
            .name("Test")
            .terms("Spirit", "Matter")
            .build()
            .unwrap();
            
        let canonical = dyad.canonical_terms();
        assert_eq!(canonical, vec!["Essence", "Existence"]);
    }
    
    #[test]
    fn test_connective_relationship() {
        let dyad = DyadBuilder::new()
            .name("Test")
            .terms("Spirit", "Matter")
            .build()
            .unwrap();
            
        // No default connective (commented out fabricated relationships)
        assert_eq!(dyad.connective(), None);
    }
    
    #[test]
    fn test_trait_compliance() {
        let dyad = DyadBuilder::new()
            .name("Test")
            .terms("Essence", "Existence")
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
} 