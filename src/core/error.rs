use thiserror::Error;

/// Core error type for the docgraph library
#[derive(Debug, Error)]
pub enum Error {
    /// IO error
    #[error("io error")]
    Io(#[from] std::io::Error),

    /// Invalid configuration
    #[error("invalid configuration: {0}")]
    InvalidConfig(String),

    /// Parse error
    #[error("parse error at {path}:{line}: {message}")]
    Parse {
        path: String,
        line: usize,
        message: String,
    },

    /// Validation error
    #[error("validation failed: {0}")]
    Validation(String),

    /// File not found
    #[error("file not found: {0}")]
    FileNotFound(String),

    /// Invalid node type
    #[error("invalid node type: {0}")]
    InvalidNodeType(String),

    /// Regex error
    #[error("regex error: {0}")]
    Regex(#[from] regex::Error),

    /// TOML deserialization error
    #[error("toml error: {0}")]
    Toml(#[from] toml::de::Error),

    /// JSON serialization/deserialization error
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    /// Generic error with message
    #[error("{0}")]
    Other(String),
}

/// Result type alias using core::Error
pub type Result<T> = std::result::Result<T, Error>;
