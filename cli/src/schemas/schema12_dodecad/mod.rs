pub mod jgb_dodecad;

pub use jgb_dodecad::BennettDodecadSchema;

use crate::schemas::StructureSchema;

/// Available dodecad schemas for dynamic selection
#[allow(dead_code)]
pub fn get_available_dodecad_schemas() -> Vec<Box<dyn StructureSchema>> {
    vec![
        Box::new(BennettDodecadSchema),
    ]
}

/// Interactive schema selection for dodecads
pub fn select_dodecad_schema() -> Box<dyn StructureSchema> {
    Box::new(BennettDodecadSchema)
} 