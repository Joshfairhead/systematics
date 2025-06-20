use crate::Connective;

/// Bennett's canonical pentadic system - Quintessence, Higher Potential, Lower Potential, Purpose, Source
#[derive(Debug, Clone)]
pub struct PentadicSystem;

impl crate::System for PentadicSystem {
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
    
    fn first_order_connectives_type(&self) -> &'static str {
        "Mutualities"
    }
    
    fn connectives_traits(&self) -> Vec<Connective> {
        vec![
            // Index 0: Range of potential
            Connective { 
                from_position: 1, 
                to_position: 3, 
                relationship: "Range Of Potential".to_string(), 
                description: Some("Higher-Lower Potential".to_string()) 
            },
            // Index 1: Range of significance
            Connective { 
                from_position: 0, 
                to_position: 4, 
                relationship: "Range Of Significance".to_string(), 
                description: Some("Purpose-Source".to_string()) 
            },
            // Index 2: Aspiration
            Connective { 
                from_position: 2, 
                to_position: 1, 
                relationship: "Aspiration".to_string(), 
                description: Some("Quintessence-Higher Potential".to_string()) 
            },
            // Index 3: Operation
            Connective { 
                from_position: 2, 
                to_position: 3, 
                relationship: "Operation".to_string(), 
                description: Some("Quintessence-Lower Potential".to_string()) 
            },
            // Index 4: Output
            Connective { 
                from_position: 1, 
                to_position: 0, 
                relationship: "Output".to_string(), 
                description: Some("Higher Potential-Purpose".to_string()) 
            },
            // Index 5: Input
            Connective { 
                from_position: 3, 
                to_position: 4, 
                relationship: "Input".to_string(), 
                description: Some("Lower Potential-Source".to_string()) 
            },
            // Index 6: Qualitive match
            Connective { 
                from_position: 2, 
                to_position: 0, 
                relationship: "Qualitive Match".to_string(), 
                description: Some("Quintessence-Purpose".to_string()) 
            },
            // Index 7: Quantitive match
            Connective { 
                from_position: 2, 
                to_position: 4, 
                relationship: "Quantitive Match".to_string(), 
                description: Some("Quintessence-Source".to_string()) 
            },
            // Index 8: Form
            Connective { 
                from_position: 3, 
                to_position: 0, 
                relationship: "Form".to_string(), 
                description: Some("Lower Potential-Purpose".to_string()) 
            },
            // Index 9: Function
            Connective { 
                from_position: 1, 
                to_position: 4, 
                relationship: "Function".to_string(), 
                description: Some("Higher Potential-Source".to_string()) 
            },
        ]
    }
    
    fn source(&self) -> &'static str {
        "H3uni.org: Hodgson's QualSystems Course (Module 2) / QualSystems Book / Bennett's Elementary Systematics"
    }
} 