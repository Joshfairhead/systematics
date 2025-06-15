use crate::schemas::Connective;

/// Bennett's canonical monad schema - Unity
#[derive(Debug, Clone)]
pub struct MonadSchema;

impl crate::schemas::Schema for MonadSchema {
    fn term_count(&self) -> usize { 
        1 
    }
    
    fn canonical_terms(&self) -> &'static [&'static str] { 
        &["Unity"] 
    }
    
    fn name(&self) -> &'static str { 
        "Monad Schema" 
    }
    
    fn connectives(&self) -> Vec<Connective> { 
        vec![] 
    }
} 