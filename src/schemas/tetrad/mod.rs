pub mod bennett;

pub use bennett::BennettTetradSchema;

use crate::schemas::StructureSchema;


/// Available tetrad schemas for dynamic selection
pub fn get_available_tetrad_schemas() -> Vec<Box<dyn StructureSchema>> {
    vec![
        Box::new(BennettTetradSchema),
    ]
}

/// Interactive schema selection for tetrads
pub fn select_tetrad_schema() -> Box<dyn StructureSchema> {
    println!("\nUsing Bennett's Tetrad Schema (Ground, Ideal, Instrumental, Directive)");
    Box::new(BennettTetradSchema)
} 