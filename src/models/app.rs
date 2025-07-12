use crate::models::color::Color;
use serde::{Deserialize, Serialize};

/// Represents a custom app on the AWTRIX3 device
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomApp {
    /// App text to display
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// Icon ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<u32>,

    /// Text color
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,

    /// Display duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,

    /// Progress bar (0-100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<u8>,

    /// Progress bar color
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_c: Option<Color>,

    /// Background color for progress
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_bc: Option<Color>,

    /// Rainbow effect
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rainbow: Option<bool>,

    /// Remove app after timeout
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifetime: Option<u32>,

    /// Save to flash memory
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save: Option<bool>,

    /// App position in the loop
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos: Option<u32>,
}

/// Information about an app in the loop
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppInfo {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl CustomApp {
    pub fn new() -> Self {
        Self {
            text: None,
            icon: None,
            color: None,
            duration: None,
            progress: None,
            progress_c: None,
            progress_bc: None,
            rainbow: None,
            lifetime: None,
            save: None,
            pos: None,
        }
    }
}

impl Default for CustomApp {
    fn default() -> Self {
        Self::new()
    }
}
