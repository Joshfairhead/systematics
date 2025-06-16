use crate::{Schema, bennett::*};

/// Provides access to schemas for different structure types
pub trait LibraryProvider {
    fn get_schema(&self, term_count: usize) -> Option<Box<dyn Schema>>;
}

/// Bennett's canonical schemas library provider
pub struct BennettLibrary;

impl LibraryProvider for BennettLibrary {
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