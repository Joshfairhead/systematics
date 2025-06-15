use crate::schemas::Connective;

/// Bennett's canonical triad schema - Will, Function, Being
#[derive(Debug, Clone)]
pub struct TriadSchema;

impl crate::schemas::Schema for TriadSchema {
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
    
    fn canonical_terms(&self) -> &'static [&'static str] { 
        &["Will", "Function", "Being"] 
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
} 