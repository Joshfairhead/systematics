//! # Systematics Library
//! 
//! A collection of systematic knowledge resources for different wisdom traditions.
//! This library provides system interfaces and implementations for various 
//! systematic frameworks, enabling pluggable knowledge sources while maintaining
//! the qualitative significance of number as the organizing principle.
//! 
//! ## Architecture
//! 
//! The library separates mathematical structure (handled by the API) from 
//! knowledge content (handled here). This enables:
//! 
//! - Multiple wisdom traditions (Bennett, Landry, Gurdjieff, etc.)
//! - Pluggable system providers
//! - Resource library for inference engine training
//! - REA (Resource-Event-Agent) accounting alignment

pub mod error;
pub mod bennett;
pub mod providers;

// Core system traits and types
/// Represents a directional relationship between two terms in a system
#[derive(Debug, Clone)]
pub struct Connective {
    pub from_position: usize,
    pub to_position: usize,
    pub relationship: String,
    pub description: Option<String>,
}

/// Core trait for all system types
pub trait System: Send + Sync {
    fn term_count(&self) -> usize;
    fn name(&self) -> &'static str;
    
    /// The coherence attribute that defines the system's internal consistency
    fn coherence_attribute(&self) -> &'static str;
    
    /// The term designation - what individual elements should be called (replaces "terms")
    fn term_designation(&self) -> &'static str;
    
    /// The source material where this system definition comes from
    fn source(&self) -> &'static str;
    
    fn term_characters(&self) -> &'static [&'static str];
    
    /// The name for the 1st order connectives within this system
    fn first_order_connectives_name(&self) -> &'static str;
    
    fn connectives(&self) -> Vec<Connective>;
    
    /// Validate that given terms fit this system
    fn validate_terms(&self, terms: &[String]) -> error::Result<()> {
        if terms.len() != self.term_count() {
            return Err(error::LibraryError::InvalidConfiguration(
                format!("Expected {} terms, got {}", self.term_count(), terms.len())
            ));
        }
        Ok(())
    }
}

// Re-export core types for convenience
pub use error::{LibraryError, Result};
pub use providers::{LibraryProvider, BennettLibrary};
pub use bennett::{
    MonadicSystem, DyadicSystem, TriadicSystem, TetradicSystem, 
    PentadicSystem, HexadicSystem, HeptadicSystem, OctadicSystem, 
    EnneadicSystem, DecadicSystem, UndecadicSystem, DodecadicSystem
}; 