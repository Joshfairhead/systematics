use crate::schemas::StructureSchema;

/// Bennett's Dyad Schema - Essence and Existence
pub struct BennettDyadSchema;

impl StructureSchema for BennettDyadSchema {
    fn get_canonical_labels(&self) -> &[&'static str] {
        &["Essence", "Existence"]
    }
    fn get_position_description(&self, pos: usize) -> Option<&'static str> {
        match pos {
            0 => Some("The essential nature or being of a thing"),
            1 => Some("The state of being, actuality, or manifestation"),
            _ => None,
        }
    }
    fn get_connective_label(&self, _i: usize, _j: usize) -> Option<&'static str> {
        None // No connectives for dyad
    }
    fn get_attribute_description(&self) -> &'static str {
        "Complementarity, polarity or force"
    }
    fn get_schema_name(&self) -> &'static str {
        "Bennett's Dyad"
    }
    fn get_structure_name(&self) -> &'static str {
        "Dyad"
    }
    fn get_position_count(&self) -> usize {
        2
    }
} 