pub mod jgb_pentad;

pub use jgb_pentad::BennettPentadSchema;

use crate::schemas::StructureSchema;

/// Available pentad schemas for dynamic selection
#[allow(dead_code)]
pub fn get_available_pentad_schemas() -> Vec<Box<dyn StructureSchema>> {
    vec![
        Box::new(BennettPentadSchema),
    ]
}

/// Interactive schema selection for pentads
pub fn select_pentad_schema() -> Box<dyn StructureSchema> {
    Box::new(BennettPentadSchema)
} 