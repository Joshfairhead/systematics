use crate::schemas::Connective;

/// Bennett's canonical tetrad schema - Ground, Ideal, Instrumental, Directive
#[derive(Debug, Clone)]
pub struct TetradSchema;

impl crate::schemas::Schema for TetradSchema {
    fn term_count(&self) -> usize { 
        4 
    }
    
    fn canonical_terms(&self) -> &'static [&'static str] { 
        &["Ground", "Ideal", "Instrumental", "Directive"] 
    }
    
    fn name(&self) -> &'static str { 
        "Tetrad Schema" 
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