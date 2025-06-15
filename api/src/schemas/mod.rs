pub mod canonical;
pub mod providers;

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
    fn canonical_terms(&self) -> &'static [&'static str];
    fn name(&self) -> &'static str;
    fn connectives(&self) -> Vec<Connective>;
    
    /// Validate that given terms fit this schema
    fn validate_terms(&self, terms: &[String]) -> crate::error::Result<()> {
        if terms.len() != self.term_count() {
            return Err(crate::error::SystematicsError::InvalidTermCount {
                expected: self.term_count(),
                actual: terms.len(),
            });
        }
        Ok(())
    }
}

// Re-export canonical schemas for convenience
pub use canonical::{
    MonadSchema, DyadSchema, TriadSchema, TetradSchema, 
    PentadSchema, HexadSchema, HeptadSchema, OctadSchema, DodecadSchema
};

// Re-export providers for convenience
pub use providers::{SchemaProvider, BennettSchemas}; 