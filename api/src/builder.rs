use crate::error::Result;

/// Generic builder trait for all systematic structures
pub trait StructureBuilder<T> {
    /// Build the structure, returning a Result
    fn build(self) -> Result<T>;
}

// The specific builders (MonadBuilder, DyadBuilder, etc.) will implement this trait 