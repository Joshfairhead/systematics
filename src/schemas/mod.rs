/// Trait defining a semantic schema for systematic structures
pub trait StructureSchema {
    /// Get the canonical term labels for each position
    fn get_canonical_labels(&self) -> &[&'static str];
    
    /// Get a description for a specific position
    #[allow(dead_code)]
    fn get_position_description(&self, pos: usize) -> Option<&'static str>;
    
    /// Get a semantic label for a connective between two positions
    fn get_connective_label(&self, i: usize, j: usize) -> Option<&'static str>;
    
    /// Get the core attribute description for this structure type
    fn get_attribute_description(&self) -> &'static str;
    
    /// Get the name of this schema
    fn get_schema_name(&self) -> &'static str;
    
    /// Get the structure name (e.g., "Tetrad", "Pentad")
    fn get_structure_name(&self) -> &'static str;
    
    /// Get the number of positions this schema supports
    fn get_position_count(&self) -> usize;
}

pub mod schema1_monad;
pub mod schema2_dyad;
pub mod schema3_triad;
pub mod schema4_tetrad;
pub mod schema5_pentad;
pub mod schema6_hexad;
pub mod schema7_heptad;
pub mod schema8_octad;
pub mod schema12_dodecad;

pub use schema1_monad::*;
pub use schema2_dyad::*;
pub use schema3_triad::*;
pub use schema4_tetrad::*;
pub use schema5_pentad::*;
pub use schema6_hexad::*;
pub use schema7_heptad::*;
pub use schema8_octad::*;
pub use schema12_dodecad::*; 