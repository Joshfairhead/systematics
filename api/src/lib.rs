//! # Systematics API
//! 
//! A core library for creating and managing systematic structures based on the qualitative significance of number.
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
pub mod definitions;
pub mod permutations;
pub mod storage;

#[cfg(feature = "server")]
pub mod server;

// =============================================================================
// Public Re-exports
// =============================================================================

pub use error::{SystematicsError, Result};
pub use definitions::*;
pub use systematics_library::{System, LibraryProvider, BennettLibrary};
pub use permutations::{Permutation, PermutationSet};
pub use storage::{SurrealStorage, StoredUserDefinition, GraphNode, GraphEdge};

#[cfg(feature = "server")]
pub use server::{start_server, create_router, AppState, ApiResponse};

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
    
    /// Get the structure type name (e.g., "monad", "dyad", "triad")
    fn structure_type(&self) -> &str;
    
    /// Get the coherence attribute for this structure type
    /// This defines what maintains internal consistency within the structure
    fn coherence_attribute(&self) -> &str;
    
    /// Get the term designation for this structure type
    /// This defines what individual elements should be called
    fn term_designation(&self) -> &str;
    
    /// Get the source material where this system definition comes from
    fn source(&self) -> &str;
    

    
    // -------------------------------------------------------------------------
    // Content Access
    // -------------------------------------------------------------------------
    
    /// Get canonical term labels from the system file
    /// For a monad: `["Unity"]`, for a dyad: `["Essence", "Existence"]`.
    fn term_characters(&self) -> Vec<String>;

    /// Get current user-provided term values for each position (for storage)
    fn user_instance_index(&self) -> &[String];
    
    /// Get the type for the first order connectives of this structure type
    fn first_order_connectives_type(&self) -> &str;
    
    /// Get connective relationship traits between terms
    /// Returns a map of (from_index, to_index) -> relationship_name
    fn connectives_traits(&self) -> &std::collections::HashMap<(usize, usize), String>;
    
    /// Get semantic coordinates/positions for this structure
    /// Returns the coordinate indices (0, 1, 2, etc.) that map to term positions.
    fn semantic_coordinates(&self) -> Vec<usize> {
        (0..Self::TERM_COUNT).collect()
    }
    
    // -------------------------------------------------------------------------
    // Schema & Structure
    // -------------------------------------------------------------------------
    
    /// Get the system used for this structure
    fn system(&self) -> &dyn systematics_library::System;
    
    // -------------------------------------------------------------------------
    // Validation & Integrity
    // -------------------------------------------------------------------------
    
    /// Validate the structure's internal consistency
    /// Checks that all terms are valid, relationships are consistent,
    /// and the structure follows systematic principles.
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
    fn to_json(&self) -> Result<String> {
        Err(SystematicsError::NotSupported {
            feature: "JSON serialization".to_string(),
            reason: "Enable serde feature and derive Serialize for this structure".to_string(),
        })
    }
    
    /// Import structure from JSON representation  
    fn from_json(_json: &str) -> Result<Self> where Self: Sized {
        Err(SystematicsError::NotSupported {
            feature: "JSON deserialization".to_string(),
            reason: "Enable serde feature and derive Deserialize for this structure".to_string(),
        })
    }
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
    pub fn create_monad(&self) -> definitions::monad::MonadBuilder {
        definitions::monad::MonadBuilder::new()
    }
    
    /// Create a new dyad structure  
    pub fn create_dyad(&self) -> definitions::dyad::DyadBuilder {
        definitions::dyad::DyadBuilder::new()
    }
    
    /// Create a new triad structure
    pub fn create_triad(&self) -> definitions::triad::TriadBuilder {
        definitions::triad::TriadBuilder::new()
    }
    
    /// Create a new tetrad structure
    pub fn create_tetrad(&self) -> definitions::tetrad::TetradBuilder {
        definitions::tetrad::TetradBuilder::new()
    }
    
    /// Create a new pentad structure
    pub fn create_pentad(&self) -> definitions::pentad::PentadBuilder {
        definitions::pentad::PentadBuilder::new()
    }
    
    /// Create a new hexad structure
    pub fn create_hexad(&self) -> definitions::hexad::HexadBuilder {
        definitions::hexad::HexadBuilder::new()
    }
    
    /// Create a new heptad structure
    pub fn create_heptad(&self) -> definitions::heptad::HeptadBuilder {
        definitions::heptad::HeptadBuilder::new()
    }
    
    /// Create a new octad structure
    pub fn create_octad(&self) -> definitions::octad::OctadBuilder {
        definitions::octad::OctadBuilder::new()
    }
    
    /// Create a new ennead structure
    pub fn create_ennead(&self) -> definitions::ennead::EnneadBuilder {
        definitions::ennead::EnneadBuilder::new()
    }
    
    /// Create a new decad structure
    pub fn create_decad(&self) -> definitions::decad::DecadBuilder {
        definitions::decad::DecadBuilder::new()
    }
    
    /// Create a new undecad structure
    pub fn create_undecad(&self) -> definitions::undecad::UndecadBuilder {
        definitions::undecad::UndecadBuilder::new()
    }
    
    /// Create a new dodecad structure
    pub fn create_dodecad(&self) -> definitions::dodecad::DodecadBuilder {
        definitions::dodecad::DodecadBuilder::new()
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