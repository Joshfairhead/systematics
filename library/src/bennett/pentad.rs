use crate::Connective;

/// Bennett's canonical pentad schema - Quintessence, Higher Potential, Lower Potential, Purpose, Source
#[derive(Debug, Clone)]
pub struct PentadSchema;

impl crate::Schema for PentadSchema {
    fn term_count(&self) -> usize { 
        5 
    }
    
    fn name(&self) -> &'static str { 
        "Pentad" 
    }
    
    fn coherence_attribute(&self) -> &'static str {
        "Significance and Potential"
    }
    
    fn term_designation(&self) -> &'static str {
        "Limits"
    }
    
    fn term_characters(&self) -> &'static [&'static str] { 
        &["Purpose", "Higher Potential", "Quintessence", "Lower Potential", "Source"] 
    }
    
    fn first_order_connectives_name(&self) -> &'static str {
        "Mutualities"
    }
    
    fn connectives(&self) -> Vec<Connective> {
        vec![
            Connective { 
                from_position: 1, 
                to_position: 3, 
                relationship: "Range of potential".to_string(), 
                description: Some("Higher-Lower Potential".to_string()) 
            },
            Connective { 
                from_position: 0, 
                to_position: 4, 
                relationship: "Range of significance".to_string(), 
                description: Some("Purpose-Source".to_string()) 
            },
            Connective { 
                from_position: 2, 
                to_position: 1, 
                relationship: "Aspiration".to_string(), 
                description: Some("Quintessence-Higher Potential".to_string()) 
            },
            Connective { 
                from_position: 2, 
                to_position: 3, 
                relationship: "Operation".to_string(), 
                description: Some("Quintessence-Lower Potential".to_string()) 
            },
            Connective { 
                from_position: 1, 
                to_position: 0, 
                relationship: "Output".to_string(), 
                description: Some("Higher Potential-Purpose".to_string()) 
            },
            Connective { 
                from_position: 3, 
                to_position: 4, 
                relationship: "Input".to_string(), 
                description: Some("Lower Potential-Source".to_string()) 
            },
            Connective { 
                from_position: 2, 
                to_position: 0, 
                relationship: "Inspiration".to_string(), 
                description: Some("Quintessence-Purpose".to_string()) 
            },
            Connective { 
                from_position: 2, 
                to_position: 4, 
                relationship: "Quantitive match".to_string(), 
                description: Some("Quintessence-Source".to_string()) 
            },
            Connective { 
                from_position: 3, 
                to_position: 0, 
                relationship: "Form".to_string(), 
                description: Some("Lower Potential-Purpose".to_string()) 
            },
            Connective { 
                from_position: 1, 
                to_position: 4, 
                relationship: "Function".to_string(), 
                description: Some("Higher Potential-Source".to_string()) 
            },
        ]
    }
} 