pub mod jgb_triad;

pub use jgb_triad::BennettTriadSchema;

use crate::schemas::StructureSchema;

/// Available triad schemas for dynamic selection
pub fn get_available_triad_schemas() -> Vec<Box<dyn StructureSchema>> {
    vec![
        Box::new(BennettTriadSchema),
    ]
}

/// Interactive schema selection for triads
pub fn select_triad_schema() -> Box<dyn StructureSchema> {
    println!("\nUsing Bennett's Triad Schema (Will/Function/Being)");
    Box::new(BennettTriadSchema)
} 