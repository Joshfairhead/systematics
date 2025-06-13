pub mod jgb_monad;

pub use jgb_monad::BennettMonadSchema;

use crate::schemas::StructureSchema;

/// Available monad schemas for dynamic selection
#[allow(dead_code)]
pub fn get_available_monad_schemas() -> Vec<Box<dyn StructureSchema>> {
    vec![
        Box::new(BennettMonadSchema),
    ]
}

/// Interactive schema selection for monads
pub fn select_monad_schema() -> Box<dyn StructureSchema> {
    Box::new(BennettMonadSchema)
} 