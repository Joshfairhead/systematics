//! # Systematics API
//! 
//! A core library for creating and managing systematic structures based on Bennett's ontological frameworks.
//! 
//! This library provides a clean API for working with systematic structures from monad (1 term) 
//! to dodecad (12 terms), plus permutation generation.
//!
//! ## Quick Start
//! 
//! ```rust
//! use systematics_api::{SystematicsApi, SystematicStructure};
//! 
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let api = SystematicsApi::new();
//!     let monad = api.create_monad()
//!         .name("Unity")
//!         .term("Absolute")
//!         .build()?;
//!         
//!     println!("Created: {}", monad.name());
//!     monad.validate()?;
//!     Ok(())
//! }
//! ```

// =============================================================================
// Module Declarations
// =============================================================================

pub mod error;
pub mod structures;
pub mod permutations;

// =============================================================================
// Public Re-exports
// =============================================================================

pub use error::{SystematicsError, Result};
pub use structures::*;
pub use systematics_library::{Schema, LibraryProvider, BennettLibrary};
pub use permutations::{Permutation, PermutationSet};

// =============================================================================
// Core Traits
// =============================================================================

/// Core trait that all systematic structures must implement
/// 
/// This trait provides a unified interface for working with any systematic structure,
/// regardless of its complexity (monad to dodecad). It ensures consistent behavior
/// across all structure types.
pub trait SystematicStructure {
    // -------------------------------------------------------------------------
    // Core Identity & Structure
    // -------------------------------------------------------------------------
    
    /// The number of terms in this structure type
    const TERM_COUNT: usize;
    
    /// Unique identifier for this structure instance: Used for database storage, DHT addressing, or other unique identification needs.
    fn id(&self) -> &str;
    
    /// Human-readable name for this structure
    fn name(&self) -> &str;
    
    /// Get the coherence attribute for this structure type
    /// This defines what maintains internal consistency within the structure
    fn coherence_attribute(&self) -> &str;
    
    /// Get the term designation for this structure type
    /// This defines what individual elements should be called
    fn term_designation(&self) -> &str;
    
    // -------------------------------------------------------------------------
    // Content Access
    // -------------------------------------------------------------------------
    
    /// Get canonical term labels from the schema file
    /// For a monad: `["Unity"]`, for a dyad: `["Essence", "Existence"]`.
    fn term_characters(&self) -> Vec<String>;
    
    /// Get the name for the first order connectives of this structure type
    fn first_order_connectives_name(&self) -> &str;
    
    /// Get semantic coordinates/positions for this structure
    /// Returns the coordinate indices (0, 1, 2, etc.) that map to term positions.
    fn semantic_coordinates(&self) -> Vec<usize> {
        (0..Self::TERM_COUNT).collect()
    }
    
    /// Get current user-provided term values for each position
    fn user_terms(&self) -> &[String];
    
    // -------------------------------------------------------------------------
    // Schema & Structure
    // -------------------------------------------------------------------------
    
    /// Get the schema used for this structure
    fn schema(&self) -> &dyn systematics_library::Schema;
    
    // -------------------------------------------------------------------------
    // Validation & Integrity
    // -------------------------------------------------------------------------
    
    /// Validate the structure's internal consistency
    /// Checks that all terms are valid, relationships are consistent,
    /// and the structure follows Bennett's systematic principles.
    fn validate(&self) -> Result<()>;
    
    // -------------------------------------------------------------------------
    // Display & Output
    // -------------------------------------------------------------------------
    
    /// Display structure details in a human-readable format
    fn display(&self);
    
    // -------------------------------------------------------------------------
    // Serialization (Optional Feature)
    // -------------------------------------------------------------------------
    
    /// Export structure to JSON representation
    #[cfg(feature = "serde_support")]
    fn to_json(&self) -> Result<String>;
    
    /// Import structure from JSON representation  
    #[cfg(feature = "serde_support")]
    fn from_json(json: &str) -> Result<Self> where Self: Sized;
}

// =============================================================================
// Main API Entry Point
// =============================================================================

/// Main API entry point for creating and managing systematic structures 
pub struct SystematicsApi {
    // Future expansion: configuration, database connections, caching, etc.
}

impl SystematicsApi {
    /// Create a new API instance
    pub fn new() -> Self {
        Self {}
    }
    
    // -------------------------------------------------------------------------
    // Structure Creation Methods
    // -------------------------------------------------------------------------
    
    /// Create a new monad structure
    pub fn create_monad(&self) -> structures::monad::MonadBuilder {
        structures::monad::MonadBuilder::new()
    }
    
    /// Create a new dyad structure  
    pub fn create_dyad(&self) -> structures::dyad::DyadBuilder {
        structures::dyad::DyadBuilder::new()
    }
    
    /// Create a new triad structure
    pub fn create_triad(&self) -> structures::triad::TriadBuilder {
        structures::triad::TriadBuilder::new()
    }
    
    /// Create a new tetrad structure
    pub fn create_tetrad(&self) -> structures::tetrad::TetradBuilder {
        structures::tetrad::TetradBuilder::new()
    }
    
    /// Create a new pentad structure
    pub fn create_pentad(&self) -> structures::pentad::PentadBuilder {
        structures::pentad::PentadBuilder::new()
    }
    
    /// Create a new hexad structure
    pub fn create_hexad(&self) -> structures::hexad::HexadBuilder {
        structures::hexad::HexadBuilder::new()
    }
    
    /// Create a new heptad structure
    pub fn create_heptad(&self) -> structures::heptad::HeptadBuilder {
        structures::heptad::HeptadBuilder::new()
    }
    
    /// Create a new octad structure
    pub fn create_octad(&self) -> structures::octad::OctadBuilder {
        structures::octad::OctadBuilder::new()
    }
    
    /// Create a new dodecad structure
    pub fn create_dodecad(&self) -> structures::dodecad::DodecadBuilder {
        structures::dodecad::DodecadBuilder::new()
    }
    
    // -------------------------------------------------------------------------
    // Permutation Utilities
    // -------------------------------------------------------------------------
    
    /// Generate all six permutations for three terms
    /// 
    /// Creates the six fundamental permutation patterns (Expansion, Interaction,
    /// Order, Concentration, Identity, Freedom) for any three terms.
    pub fn permutations<T: Clone>(&self, terms: [T; 3]) -> PermutationSet<T> {
        PermutationSet::new(terms)
    }
}

impl Default for SystematicsApi {
    fn default() -> Self {
        Self::new()
    }
} 