use crate::schemas::StructureSchema;

/// Bennett's Tetrad Schema - Ground, Ideal, Instrumental, Directive
pub struct BennettTetradSchema;

impl StructureSchema for BennettTetradSchema {
    fn get_canonical_labels(&self) -> &[&'static str] {
        &["Ground", "Ideal", "Instrumental", "Directive"]
    }
    
    fn get_position_description(&self, pos: usize) -> Option<&'static str> {
        match pos {
            0 => Some("The foundational basis or substrate"),
            1 => Some("The perfect form or aspiration"),
            2 => Some("The means or tools for transformation"),
            3 => Some("The guiding purpose or direction"),
            _ => None,
        }
    }
    
    fn get_connective_label(&self, i: usize, j: usize) -> Option<&'static str> {
        match (i, j) {
            (0, 1) => Some("Motivational imperative"),          //Ground-Ideal 
            (0, 2) => Some("Technical power"),                  //Ground-Instrumental 
            (0, 3) => Some("Material Mastery"),                 //Ground-Directive 
            (1, 2) => Some("Effectual compatibility"),          //Ideal-Instrumental 
            (1, 3) => Some("Receptive regard"),                 //Ideal-Directive 
            (2, 3) => Some("Demonstrable activity"),            //Instrumental-Directive 
            _ => None,
        }
    }
    
    fn get_attribute_description(&self) -> &'static str {
        "A Field of Action"
    }
    
    fn get_schema_name(&self) -> &'static str {
        "Bennett's Tetrad"
    }
    
    fn get_structure_name(&self) -> &'static str {
        "Tetrad"
    }
    
    fn get_position_count(&self) -> usize {
        4
    }
} 