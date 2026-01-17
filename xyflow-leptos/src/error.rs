//! Error types for XYFlow Leptos

use thiserror::Error;

/// Result type for XYFlow operations
pub type Result<T> = std::result::Result<T, FlowError>;

/// Errors that can occur in XYFlow operations
#[derive(Debug, Error)]
pub enum FlowError {
    /// Node not found in the flow
    #[error("Node not found: {0}")]
    NodeNotFound(String),

    /// Edge not found in the flow
    #[error("Edge not found: {0}")]
    EdgeNotFound(String),

    /// Invalid connection between nodes
    #[error("Invalid connection: cannot connect {0} to {1}")]
    InvalidConnection(String, String),

    /// Handle not found
    #[error("Handle not found: {0}")]
    HandleNotFound(String),

    /// DOM operation failed
    #[error("DOM operation failed: {0}")]
    DomError(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// Invalid viewport state
    #[error("Invalid viewport state: {0}")]
    InvalidViewport(String),

    /// Serialization error
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// Generic error
    #[error("{0}")]
    Other(String),
}

impl From<serde_json::Error> for FlowError {
    fn from(err: serde_json::Error) -> Self {
        FlowError::SerializationError(err.to_string())
    }
}
