//! Error types for the Cocoanut crate

use thiserror::Error;

/// Result type alias for Cocoanut operations
pub type Result<T> = std::result::Result<T, CocoanutError>;

/// Errors that can occur in Cocoanut operations
#[derive(Error, Debug)]
pub enum CocoanutError {
    /// Failed to initialize the application
    #[error("Failed to initialize application: {0}")]
    ApplicationInitFailed(String),
    
    /// Failed to create a window
    #[error("Failed to create window: {0}")]
    WindowCreationFailed(String),
    
    /// Failed to create a menu
    #[error("Failed to create menu: {0}")]
    MenuCreationFailed(String),
    
    /// Failed to create a control
    #[error("Failed to create control: {0}")]
    ControlCreationFailed(String),
    
    /// Invalid parameter provided
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),
    
    /// System error from macOS
    #[error("System error: {0}")]
    SystemError(String),
    
    /// Threading error
    #[error("Threading error: {0}")]
    ThreadingError(String),
    
    /// Drawing error
    #[error("Drawing error: {0}")]
    DrawingError(String),
    
    /// Event handling error
    #[error("Event handling error: {0}")]
    EventError(String),
    
    /// Generic error with message
    #[error("Cocoanut error: {0}")]
    Generic(String),
}

impl From<String> for CocoanutError {
    fn from(msg: String) -> Self {
        CocoanutError::Generic(msg)
    }
}

impl From<&str> for CocoanutError {
    fn from(msg: &str) -> Self {
        CocoanutError::Generic(msg.to_string())
    }
}

impl From<tokio::sync::mpsc::error::SendError<futures::future::BoxFuture<'static, ()>>> for CocoanutError {
    fn from(err: tokio::sync::mpsc::error::SendError<futures::future::BoxFuture<'static, ()>>) -> Self {
        CocoanutError::ThreadingError(format!("Failed to send async operation: {}", err))
    }
}

impl From<tokio::sync::oneshot::error::RecvError> for CocoanutError {
    fn from(err: tokio::sync::oneshot::error::RecvError) -> Self {
        CocoanutError::ThreadingError(format!("Failed to receive async result: {}", err))
    }
}

impl From<std::ffi::NulError> for CocoanutError {
    fn from(err: std::ffi::NulError) -> Self {
        CocoanutError::InvalidParameter(format!("Null byte in string: {}", err))
    }
}
