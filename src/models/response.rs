use serde::{Deserialize, Serialize};
use crate::models::app::AppInfo;

/// Device statistics response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    /// Device uptime in seconds
    pub uptime: u64,
    
    /// WiFi signal strength
    pub wifi_signal: i8,
    
    /// Free heap memory
    pub heap: u32,
    
    /// Temperature in Celsius (if sensor available)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    
    /// Humidity percentage (if sensor available)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub humidity: Option<f32>,
    
    /// LDR (light sensor) value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ldr: Option<u16>,
    
    /// Light level
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lux: Option<f32>,
    
    /// Battery percentage (if battery powered)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub battery: Option<u8>,
    
    /// Matrix on/off state
    pub matrix: bool,
    
    /// Current app name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_app: Option<String>,
    
    /// Indicator states
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indicators: Option<IndicatorStates>,
}

/// Indicator LED states
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndicatorStates {
    pub indicator1: bool,
    pub indicator2: bool,
    pub indicator3: bool,
}

/// App loop information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopInfo {
    pub apps: Vec<AppInfo>,
    pub current: Option<String>,
}

/// Version information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionInfo {
    pub version: String,
    pub build: Option<String>,
    pub git_hash: Option<String>,
}

/// Effects list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectsList {
    pub effects: Vec<String>,
}

/// Transitions list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionsList {
    pub transitions: Vec<String>,
}

/// Screen data response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenData {
    /// Matrix width
    pub width: u8,
    
    /// Matrix height
    pub height: u8,
    
    /// Pixel data (RGB values)
    pub pixels: Vec<Vec<Vec<u8>>>,
}