use crate::schemas::StructureSchema;

/// Bennett's Triad Schema - Will, Function, Being
pub struct BennettTriadSchema;

impl StructureSchema for BennettTriadSchema {
    fn get_canonical_labels(&self) -> &[&'static str] {
        &["Will", "Function", "Being"]
    }
    fn get_position_description(&self, pos: usize) -> Option<&'static str> {
        // TODO: Replace with authentic Bennett descriptions from original source
        // Research needed: Find Bennett's specific descriptions of Function, Being, and Will in the Triad
        match pos {
            0 => Some("[Research needed: Bennett's description of Function]"),
            1 => Some("[Research needed: Bennett's description of Being]"),
            2 => Some("[Research needed: Bennett's description of Will]"),
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