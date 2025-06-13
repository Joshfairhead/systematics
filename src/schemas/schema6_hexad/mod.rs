pub mod jgb_hexad;

pub use jgb_hexad::BennettHexadSchema;

use crate::schemas::StructureSchema;

/// Available hexad schemas for dynamic selection
pub fn get_available_hexad_schemas() -> Vec<Box<dyn StructureSchema>> {
    vec![
        Box::new(BennettHexadSchema),
    ]
}

/// Interactive schema selection for hexads
pub fn select_hexad_schema() -> Box<dyn StructureSchema> {
    println!("\nUsing Bennett's Hexad Schema (Resources, Values, Options, Criteria, Facts, Priorities)");
    Box::new(BennettHexadSchema)
} 