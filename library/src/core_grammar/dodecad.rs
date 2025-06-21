use crate::Connective;

/// Bennett's canonical dodecadic system - Autocracy, Domination, Creativity, Pattern, Individuality, Structure, Repetition, Potentiality, Subsistence, Relatedness, Polarity, Wholeness
#[derive(Debug, Clone)]
pub struct DodecadicSystem;

impl crate::System for DodecadicSystem {
    fn term_count(&self) -> usize { 
        12 
    }
    
    fn name(&self) -> &'static str { 
        "Dodecad" 
    }
    
    /// NOTE: Research needed - these values are educated guesses based on Bennett's framework
    fn coherence_attribute(&self) -> &'static str {
        "Harmony" // TODO: Verify with Bennett's canonical texts
    }
    
    fn term_designation(&self) -> &'static str {
        "Tones" // TODO: Verify with Bennett's canonical texts
    }
    
    fn term_characters(&self) -> &'static [&'static str] { 
        &["Autocracy", "Domination", "Creativity", "Pattern", "Individuality", "Structure", "Repetition", "Potentiality", "Subsistence", "Relatedness", "Polarity", "Wholeness"] 
    }
    
    fn first_order_connectives_type(&self) -> &'static str {
        // TODO: Research proper canonical name for Dodecad connectives
        "Connectives"
    }
    
    fn connectives_traits(&self) -> Vec<Connective> {
        vec![
            // Row 0: Autocracy with all others (11 connections)
            Connective { from_position: 0, to_position: 1, relationship: "Autocracy <> Domination".to_string(), description: None },
            Connective { from_position: 0, to_position: 2, relationship: "Autocracy <> Creativity".to_string(), description: None },
            Connective { from_position: 0, to_position: 3, relationship: "Autocracy <> Pattern".to_string(), description: None },
            Connective { from_position: 0, to_position: 4, relationship: "Autocracy <> Individuality".to_string(), description: None },
            Connective { from_position: 0, to_position: 5, relationship: "Autocracy <> Structure".to_string(), description: None },
            Connective { from_position: 0, to_position: 6, relationship: "Autocracy <> Repetition".to_string(), description: None },
            Connective { from_position: 0, to_position: 7, relationship: "Autocracy <> Potentiality".to_string(), description: None },
            Connective { from_position: 0, to_position: 8, relationship: "Autocracy <> Subsistence".to_string(), description: None },
            Connective { from_position: 0, to_position: 9, relationship: "Autocracy <> Relatedness".to_string(), description: None },
            Connective { from_position: 0, to_position: 10, relationship: "Autocracy <> Polarity".to_string(), description: None },
            Connective { from_position: 0, to_position: 11, relationship: "Autocracy <> Wholeness".to_string(), description: None },
            
            // Row 1: Domination with remaining (10 connections)
            Connective { from_position: 1, to_position: 2, relationship: "Domination <> Creativity".to_string(), description: None },
            Connective { from_position: 1, to_position: 3, relationship: "Domination <> Pattern".to_string(), description: None },
            Connective { from_position: 1, to_position: 4, relationship: "Domination <> Individuality".to_string(), description: None },
            Connective { from_position: 1, to_position: 5, relationship: "Domination <> Structure".to_string(), description: None },
            Connective { from_position: 1, to_position: 6, relationship: "Domination <> Repetition".to_string(), description: None },
            Connective { from_position: 1, to_position: 7, relationship: "Domination <> Potentiality".to_string(), description: None },
            Connective { from_position: 1, to_position: 8, relationship: "Domination <> Subsistence".to_string(), description: None },
            Connective { from_position: 1, to_position: 9, relationship: "Domination <> Relatedness".to_string(), description: None },
            Connective { from_position: 1, to_position: 10, relationship: "Domination <> Polarity".to_string(), description: None },
            Connective { from_position: 1, to_position: 11, relationship: "Domination <> Wholeness".to_string(), description: None },
            
            // Row 2: Creativity with remaining (9 connections)
            Connective { from_position: 2, to_position: 3, relationship: "Creativity <> Pattern".to_string(), description: None },
            Connective { from_position: 2, to_position: 4, relationship: "Creativity <> Individuality".to_string(), description: None },
            Connective { from_position: 2, to_position: 5, relationship: "Creativity <> Structure".to_string(), description: None },
            Connective { from_position: 2, to_position: 6, relationship: "Creativity <> Repetition".to_string(), description: None },
            Connective { from_position: 2, to_position: 7, relationship: "Creativity <> Potentiality".to_string(), description: None },
            Connective { from_position: 2, to_position: 8, relationship: "Creativity <> Subsistence".to_string(), description: None },
            Connective { from_position: 2, to_position: 9, relationship: "Creativity <> Relatedness".to_string(), description: None },
            Connective { from_position: 2, to_position: 10, relationship: "Creativity <> Polarity".to_string(), description: None },
            Connective { from_position: 2, to_position: 11, relationship: "Creativity <> Wholeness".to_string(), description: None },
            
            // Row 3: Pattern with remaining (8 connections)
            Connective { from_position: 3, to_position: 4, relationship: "Pattern <> Individuality".to_string(), description: None },
            Connective { from_position: 3, to_position: 5, relationship: "Pattern <> Structure".to_string(), description: None },
            Connective { from_position: 3, to_position: 6, relationship: "Pattern <> Repetition".to_string(), description: None },
            Connective { from_position: 3, to_position: 7, relationship: "Pattern <> Potentiality".to_string(), description: None },
            Connective { from_position: 3, to_position: 8, relationship: "Pattern <> Subsistence".to_string(), description: None },
            Connective { from_position: 3, to_position: 9, relationship: "Pattern <> Relatedness".to_string(), description: None },
            Connective { from_position: 3, to_position: 10, relationship: "Pattern <> Polarity".to_string(), description: None },
            Connective { from_position: 3, to_position: 11, relationship: "Pattern <> Wholeness".to_string(), description: None },
            
            // Row 4: Individuality with remaining (7 connections)
            Connective { from_position: 4, to_position: 5, relationship: "Individuality <> Structure".to_string(), description: None },
            Connective { from_position: 4, to_position: 6, relationship: "Individuality <> Repetition".to_string(), description: None },
            Connective { from_position: 4, to_position: 7, relationship: "Individuality <> Potentiality".to_string(), description: None },
            Connective { from_position: 4, to_position: 8, relationship: "Individuality <> Subsistence".to_string(), description: None },
            Connective { from_position: 4, to_position: 9, relationship: "Individuality <> Relatedness".to_string(), description: None },
            Connective { from_position: 4, to_position: 10, relationship: "Individuality <> Polarity".to_string(), description: None },
            Connective { from_position: 4, to_position: 11, relationship: "Individuality <> Wholeness".to_string(), description: None },
            
            // Row 5: Structure with remaining (6 connections)
            Connective { from_position: 5, to_position: 6, relationship: "Structure <> Repetition".to_string(), description: None },
            Connective { from_position: 5, to_position: 7, relationship: "Structure <> Potentiality".to_string(), description: None },
            Connective { from_position: 5, to_position: 8, relationship: "Structure <> Subsistence".to_string(), description: None },
            Connective { from_position: 5, to_position: 9, relationship: "Structure <> Relatedness".to_string(), description: None },
            Connective { from_position: 5, to_position: 10, relationship: "Structure <> Polarity".to_string(), description: None },
            Connective { from_position: 5, to_position: 11, relationship: "Structure <> Wholeness".to_string(), description: None },
            
            // Row 6: Repetition with remaining (5 connections)
            Connective { from_position: 6, to_position: 7, relationship: "Repetition <> Potentiality".to_string(), description: None },
            Connective { from_position: 6, to_position: 8, relationship: "Repetition <> Subsistence".to_string(), description: None },
            Connective { from_position: 6, to_position: 9, relationship: "Repetition <> Relatedness".to_string(), description: None },
            Connective { from_position: 6, to_position: 10, relationship: "Repetition <> Polarity".to_string(), description: None },
            Connective { from_position: 6, to_position: 11, relationship: "Repetition <> Wholeness".to_string(), description: None },
            
            // Row 7: Potentiality with remaining (4 connections)
            Connective { from_position: 7, to_position: 8, relationship: "Potentiality <> Subsistence".to_string(), description: None },
            Connective { from_position: 7, to_position: 9, relationship: "Potentiality <> Relatedness".to_string(), description: None },
            Connective { from_position: 7, to_position: 10, relationship: "Potentiality <> Polarity".to_string(), description: None },
            Connective { from_position: 7, to_position: 11, relationship: "Potentiality <> Wholeness".to_string(), description: None },
            
            // Row 8: Subsistence with remaining (3 connections)
            Connective { from_position: 8, to_position: 9, relationship: "Subsistence <> Relatedness".to_string(), description: None },
            Connective { from_position: 8, to_position: 10, relationship: "Subsistence <> Polarity".to_string(), description: None },
            Connective { from_position: 8, to_position: 11, relationship: "Subsistence <> Wholeness".to_string(), description: None },
            
            // Row 9: Relatedness with remaining (2 connections)
            Connective { from_position: 9, to_position: 10, relationship: "Relatedness <> Polarity".to_string(), description: None },
            Connective { from_position: 9, to_position: 11, relationship: "Relatedness <> Wholeness".to_string(), description: None },
            
            // Row 10: Polarity with remaining (1 connection)
            Connective { from_position: 10, to_position: 11, relationship: "Polarity <> Wholeness".to_string(), description: None },
        ]
        // Total: 11+10+9+8+7+6+5+4+3+2+1 = 66 connectives ✅
    }
    
    fn source(&self) -> &'static str {
        "H3uni.org: Hodgson's QualSystems Course (Module 6)"
    }
} 