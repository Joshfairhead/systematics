use crate::schemas::StructureSchema;

/// Bennett's Octad Schema - Smallest Significant Holon, Critical Functions, Supportive Platform, Necessary Resourcing, Integrative Totality, Inherent Values, Intrinsic Nature, Organisational Modes
pub struct BennettOctadSchema;

impl StructureSchema for BennettOctadSchema {
    fn get_canonical_labels(&self) -> &[&'static str] {
        &[
            "Smallest Significant Holon", 
            "Critical Functions", 
            "Supportive Platform", 
            "Necessary Resourcing", 
            "Integrative Totality", 
            "Inherent Values", 
            "Intrinsic Nature", 
            "Organisational Modes"
        ]
    }
    
    fn get_position_description(&self, pos: usize) -> Option<&'static str> {
        match pos {
            0 => Some("The smallest significant holon"),
            1 => Some("Critical functions and processes"),
            2 => Some("Supportive platform and foundation"),
            3 => Some("Necessary resourcing and support"),
            4 => Some("Integrative totality and wholeness"),
            5 => Some("Inherent values and principles"),
            6 => Some("Intrinsic nature and essence"),
            7 => Some("Organisational modes and structures"),
            _ => None,
        }
    }
    
    fn get_connective_label(&self, i: usize, j: usize) -> Option<&'static str> {
        match (i, j) {
            (0, 1) => Some("Smallest Significant Holon <> Critical Functions"),
            (0, 2) => Some("Smallest Significant Holon <> Supportive Platform"),
            (0, 3) => Some("Smallest Significant Holon <> Necessary Resourcing"),
            (0, 4) => Some("Smallest Significant Holon <> Integrative Totality"),
            (0, 5) => Some("Smallest Significant Holon <> Inherent Values"),
            (0, 6) => Some("Smallest Significant Holon <> Intrinsic Nature"),
            (0, 7) => Some("Smallest Significant Holon <> Organisational Modes"),
            (1, 2) => Some("Critical Functions <> Supportive Platform"),
            (1, 3) => Some("Critical Functions <> Necessary Resourcing"),
            (1, 4) => Some("Critical Functions <> Integrative Totality"),
            (1, 5) => Some("Critical Functions <> Inherent Values"),
            (1, 6) => Some("Critical Functions <> Intrinsic Nature"),
            (1, 7) => Some("Critical Functions <> Organisational Modes"),
            (2, 3) => Some("Supportive Platform <> Necessary Resourcing"),
            (2, 4) => Some("Supportive Platform <> Integrative Totality"),
            (2, 5) => Some("Supportive Platform <> Inherent Values"),
            (2, 6) => Some("Supportive Platform <> Intrinsic Nature"),
            (2, 7) => Some("Supportive Platform <> Organisational Modes"),
            (3, 4) => Some("Necessary Resourcing <> Integrative Totality"),
            (3, 5) => Some("Necessary Resourcing <> Inherent Values"),
            (3, 6) => Some("Necessary Resourcing <> Intrinsic Nature"),
            (3, 7) => Some("Necessary Resourcing <> Organisational Modes"),
            (4, 5) => Some("Integrative Totality <> Inherent Values"),
            (4, 6) => Some("Integrative Totality <> Intrinsic Nature"),
            (4, 7) => Some("Integrative Totality <> Organisational Modes"),
            (5, 6) => Some("Inherent Values <> Intrinsic Nature"),
            (5, 7) => Some("Inherent Values <> Organisational Modes"),
            (6, 7) => Some("Intrinsic Nature <> Organisational Modes"),
            _ => None,
        }
    }
    
    fn get_attribute_description(&self) -> &'static str {
        "Completedness"
    }
    
    fn get_schema_name(&self) -> &'static str {
        "Bennett's Octad"
    }
    
    fn get_structure_name(&self) -> &'static str {
        "Octad"
    }
    
    fn get_position_count(&self) -> usize {
        8
    }
} 