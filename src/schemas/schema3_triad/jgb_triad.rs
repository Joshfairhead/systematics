use crate::schemas::StructureSchema;

/// Bennett's Triad Schema - Will, Function, Being
pub struct BennettTriadSchema;

impl StructureSchema for BennettTriadSchema {
    fn get_canonical_labels(&self) -> &[&'static str] {
        &["Will", "Function", "Being"]
    }
    fn get_position_description(&self, pos: usize) -> Option<&'static str> {
        match pos {
            0 => Some("The active, initiating principle"),
            1 => Some("The passive, receptive principle"),
            2 => Some("The reconciling, harmonizing principle"),
            _ => None,
        }
    }
    fn get_connective_label(&self, _i: usize, _j: usize) -> Option<&'static str> {
        None // No connectives for triad
    }
    fn get_attribute_description(&self) -> &'static str {
        "Dynamism, relation, will"
    }
    fn get_schema_name(&self) -> &'static str {
        "Bennett's Triad"
    }
    fn get_structure_name(&self) -> &'static str {
        "Triad"
    }
    fn get_position_count(&self) -> usize {
        3
    }
} 