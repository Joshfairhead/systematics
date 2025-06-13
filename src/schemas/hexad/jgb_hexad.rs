use crate::schemas::StructureSchema;

/// Bennett's Hexad Schema - Resources, Values, Options, Criteria, Facts, Priorities
pub struct BennettHexadSchema;

impl StructureSchema for BennettHexadSchema {
    fn get_canonical_labels(&self) -> &[&'static str] {
        &["Resources", "Values", "Options", "Criteria", "Facts", "Priorities"]
    }
    
    fn get_position_description(&self, pos: usize) -> Option<&'static str> {
        match pos {
            0 => Some("Available resources and assets"),
            1 => Some("Core values and principles"),
            2 => Some("Possible options and alternatives"),
            3 => Some("Selection criteria and standards"),
            4 => Some("Known facts and information"),
            5 => Some("Key priorities and importance"),
            _ => None,
        }
    }
    
    fn get_connective_label(&self, i: usize, j: usize) -> Option<&'static str> {
        match (i, j) {
            (0, 1) => Some("Resources <> Values"),
            (0, 2) => Some("Resources <> Options"),
            (0, 3) => Some("Resources <> Criteria"),
            (0, 4) => Some("Resources <> Facts"),
            (0, 5) => Some("Resources <> Priorities"),
            (1, 2) => Some("Values <> Options"),
            (1, 3) => Some("Values <> Criteria"),
            (1, 4) => Some("Values <> Facts"),
            (1, 5) => Some("Values <> Priorities"),
            (2, 3) => Some("Options <> Criteria"),
            (2, 4) => Some("Options <> Facts"),
            (2, 5) => Some("Options <> Priorities"),
            (3, 4) => Some("Criteria <> Facts"),
            (3, 5) => Some("Criteria <> Priorities"),
            (4, 5) => Some("Facts <> Priorities"),
            _ => None,
        }
    }
    
    fn get_attribute_description(&self) -> &'static str {
        "Coalescence"
    }
    
    fn get_schema_name(&self) -> &'static str {
        "Bennett's Hexad"
    }
    
    fn get_structure_name(&self) -> &'static str {
        "Hexad"
    }
    
    fn get_position_count(&self) -> usize {
        6
    }
} 