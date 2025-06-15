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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schemas::canonical::*;

    #[test]
    fn test_schema_refactoring_new_methods() {
        // Test that all schemas have the new methods properly implemented
        let monad = MonadSchema;
        let dyad = DyadSchema;
        let triad = TriadSchema;
        let tetrad = TetradSchema;
        let pentad = PentadSchema;
        let hexad = HexadSchema;
        let heptad = HeptadSchema;
        let octad = OctadSchema;
        let dodecad = DodecadSchema;

        // Test Monad
        assert_eq!(monad.name(), "Monad");
        assert_eq!(monad.coherence_attribute(), "Universality");
        assert_eq!(monad.term_designation(), "Totality");
        assert_eq!(monad.first_order_connectives_name(), "Connectionless unity");

        // Test Dyad
        assert_eq!(dyad.name(), "Dyad");
        assert_eq!(dyad.coherence_attribute(), "Complimentarity");
        assert_eq!(dyad.term_designation(), "Poles");
        assert_eq!(dyad.first_order_connectives_name(), "Force");

        // Test Triad
        assert_eq!(triad.name(), "Triad");
        assert_eq!(triad.coherence_attribute(), "Dynamism");
        assert_eq!(triad.term_designation(), "Impulses");
        assert_eq!(triad.first_order_connectives_name(), "Acts");

        // Test Tetrad
        assert_eq!(tetrad.name(), "Tetrad");
        assert_eq!(tetrad.coherence_attribute(), "Activity Field");
        assert_eq!(tetrad.term_designation(), "Sources");
        assert_eq!(tetrad.first_order_connectives_name(), "Interplays");

        // Test Pentad
        assert_eq!(pentad.name(), "Pentad");
        assert_eq!(pentad.coherence_attribute(), "Significance and Potential");
        assert_eq!(pentad.term_designation(), "Limits");
        assert_eq!(pentad.first_order_connectives_name(), "Mutualities");

        // Test Hexad
        assert_eq!(hexad.name(), "Hexad");
        assert_eq!(hexad.coherence_attribute(), "Coalescence");
        assert_eq!(hexad.term_designation(), "Laws");
        assert_eq!(hexad.first_order_connectives_name(), "Connectives");

        // Test Heptad
        assert_eq!(heptad.name(), "Heptad");
        assert_eq!(heptad.coherence_attribute(), "Transformation");
        assert_eq!(heptad.term_designation(), "States");
        assert_eq!(heptad.first_order_connectives_name(), "Connectives");

        // Test Octad
        assert_eq!(octad.name(), "Octad");
        assert_eq!(octad.coherence_attribute(), "Self-Sufficiency");
        assert_eq!(octad.term_designation(), "Elements");
        assert_eq!(octad.first_order_connectives_name(), "Connectives");

        // Test Dodecad (with research needed note)
        assert_eq!(dodecad.name(), "Dodecad");
        assert_eq!(dodecad.coherence_attribute(), "Harmony");
        assert_eq!(dodecad.term_designation(), "Tones");
        assert_eq!(dodecad.first_order_connectives_name(), "Connectives");
    }
} 