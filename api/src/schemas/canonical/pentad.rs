use crate::schemas::Connective;

/// Bennett's canonical pentad schema - Quintessence, Higher Potential, Lower Potential, Purpose, Source
#[derive(Debug, Clone)]
pub struct PentadSchema;

impl crate::schemas::Schema for PentadSchema {
    fn term_count(&self) -> usize { 
        5 
    }
    
    fn canonical_terms(&self) -> &'static [&'static str] { 
        &["Quintessence", "Higher Potential", "Lower Potential", "Purpose", "Source"] 
    }
    
    fn name(&self) -> &'static str { 
        "Pentad Schema" 
    }
    
    fn connectives(&self) -> Vec<Connective> {
        vec![
            Connective { 
                from_position: 1, 
                to_position: 2, 
                relationship: "Range of potential".to_string(), 
                description: Some("Higher-Lower Potential".to_string()) 
            },
            Connective { 
                from_position: 3, 
                to_position: 4, 
                relationship: "Range of significance".to_string(), 
                description: Some("Purpose-Source".to_string()) 
            },
            Connective { 
                from_position: 0, 
                to_position: 1, 
                relationship: "Aspiration".to_string(), 
                description: Some("Quintessence-Higher Potential".to_string()) 
            },
            Connective { 
                from_position: 0, 
                to_position: 2, 
                relationship: "Operation".to_string(), 
                description: Some("Quintessence-Lower Potential".to_string()) 
            },
            Connective { 
                from_position: 1, 
                to_position: 3, 
                relationship: "Output".to_string(), 
                description: Some("Higher Potential-Purpose".to_string()) 
            },
            Connective { 
                from_position: 2, 
                to_position: 4, 
                relationship: "Input".to_string(), 
                description: Some("Lower Potential-Source".to_string()) 
            },
            Connective { 
                from_position: 0, 
                to_position: 3, 
                relationship: "Inspiration".to_string(), 
                description: Some("Quintessence-Purpose".to_string()) 
            },
            Connective { 
                from_position: 0, 
                to_position: 4, 
                relationship: "Quantitive match".to_string(), 
                description: Some("Quintessence-Source".to_string()) 
            },
            Connective { 
                from_position: 2, 
                to_position: 3, 
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