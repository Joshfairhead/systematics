pub mod jgb_dyad;

pub use jgb_dyad::BennettDyadSchema;

use crate::schemas::StructureSchema;

/// Available dyad schemas for dynamic selection
#[allow(dead_code)]
pub fn get_available_dyad_schemas() -> Vec<Box<dyn StructureSchema>> {
    vec![
        Box::new(BennettDyadSchema),
    ]
}

/// Interactive schema selection for dyads
pub fn select_dyad_schema() -> Box<dyn StructureSchema> {
    Box::new(BennettDyadSchema)
} 