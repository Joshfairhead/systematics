//! # Systematics API
//! 
//! A core library for creating and managing systematic structures based on Bennett's ontological frameworks.
//! 
//! This library provides a clean API for working with systematic structures from monad (1 term) 
//! to dodecad (12 terms), plus permutation generation.

pub mod error;
pub mod structures;
pub mod schemas;
pub mod builder;
pub mod permutations;

// Re-export key types
pub use error::{SystematicsError, Result};
pub use structures::*;
pub use schemas::{Schema, SchemaProvider};
pub use builder::StructureBuilder;
pub use permutations::{Permutation, PermutationSet};

/// Core trait for all systematic structures
pub trait SystematicStructure {
    /// The number of terms in this structure
    const TERM_COUNT: usize;
    
    /// Get the structure's unique identifier
    fn id(&self) -> &str;
    
    /// Get the structure's name/title
    fn name(&self) -> &str;
    
    /// Get all terms in the structure
    fn terms(&self) -> &[String];
    
    /// Get the schema used for this structure
    fn schema(&self) -> &dyn Schema;
    
    /// Validate the structure's internal consistency
    fn validate(&self) -> Result<()>;
    
    /// Export to JSON representation
    #[cfg(feature = "serde_support")]
    fn to_json(&self) -> Result<String>;
    
    /// Import from JSON representation
    #[cfg(feature = "serde_support")]
    fn from_json(json: &str) -> Result<Self> where Self: Sized;
}

/// Builder pattern for creating structures
pub struct SystematicsBuilder;

impl SystematicsBuilder {
    pub fn new() -> Self {
        Self
    }
    
    pub fn monad(&self) -> structures::monad::MonadBuilder {
        structures::monad::MonadBuilder::new()
    }
    
    pub fn dyad(&self) -> structures::dyad::DyadBuilder {
        structures::dyad::DyadBuilder::new()
    }
    
    pub fn triad(&self) -> structures::triad::TriadBuilder {
        structures::triad::TriadBuilder::new()
    }
    
    // ... additional structure builders
}

/// Main API entry point
pub struct SystematicsApi {
    // Future: could hold configuration, database connections, etc.
}

impl SystematicsApi {
    pub fn new() -> Self {
        Self {}
    }
    
    pub fn builder(&self) -> SystematicsBuilder {
        SystematicsBuilder::new()
    }
    
    pub fn permutations<T: Clone>(&self, terms: [T; 3]) -> PermutationSet<T> {
        PermutationSet::new(terms)
    }
}

impl Default for SystematicsApi {
    fn default() -> Self {
        Self::new()
    }
} 