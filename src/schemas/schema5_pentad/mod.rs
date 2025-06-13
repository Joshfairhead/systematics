pub mod jgb_pentad;

pub use jgb_pentad::BennettPentadSchema;

use crate::schemas::StructureSchema;

/// Available pentad schemas for dynamic selection
pub fn get_available_pentad_schemas() -> Vec<Box<dyn StructureSchema>> {
    vec![
        Box::new(BennettPentadSchema),
    ]
}

/// Interactive schema selection for pentads
pub fn select_pentad_schema() -> Box<dyn StructureSchema> {
    println!("\nUsing Bennett's Pentad Schema (Quintessence, Higher Potential, Lower Potential, Purpose, Source)");
    Box::new(BennettPentadSchema)
} 