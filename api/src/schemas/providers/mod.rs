use crate::schemas::{Schema, canonical::*};

/// Provides access to schemas for different structure types
pub trait SchemaProvider {
    fn get_schema(&self, term_count: usize) -> Option<Box<dyn Schema>>;
}

/// Bennett's canonical schemas provider
pub struct BennettSchemas;

impl SchemaProvider for BennettSchemas {
    fn get_schema(&self, term_count: usize) -> Option<Box<dyn Schema>> {
        match term_count {
            1 => Some(Box::new(MonadSchema)),
            2 => Some(Box::new(DyadSchema)),
            3 => Some(Box::new(TriadSchema)),
            4 => Some(Box::new(TetradSchema)),
            5 => Some(Box::new(PentadSchema)),
            6 => Some(Box::new(HexadSchema)),
            7 => Some(Box::new(HeptadSchema)),
            8 => Some(Box::new(OctadSchema)),
            12 => Some(Box::new(DodecadSchema)),
            _ => None,
        }
    }
} 