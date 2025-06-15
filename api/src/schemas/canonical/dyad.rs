use crate::schemas::Connective;

/// Bennett's canonical dyad schema - Essence, Existence
#[derive(Debug, Clone)]
pub struct DyadSchema;

impl crate::schemas::Schema for DyadSchema {
    fn term_count(&self) -> usize { 
        2 
    }
    
    fn canonical_terms(&self) -> &'static [&'static str] { 
        &["Essence", "Existence"] 
    }
    
    fn name(&self) -> &'static str { 
        "Dyad Schema" 
    }
    
    fn connectives(&self) -> Vec<Connective> {
        // TODO: Add proper Bennett framework connective relationships
        vec![]
        // vec![
        //     Connective {
        //         from_position: 0,
        //         to_position: 1,
        //         relationship: "manifests as".to_string(),
        //         description: Some("Essence manifests as Existence".to_string()),
        //     }
        // ]
    }
} 