pub mod jgb_heptad;

pub use jgb_heptad::BennettHeptadSchema;

use crate::schemas::StructureSchema;

/// Available heptad schemas for dynamic selection
pub fn get_available_heptad_schemas() -> Vec<Box<dyn StructureSchema>> {
    vec![
        Box::new(BennettHeptadSchema),
    ]
}

/// Interactive schema selection for heptads
pub fn select_heptad_schema() -> Box<dyn StructureSchema> {
    println!("\nUsing JGB's Heptad Schema (Insight, Research, Design, Synthesis, Application, Delivery, Value)");
    Box::new(BennettHeptadSchema)
} 