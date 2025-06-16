use crate::Connective;

/// Bennett's canonical dyadic system - Essence, Existence
#[derive(Debug, Clone)]
pub struct DyadicSystem;

impl crate::System for DyadicSystem {
    fn term_count(&self) -> usize { 
        2 
    }
    
    fn name(&self) -> &'static str { 
        "Dyad" 
    }
    
    fn coherence_attribute(&self) -> &'static str {
        "Complimentarity"
    }
    
    fn term_designation(&self) -> &'static str {
        "Poles"
    }
    
    fn term_characters(&self) -> &'static [&'static str] { 
        &["Essence", "Existence"] 
    }
    
    fn first_order_connectives_name(&self) -> &'static str {
        "Force"
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