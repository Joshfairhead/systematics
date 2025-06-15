use crate::schemas::Connective;

/// Bennett's canonical triad schema - Will, Function, Being
#[derive(Debug, Clone)]
pub struct TriadSchema;

impl crate::schemas::Schema for TriadSchema {
    fn term_count(&self) -> usize { 
        3 
    }
    
    fn canonical_terms(&self) -> &'static [&'static str] { 
        &["Will", "Function", "Being"] 
    }
    
    fn name(&self) -> &'static str { 
        "Triad Schema" 
    }
    
    fn connectives(&self) -> Vec<Connective> {
        // TODO: Add proper Bennett framework connective relationships  
        vec![]
        // vec![
        //     Connective {
        //         from_position: 0,
        //         to_position: 1,
        //         relationship: "active force".to_string(),
        //         description: Some("Will as active force on Function".to_string()),
        //     },
        //     Connective {
        //         from_position: 1,
        //         to_position: 2,
        //         relationship: "passive force".to_string(),
        //         description: Some("Function as passive force on Being".to_string()),
        //     },
        //     Connective {
        //         from_position: 2,
        //         to_position: 0,
        //         relationship: "reconciling force".to_string(),
        //         description: Some("Being as reconciling force".to_string()),
        //     },
        // ]
    }
} 