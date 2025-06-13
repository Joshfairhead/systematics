pub mod jgb_tetrad;

pub use jgb_tetrad::BennettTetradSchema;

use crate::schemas::StructureSchema;


/// Available tetrad schemas for dynamic selection
#[allow(dead_code)]
pub fn get_available_tetrad_schemas() -> Vec<Box<dyn StructureSchema>> {
    vec![
        Box::new(BennettTetradSchema),
    ]
}

/// Interactive schema selection for tetrads
pub fn select_tetrad_schema() -> Box<dyn StructureSchema> {
    Box::new(BennettTetradSchema)
} 