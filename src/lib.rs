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
pub mod models;
pub mod error;

// Re-exports for convenience
pub use client::{Client, ClientBuilder};
pub use models::{Notification, CustomApp, Settings, Color, Effect, Transition};
pub use error::{AwtrixError, Result};

/// Prelude module for easy imports
pub mod prelude {
    pub use crate::{Client, ClientBuilder, Notification, CustomApp, Result};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(env!("CARGO_PKG_VERSION"), "0.1.0");
    }
}