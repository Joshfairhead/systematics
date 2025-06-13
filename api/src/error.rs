use thiserror::Error;

/// Result type for systematics operations
pub type Result<T> = std::result::Result<T, SystematicsError>;

/// Errors that can occur when working with systematic structures
#[derive(Error, Debug)]
pub enum SystematicsError {
    #[error("Invalid term count: expected {expected}, got {actual}")]
    InvalidTermCount { expected: usize, actual: usize },
    
    #[error("Invalid term at position {position}: {reason}")]
    InvalidTerm { position: usize, reason: String },
    
    #[error("Schema validation failed: {reason}")]
    SchemaValidation { reason: String },
    
    #[error("Structure validation failed: {reason}")]
    StructureValidation { reason: String },
    
    #[error("Builder error: {reason}")]
    Builder { reason: String },
    
    #[error("Serialization error: {0}")]
    Serialization(String),
    
    #[error("Deserialization error: {0}")]
    Deserialization(String),
    
    #[error("IO error: {source}")]
    Io {
        #[from]
        source: std::io::Error,
    },
    
    #[cfg(feature = "serde_support")]
    #[error("JSON error: {source}")]
    Json {
        #[from]
        source: serde_json::Error,
    },
} 