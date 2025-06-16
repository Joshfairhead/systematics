use thiserror::Error;

/// Library-specific error types
#[derive(Error, Debug)]
pub enum LibraryError {
    #[error("Schema not found: {0}")]
    SchemaNotFound(String),
    
    #[error("Invalid schema configuration: {0}")]
    InvalidConfiguration(String),
    
    #[error("Provider error: {0}")]
    ProviderError(String),
}

pub type Result<T> = std::result::Result<T, LibraryError>; 