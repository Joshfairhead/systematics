use crate::schemas::Connective;

/// Bennett's canonical hexad schema - Resources, Values, Options, Criteria, Facts, Priorities
#[derive(Debug, Clone)]
pub struct HexadSchema;

impl crate::schemas::Schema for HexadSchema {
    fn term_count(&self) -> usize { 
        6 
    }
    
    fn canonical_terms(&self) -> &'static [&'static str] { 
        &["Resources", "Values", "Options", "Criteria", "Facts", "Priorities"] 
    }
    
    fn name(&self) -> &'static str { 
        "Hexad Schema" 
    }
    
    fn connectives(&self) -> Vec<Connective> {
        vec![
            Connective { from_position: 0, to_position: 1, relationship: "Resources <> Values".to_string(), description: None },
            Connective { from_position: 0, to_position: 2, relationship: "Resources <> Options".to_string(), description: None },
            Connective { from_position: 0, to_position: 3, relationship: "Resources <> Criteria".to_string(), description: None },
            Connective { from_position: 0, to_position: 4, relationship: "Resources <> Facts".to_string(), description: None },
            Connective { from_position: 0, to_position: 5, relationship: "Resources <> Priorities".to_string(), description: None },
            Connective { from_position: 1, to_position: 2, relationship: "Values <> Options".to_string(), description: None },
            Connective { from_position: 1, to_position: 3, relationship: "Values <> Criteria".to_string(), description: None },
            Connective { from_position: 1, to_position: 4, relationship: "Values <> Facts".to_string(), description: None },
            Connective { from_position: 1, to_position: 5, relationship: "Values <> Priorities".to_string(), description: None },
            Connective { from_position: 2, to_position: 3, relationship: "Options <> Criteria".to_string(), description: None },
            Connective { from_position: 2, to_position: 4, relationship: "Options <> Facts".to_string(), description: None },
            Connective { from_position: 2, to_position: 5, relationship: "Options <> Priorities".to_string(), description: None },
            Connective { from_position: 3, to_position: 4, relationship: "Criteria <> Facts".to_string(), description: None },
            Connective { from_position: 3, to_position: 5, relationship: "Criteria <> Priorities".to_string(), description: None },
            Connective { from_position: 4, to_position: 5, relationship: "Facts <> Priorities".to_string(), description: None },
        ]
    }
} 