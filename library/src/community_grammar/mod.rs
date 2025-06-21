use crate::{System, core_grammar::*};

/// Provides access to systems for different structure types
pub trait LibraryProvider {
    fn get_system(&self, term_count: usize) -> Option<Box<dyn System>>;
}

/// Bennett's canonical systems library provider
pub struct BennettLibrary;

impl LibraryProvider for BennettLibrary {
    fn get_system(&self, term_count: usize) -> Option<Box<dyn System>> {
        match term_count {
            1 => Some(Box::new(MonadicSystem)),
            2 => Some(Box::new(DyadicSystem)),
            3 => Some(Box::new(TriadicSystem)),
            4 => Some(Box::new(TetradicSystem)),
            5 => Some(Box::new(PentadicSystem)),
            6 => Some(Box::new(HexadicSystem)),
            7 => Some(Box::new(HeptadicSystem)),
            8 => Some(Box::new(OctadicSystem)),
            12 => Some(Box::new(DodecadicSystem)),
            _ => None,
        }
    }
} 