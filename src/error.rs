use thiserror::Error;

/// Error types for the AWTRIX3 client
#[derive(Error, Debug)]
pub enum AwtrixError {
    /// HTTP request failed
    #[error("HTTP request failed")]
    Http(#[from] reqwest::Error),
    
    /// Invalid configuration
    #[error("Invalid configuration: {0}")]
    Config(String),
    
    /// Device not reachable
    #[error("Device '{device}' not reachable at {host}")]
    DeviceUnreachable { device: String, host: String },
    
    /// Invalid color format
    #[error("Invalid color format: {0}")]
    InvalidColor(String),
    
    /// Invalid icon ID
    #[error("Invalid icon ID: {0}")]
    InvalidIcon(u32),
    
    /// API error from device
    #[error("API error: {message} (code: {code})")]
    Api { message: String, code: u16 },
    
    /// Serialization/deserialization error
    #[error("Serialization error")]
    Serialization(#[from] serde_json::Error),
    
    /// IO error
    #[error("IO error")]
    Io(#[from] std::io::Error),
    
    /// URL parsing error
    #[error("Invalid URL")]
    Url(#[from] url::ParseError),
    
    /// Other errors
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// Result type alias for AWTRIX operations
pub type Result<T> = std::result::Result<T, AwtrixError>;