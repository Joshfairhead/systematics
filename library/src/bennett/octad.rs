use crate::Connective;

/// Bennett's canonical octadic system - Smallest Significant Holon, Critical Functions, Supportive Platform, Necessary Resourcing, Integrative Totality, Inherent Values, Intrinsic Nature, Organisational Modes
#[derive(Debug, Clone)]
pub struct OctadicSystem;

impl crate::System for OctadicSystem {
    fn term_count(&self) -> usize { 
        8 
    }
    
    fn name(&self) -> &'static str { 
        "Octad" 
    }
    
    fn coherence_attribute(&self) -> &'static str {
        "Self-Sufficiency"
    }
    
    fn term_designation(&self) -> &'static str {
        "Elements"
    }
    
    fn term_characters(&self) -> &'static [&'static str] { 
        &["Smallest Significant Holon", "Critical Functions", "Supportive Platform", "Necessary Resourcing", "Integrative Totality", "Inherent Values", "Intrinsic Nature", "Organisational Modes"] 
    }
    
    fn first_order_connectives_name(&self) -> &'static str {
        // TODO: Research proper canonical name for Octad connectives
        "Connectives"
    }
    
    fn connectives(&self) -> Vec<Connective> {
        vec![
            Connective { from_position: 0, to_position: 1, relationship: "Smallest Significant Holon <> Critical Functions".to_string(), description: None },
            Connective { from_position: 0, to_position: 2, relationship: "Smallest Significant Holon <> Supportive Platform".to_string(), description: None },
            Connective { from_position: 0, to_position: 3, relationship: "Smallest Significant Holon <> Necessary Resourcing".to_string(), description: None },
            Connective { from_position: 0, to_position: 4, relationship: "Smallest Significant Holon <> Integrative Totality".to_string(), description: None },
            Connective { from_position: 0, to_position: 5, relationship: "Smallest Significant Holon <> Inherent Values".to_string(), description: None },
            Connective { from_position: 0, to_position: 6, relationship: "Smallest Significant Holon <> Intrinsic Nature".to_string(), description: None },
            Connective { from_position: 0, to_position: 7, relationship: "Smallest Significant Holon <> Organisational Modes".to_string(), description: None },
            Connective { from_position: 1, to_position: 2, relationship: "Critical Functions <> Supportive Platform".to_string(), description: None },
            Connective { from_position: 1, to_position: 3, relationship: "Critical Functions <> Necessary Resourcing".to_string(), description: None },
            Connective { from_position: 1, to_position: 4, relationship: "Critical Functions <> Integrative Totality".to_string(), description: None },
            Connective { from_position: 1, to_position: 5, relationship: "Critical Functions <> Inherent Values".to_string(), description: None },
            Connective { from_position: 1, to_position: 6, relationship: "Critical Functions <> Intrinsic Nature".to_string(), description: None },
            Connective { from_position: 1, to_position: 7, relationship: "Critical Functions <> Organisational Modes".to_string(), description: None },
            Connective { from_position: 2, to_position: 3, relationship: "Supportive Platform <> Necessary Resourcing".to_string(), description: None },
            Connective { from_position: 2, to_position: 4, relationship: "Supportive Platform <> Integrative Totality".to_string(), description: None },
            Connective { from_position: 2, to_position: 5, relationship: "Supportive Platform <> Inherent Values".to_string(), description: None },
            Connective { from_position: 2, to_position: 6, relationship: "Supportive Platform <> Intrinsic Nature".to_string(), description: None },
            Connective { from_position: 2, to_position: 7, relationship: "Supportive Platform <> Organisational Modes".to_string(), description: None },
            Connective { from_position: 3, to_position: 4, relationship: "Necessary Resourcing <> Integrative Totality".to_string(), description: None },
            Connective { from_position: 3, to_position: 5, relationship: "Necessary Resourcing <> Inherent Values".to_string(), description: None },
            Connective { from_position: 3, to_position: 6, relationship: "Necessary Resourcing <> Intrinsic Nature".to_string(), description: None },
            Connective { from_position: 3, to_position: 7, relationship: "Necessary Resourcing <> Organisational Modes".to_string(), description: None },
            Connective { from_position: 4, to_position: 5, relationship: "Integrative Totality <> Inherent Values".to_string(), description: None },
            Connective { from_position: 4, to_position: 6, relationship: "Integrative Totality <> Intrinsic Nature".to_string(), description: None },
            Connective { from_position: 4, to_position: 7, relationship: "Integrative Totality <> Organisational Modes".to_string(), description: None },
            Connective { from_position: 5, to_position: 6, relationship: "Inherent Values <> Intrinsic Nature".to_string(), description: None },
            Connective { from_position: 5, to_position: 7, relationship: "Inherent Values <> Organisational Modes".to_string(), description: None },
            Connective { from_position: 6, to_position: 7, relationship: "Intrinsic Nature <> Organisational Modes".to_string(), description: None },
        ]
    }
    
    fn source(&self) -> &'static str {
        "H3uni.org: Hodgson's QualSystems Course (Module 3) / QualSystems Book"
    }
} 