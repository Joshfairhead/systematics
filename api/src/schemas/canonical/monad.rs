use crate::schemas::Connective;

/// Bennett's canonical monad schema - Unity
#[derive(Debug, Clone)]
pub struct MonadSchema;

impl crate::schemas::Schema for MonadSchema {
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
    
    fn canonical_terms(&self) -> &'static [&'static str] { 
        &["Unity"] 
    }
    
    fn first_order_connectives_name(&self) -> &'static str {
        "Connectionless unity"
    }
    
    fn connectives(&self) -> Vec<Connective> { 
        vec![] 
    }
} 