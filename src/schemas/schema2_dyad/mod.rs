pub mod jgb_dyad;

pub use jgb_dyad::BennettDyadSchema;

use crate::schemas::StructureSchema;

/// Available dyad schemas for dynamic selection
pub fn get_available_dyad_schemas() -> Vec<Box<dyn StructureSchema>> {
    vec![
        Box::new(BennettDyadSchema),
    ]
}

/// Interactive schema selection for dyads
pub fn select_dyad_schema() -> Box<dyn StructureSchema> {
    println!("\nUsing Bennett's Dyad Schema (Essence/Existence)");
    Box::new(BennettDyadSchema)
} 