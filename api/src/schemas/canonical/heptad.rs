use crate::schemas::Connective;

/// Bennett's canonical heptad schema - Insight, Research, Design, Synthesis, Application, Delivery, Value
#[derive(Debug, Clone)]
pub struct HeptadSchema;

impl crate::schemas::Schema for HeptadSchema {
    fn term_count(&self) -> usize { 
        7 
    }
    
    fn name(&self) -> &'static str { 
        "Heptad" 
    }
    
    fn coherence_attribute(&self) -> &'static str {
        "Transformation"
    }
    
    fn term_designation(&self) -> &'static str {
        "States"
    }
    
    fn term_characters(&self) -> &'static [&'static str] { 
        &["Insight", "Research", "Design", "Synthesis", "Application", "Delivery", "Value"] 
    }
    
    fn first_order_connectives_name(&self) -> &'static str {
        // TODO: Research proper canonical name for Heptad connectives
        "Connectives"
    }
    
    fn connectives(&self) -> Vec<Connective> {
        vec![
            Connective { from_position: 0, to_position: 1, relationship: "Insight <> Research".to_string(), description: None },
            Connective { from_position: 0, to_position: 2, relationship: "Insight <> Design".to_string(), description: None },
            Connective { from_position: 0, to_position: 3, relationship: "Insight <> Synthesis".to_string(), description: None },
            Connective { from_position: 0, to_position: 4, relationship: "Insight <> Application".to_string(), description: None },
            Connective { from_position: 0, to_position: 5, relationship: "Insight <> Delivery".to_string(), description: None },
            Connective { from_position: 0, to_position: 6, relationship: "Insight <> Value".to_string(), description: None },
            Connective { from_position: 1, to_position: 2, relationship: "Research <> Design".to_string(), description: None },
            Connective { from_position: 1, to_position: 3, relationship: "Research <> Synthesis".to_string(), description: None },
            Connective { from_position: 1, to_position: 4, relationship: "Research <> Application".to_string(), description: None },
            Connective { from_position: 1, to_position: 5, relationship: "Research <> Delivery".to_string(), description: None },
            Connective { from_position: 1, to_position: 6, relationship: "Research <> Value".to_string(), description: None },
            Connective { from_position: 2, to_position: 3, relationship: "Design <> Synthesis".to_string(), description: None },
            Connective { from_position: 2, to_position: 4, relationship: "Design <> Application".to_string(), description: None },
            Connective { from_position: 2, to_position: 5, relationship: "Design <> Delivery".to_string(), description: None },
            Connective { from_position: 2, to_position: 6, relationship: "Design <> Value".to_string(), description: None },
            Connective { from_position: 3, to_position: 4, relationship: "Synthesis <> Application".to_string(), description: None },
            Connective { from_position: 3, to_position: 5, relationship: "Synthesis <> Delivery".to_string(), description: None },
            Connective { from_position: 3, to_position: 6, relationship: "Synthesis <> Value".to_string(), description: None },
            Connective { from_position: 4, to_position: 5, relationship: "Application <> Delivery".to_string(), description: None },
            Connective { from_position: 4, to_position: 6, relationship: "Application <> Value".to_string(), description: None },
            Connective { from_position: 5, to_position: 6, relationship: "Delivery <> Value".to_string(), description: None },
        ]
    }
} 