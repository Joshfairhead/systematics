pub mod jgb_octad;

pub use jgb_octad::BennettOctadSchema;

use crate::schemas::StructureSchema;

/// Available octad schemas for dynamic selection
pub fn get_available_octad_schemas() -> Vec<Box<dyn StructureSchema>> {
    vec![
        Box::new(BennettOctadSchema),
    ]
}

/// Interactive schema selection for octads
pub fn select_octad_schema() -> Box<dyn StructureSchema> {
    println!("\nUsing JGB's Octad Schema (Smallest Significant Holon, Critical Functions, Supportive Platform, Necessary Resourcing, Integrative Totality, Inherent Values, Intrinsic Nature, Organisational Modes)");
    Box::new(BennettOctadSchema)
} 