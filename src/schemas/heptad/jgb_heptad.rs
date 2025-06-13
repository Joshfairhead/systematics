use crate::schemas::StructureSchema;

/// JGB's Heptad Schema - Insight, Research, Design, Synthesis, Application, Delivery, Value
pub struct BennettHeptadSchema;

impl StructureSchema for BennettHeptadSchema {
    fn get_canonical_labels(&self) -> &[&'static str] {
        &["Insight", "Research", "Design", "Synthesis", "Application", "Delivery", "Value"]
    }
    
    fn get_position_description(&self, pos: usize) -> Option<&'static str> {
        match pos {
            0 => Some("Core insight and understanding"),
            1 => Some("Research and investigation"),
            2 => Some("Design and planning"),
            3 => Some("Synthesis and integration"),
            4 => Some("Application and implementation"),
            5 => Some("Delivery and execution"),
            6 => Some("Value and outcome"),
            _ => None,
        }
    }
    
    fn get_connective_label(&self, i: usize, j: usize) -> Option<&'static str> {
        match (i, j) {
            (0, 1) => Some("Insight <> Research"),
            (0, 2) => Some("Insight <> Design"),
            (0, 3) => Some("Insight <> Synthesis"),
            (0, 4) => Some("Insight <> Application"),
            (0, 5) => Some("Insight <> Delivery"),
            (0, 6) => Some("Insight <> Value"),
            (1, 2) => Some("Research <> Design"),
            (1, 3) => Some("Research <> Synthesis"),
            (1, 4) => Some("Research <> Application"),
            (1, 5) => Some("Research <> Delivery"),
            (1, 6) => Some("Research <> Value"),
            (2, 3) => Some("Design <> Synthesis"),
            (2, 4) => Some("Design <> Application"),
            (2, 5) => Some("Design <> Delivery"),
            (2, 6) => Some("Design <> Value"),
            (3, 4) => Some("Synthesis <> Application"),
            (3, 5) => Some("Synthesis <> Delivery"),
            (3, 6) => Some("Synthesis <> Value"),
            (4, 5) => Some("Application <> Delivery"),
            (4, 6) => Some("Application <> Value"),
            (5, 6) => Some("Delivery <> Value"),
            _ => None,
        }
    }
    
    fn get_attribute_description(&self) -> &'static str {
        "Generative power"
    }
    
    fn get_schema_name(&self) -> &'static str {
        "JGB's Heptad"
    }
    
    fn get_structure_name(&self) -> &'static str {
        "Heptad"
    }
    
    fn get_position_count(&self) -> usize {
        7
    }
} 