use crate::Connective;

/// Bennett's canonical monadic system - Unity
#[derive(Debug, Clone)]
pub struct MonadicSystem;

impl crate::System for MonadicSystem {
    fn term_count(&self) -> usize { 
        1 
    }
    
    fn name(&self) -> &'static str { 
        "Monad" 
    }
    
    fn coherence_attribute(&self) -> &'static str {
        "Universality"
    }
    
    fn term_designation(&self) -> &'static str {
        "Totality"
    }
    
    fn term_characters(&self) -> &'static [&'static str] { 
        &["Unity"] 
    }
    
    fn first_order_connectives_name(&self) -> &'static str {
        "Connectionless unity"
    }
    
    fn connectives(&self) -> Vec<Connective> { 
        vec![] 
    }
} 