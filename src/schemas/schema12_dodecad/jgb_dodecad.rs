use crate::schemas::StructureSchema;

/// Bennett's Dodecad Schema - Autocracy, Domination, Creativity, Pattern, Individuality, Structure, Repetition, Potentiality, Subsistence, Relatedness, Polarity, Wholeness
pub struct BennettDodecadSchema;

impl StructureSchema for BennettDodecadSchema {
    fn get_canonical_labels(&self) -> &[&'static str] {
        &[
            "Autocracy", 
            "Domination", 
            "Creativity", 
            "Pattern", 
            "Individuality", 
            "Structure", 
            "Repetition", 
            "Potentiality", 
            "Subsistence", 
            "Relatedness", 
            "Polarity", 
            "Wholeness"
        ]
    }
    
    fn get_position_description(&self, position: usize) -> Option<&'static str> {
        // TODO: Replace with authentic Bennett descriptions from original source
        // Research needed: Find Bennett's specific descriptions of all twelve Dodecad positions
        match position {
            0 => Some("[Research needed: Bennett's description of Autocracy]"),
            1 => Some("[Research needed: Bennett's description of Domination]"),
            2 => Some("[Research needed: Bennett's description of Creativity]"),
            3 => Some("[Research needed: Bennett's description of Pattern]"),
            4 => Some("[Research needed: Bennett's description of Individuality]"),
            5 => Some("[Research needed: Bennett's description of Structure]"),
            6 => Some("[Research needed: Bennett's description of Repetition]"),
            7 => Some("[Research needed: Bennett's description of Potentiality]"),
            8 => Some("[Research needed: Bennett's description of Subsistence]"),
            9 => Some("[Research needed: Bennett's description of Relatedness]"),
            10 => Some("[Research needed: Bennett's description of Polarity]"),
            11 => Some("[Research needed: Bennett's description of Wholeness]"),
            _ => None,
        }
    }
    
    fn get_connective_label(&self, i: usize, j: usize) -> Option<&'static str> {
        let _labels = self.get_canonical_labels();
        match (i, j) {
            // Row 0: Autocracy with all others
            (0, 1) => Some("Autocracy <> Domination"),
            (0, 2) => Some("Autocracy <> Creativity"),
            (0, 3) => Some("Autocracy <> Pattern"),
            (0, 4) => Some("Autocracy <> Individuality"),
            (0, 5) => Some("Autocracy <> Structure"),
            (0, 6) => Some("Autocracy <> Repetition"),
            (0, 7) => Some("Autocracy <> Potentiality"),
            (0, 8) => Some("Autocracy <> Subsistence"),
            (0, 9) => Some("Autocracy <> Relatedness"),
            (0, 10) => Some("Autocracy <> Polarity"),
            (0, 11) => Some("Autocracy <> Wholeness"),
            
            // Row 1: Domination with remaining
            (1, 2) => Some("Domination <> Creativity"),
            (1, 3) => Some("Domination <> Pattern"),
            (1, 4) => Some("Domination <> Individuality"),
            (1, 5) => Some("Domination <> Structure"),
            (1, 6) => Some("Domination <> Repetition"),
            (1, 7) => Some("Domination <> Potentiality"),
            (1, 8) => Some("Domination <> Subsistence"),
            (1, 9) => Some("Domination <> Relatedness"),
            (1, 10) => Some("Domination <> Polarity"),
            (1, 11) => Some("Domination <> Wholeness"),
            
            // Row 2: Creativity with remaining
            (2, 3) => Some("Creativity <> Pattern"),
            (2, 4) => Some("Creativity <> Individuality"),
            (2, 5) => Some("Creativity <> Structure"),
            (2, 6) => Some("Creativity <> Repetition"),
            (2, 7) => Some("Creativity <> Potentiality"),
            (2, 8) => Some("Creativity <> Subsistence"),
            (2, 9) => Some("Creativity <> Relatedness"),
            (2, 10) => Some("Creativity <> Polarity"),
            (2, 11) => Some("Creativity <> Wholeness"),
            
            // Row 3: Pattern with remaining
            (3, 4) => Some("Pattern <> Individuality"),
            (3, 5) => Some("Pattern <> Structure"),
            (3, 6) => Some("Pattern <> Repetition"),
            (3, 7) => Some("Pattern <> Potentiality"),
            (3, 8) => Some("Pattern <> Subsistence"),
            (3, 9) => Some("Pattern <> Relatedness"),
            (3, 10) => Some("Pattern <> Polarity"),
            (3, 11) => Some("Pattern <> Wholeness"),
            
            // Row 4: Individuality with remaining
            (4, 5) => Some("Individuality <> Structure"),
            (4, 6) => Some("Individuality <> Repetition"),
            (4, 7) => Some("Individuality <> Potentiality"),
            (4, 8) => Some("Individuality <> Subsistence"),
            (4, 9) => Some("Individuality <> Relatedness"),
            (4, 10) => Some("Individuality <> Polarity"),
            (4, 11) => Some("Individuality <> Wholeness"),
            
            // Row 5: Structure with remaining
            (5, 6) => Some("Structure <> Repetition"),
            (5, 7) => Some("Structure <> Potentiality"),
            (5, 8) => Some("Structure <> Subsistence"),
            (5, 9) => Some("Structure <> Relatedness"),
            (5, 10) => Some("Structure <> Polarity"),
            (5, 11) => Some("Structure <> Wholeness"),
            
            // Row 6: Repetition with remaining
            (6, 7) => Some("Repetition <> Potentiality"),
            (6, 8) => Some("Repetition <> Subsistence"),
            (6, 9) => Some("Repetition <> Relatedness"),
            (6, 10) => Some("Repetition <> Polarity"),
            (6, 11) => Some("Repetition <> Wholeness"),
            
            // Row 7: Potentiality with remaining
            (7, 8) => Some("Potentiality <> Subsistence"),
            (7, 9) => Some("Potentiality <> Relatedness"),
            (7, 10) => Some("Potentiality <> Polarity"),
            (7, 11) => Some("Potentiality <> Wholeness"),
            
            // Row 8: Subsistence with remaining
            (8, 9) => Some("Subsistence <> Relatedness"),
            (8, 10) => Some("Subsistence <> Polarity"),
            (8, 11) => Some("Subsistence <> Wholeness"),
            
            // Row 9: Relatedness with remaining
            (9, 10) => Some("Relatedness <> Polarity"),
            (9, 11) => Some("Relatedness <> Wholeness"),
            
            // Row 10: Polarity with remaining
            (10, 11) => Some("Polarity <> Wholeness"),
            
            _ => None,
        }
    }
    
    fn get_attribute_description(&self) -> &'static str {
        "Totality"
    }
    
    fn get_schema_name(&self) -> &'static str {
        "Bennett's Dodecad"
    }
    
    fn get_structure_name(&self) -> &'static str {
        "Dodecad"
    }
    
    fn get_position_count(&self) -> usize {
        12
    }
} 