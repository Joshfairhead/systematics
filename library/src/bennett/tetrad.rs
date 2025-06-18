use crate::Connective;

/// Bennett's canonical tetradic system - Ground, Ideal, Instrumental, Directive
#[derive(Debug, Clone)]
pub struct TetradicSystem;

impl crate::System for TetradicSystem {
    fn term_count(&self) -> usize { 
        4 
    }
    
    fn name(&self) -> &'static str { 
        "Tetrad" 
    }
    
    fn coherence_attribute(&self) -> &'static str {
        "Activity Field"
    }
    
    fn term_designation(&self) -> &'static str {
        "Sources"
    }
    
    fn term_characters(&self) -> &'static [&'static str] { 
        &["Ideal", "Directive", "Instrumental", "Ground"] 
    }
    
    fn first_order_connectives_name(&self) -> &'static str {
        "Interplays"
    }
    
    fn connectives(&self) -> Vec<Connective> {
        vec![
            Connective {
                from_position: 0,
                to_position: 1,
                relationship: "Receptive Regard".to_string(),
                description: Some("Ideal-Directive".to_string()),
            },
            Connective {
                from_position: 0,
                to_position: 2,
                relationship: "Effectual Compatibility".to_string(),
                description: Some("Ideal-Instrumental".to_string()),
            },
            Connective {
                from_position: 0,
                to_position: 3,
                relationship: "Motivational Imperative".to_string(),
                description: Some("Ideal-Ground".to_string()),
            },
            Connective {
                from_position: 1,
                to_position: 2,
                relationship: "Demonstrable Activity".to_string(),
                description: Some("Directive-Instrumental".to_string()),
            },
            Connective {
                from_position: 1,
                to_position: 3,
                relationship: "Material Mastery".to_string(),
                description: Some("Directive-Ground".to_string()),
            },
            Connective {
                from_position: 2,
                to_position: 3,
                relationship: "Technical Power".to_string(),
                description: Some("Instrumental-Ground".to_string()),
            },
        ]
    }
    
    fn source(&self) -> &'static str {
        "H3uni.org: Hodgson's QualSystems Course (Module 2) / QualSystems Book / Bennett's Elementary Systematics"
    }
} 