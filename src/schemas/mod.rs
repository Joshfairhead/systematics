/// Trait defining a semantic schema for systematic structures
pub trait StructureSchema {
    /// Get the canonical term labels for each position
    fn get_canonical_labels(&self) -> &[&'static str];
    
    /// Get a description for a specific position
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

pub mod tetrad;

pub use tetrad::*; 