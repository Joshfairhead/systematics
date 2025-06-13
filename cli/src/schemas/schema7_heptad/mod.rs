pub mod jgb_heptad;

pub use jgb_heptad::BennettHeptadSchema;

use crate::schemas::StructureSchema;

/// Available heptad schemas for dynamic selection
#[allow(dead_code)]
pub fn get_available_heptad_schemas() -> Vec<Box<dyn StructureSchema>> {
    vec![
        Box::new(BennettHeptadSchema),
    ]
}

/// Interactive schema selection for heptads
pub fn select_heptad_schema() -> Box<dyn StructureSchema> {
    Box::new(BennettHeptadSchema)
} 