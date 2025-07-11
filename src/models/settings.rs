use serde::{Deserialize, Serialize};
use crate::models::color::Color;

/// Device settings structure
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    /// Matrix brightness (0-255)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brightness: Option<u8>,
    
    /// Automatic brightness control
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_brightness: Option<bool>,
    
    /// Automatic app switching
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_transition: Option<bool>,
    
    /// App display duration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_time: Option<u32>,
    
    /// Transition effect
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transition: Option<String>,
    
    /// Transition time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transition_time: Option<u32>,
    
    /// Global text color
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_color: Option<Color>,
    
    /// Time app settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_app: Option<TimeAppSettings>,
    
    /// Date app settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_app: Option<DateAppSettings>,
    
    /// Temperature unit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temp_unit: Option<String>,
    
    /// Scroll speed percentage
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scroll_speed: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeAppSettings {
    /// Time format (0-5)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<u8>,
    
    /// Show weekday
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_weekday: Option<bool>,
    
    /// Calendar header color
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cal_header_color: Option<Color>,
    
    /// Calendar body color
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cal_body_color: Option<Color>,
    
    /// Calendar text color
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cal_text_color: Option<Color>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DateAppSettings {
    /// Show date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    
    /// Date format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            brightness: None,
            auto_brightness: None,
            auto_transition: None,
            app_time: None,
            transition: None,
            transition_time: None,
            text_color: None,
            time_app: None,
            date_app: None,
            temp_unit: None,
            scroll_speed: None,
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}