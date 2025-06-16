//! # Systematics Library
//! 
//! A collection of systematic knowledge resources for different wisdom traditions.
//! This library provides schema interfaces and implementations for various 
//! systematic frameworks while keeping the core mathematical structures neutral.

pub mod error;

// Core schema traits and types
/// Represents a directional relationship between two terms in a schema
#[derive(Debug, Clone)]
pub struct Connective {
    pub from_position: usize,
    pub to_position: usize,
    pub relationship: String,
    pub description: Option<String>,
}

/// Core trait for all schema types
pub trait Schema: Send + Sync {
    fn term_count(&self) -> usize;
    fn name(&self) -> &'static str;
    
    /// The coherence attribute that defines the system's internal consistency
    fn coherence_attribute(&self) -> &'static str;
    
    /// The term designation - what individual elements should be called (replaces "terms")
    fn term_designation(&self) -> &'static str;
    
    fn term_characters(&self) -> &'static [&'static str];
    
    /// The name for the 1st order connectives within this system
    fn first_order_connectives_name(&self) -> &'static str;
    
    fn connectives(&self) -> Vec<Connective>;
    
    /// Validate that given terms fit this schema
    fn validate_terms(&self, terms: &[String]) -> error::Result<()> {
        if terms.len() != self.term_count() {
            return Err(error::LibraryError::InvalidConfiguration(
                format!("Expected {} terms, got {}", self.term_count(), terms.len())
            ));
        }
        Ok(())
    }
}

// Re-export error types for convenience
pub use error::{LibraryError, Result}; 