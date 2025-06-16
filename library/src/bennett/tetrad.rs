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
        &["Ground", "Ideal", "Instrumental", "Directive"] 
    }
    
    fn first_order_connectives_name(&self) -> &'static str {
        "Interplays"
    }
    
    fn connectives(&self) -> Vec<Connective> {
        vec![
            Connective {
                from_position: 0,
                to_position: 1,
                relationship: "Motivational imperative".to_string(),
                description: Some("Ground-Ideal".to_string()),
            },
            Connective {
                from_position: 0,
                to_position: 2,
                relationship: "Technical power".to_string(),
                description: Some("Ground-Instrumental".to_string()),
            },
            Connective {
                from_position: 0,
                to_position: 3,
                relationship: "Material Mastery".to_string(),
                description: Some("Ground-Directive".to_string()),
            },
            Connective {
                from_position: 1,
                to_position: 2,
                relationship: "Effectual compatibility".to_string(),
                description: Some("Ideal-Instrumental".to_string()),
            },
            Connective {
                from_position: 1,
                to_position: 3,
                relationship: "Receptive regard".to_string(),
                description: Some("Ideal-Directive".to_string()),
            },
            Connective {
                from_position: 2,
                to_position: 3,
                relationship: "Demonstrable activity".to_string(),
                description: Some("Instrumental-Directive".to_string()),
            },
        ]
    }
} 