use crate::Connective;

/// Bennett's canonical triadic system - Will, Function, Being
#[derive(Debug, Clone)]
pub struct TriadicSystem;

impl crate::System for TriadicSystem {
    fn term_count(&self) -> usize { 
        3 
    }
    
    fn name(&self) -> &'static str { 
        "Triad" 
    }
    
    fn coherence_attribute(&self) -> &'static str {
        "Dynamism"
    }
    
    fn term_designation(&self) -> &'static str {
        "Impulses"
    }
    
    fn term_characters(&self) -> &'static [&'static str] { 
        &["Will", "Being", "Function"] 
    }
    
    fn first_order_connectives_name(&self) -> &'static str {
        "Acts"
    }
    
    fn connectives(&self) -> Vec<Connective> {
        // TODO: Review these connective names for Bennett canonical accuracy
        vec![
            Connective { 
                from_position: 0, 
                to_position: 1, 
                relationship: "Will → Function".to_string(), 
                description: Some("Active impulse operates through".to_string()) 
            },
            Connective { 
                from_position: 1, 
                to_position: 2, 
                relationship: "Function → Being".to_string(), 
                description: Some("Function manifests in".to_string()) 
            },
            Connective { 
                from_position: 2, 
                to_position: 0, 
                relationship: "Being → Will".to_string(), 
                description: Some("Being enables".to_string()) 
            },
        ]
    }
    
    fn source(&self) -> &'static str {
        "H3uni.org: Hodgson's QualSystems Course (Module 2) / QualSystems Book / Bennett's Elementary Systematics"
    }
} 