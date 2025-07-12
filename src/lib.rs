//! # AWTRIX3 Rust Client
//!
//! A modern, async Rust client for AWTRIX3 LED matrix displays.
//!
//! ## Quick Start
//! ```no_run
//! use awtrix3::{Client, Notification};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Client::new("192.168.1.100")?;
//!     
//!     client.notify(
//!         Notification::builder()
//!             .text("Hello, World!")
//!             .icon(1234)
//!             .color("#FF0000")
//!             .build()
//!     ).await?;
//!     
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod error;
pub mod models;

// Export utility modules
pub mod config;
pub mod utils;

// Re-exports for convenience
pub use client::{Client, ClientBuilder};
pub use error::{AwtrixError, Result};
pub use models::{Color, CustomApp, Effect, Notification, Settings, Transition};

/// Prelude module for easy imports
pub mod prelude {
    pub use crate::{Client, ClientBuilder, CustomApp, Notification, Result};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(env!("CARGO_PKG_VERSION"), "1.0.0");
    }
}
