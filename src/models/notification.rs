use serde::{Deserialize, Serialize};
use crate::models::color::Color;

/// Represents a notification to be sent to the AWTRIX3 device
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    /// The text to display
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    
    /// Icon ID to display
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<u32>,
    
    /// Text color
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    
    /// Duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,
    
    /// Sound to play
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound: Option<String>,
    
    /// RTTTL sound string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtttl: Option<String>,
    
    /// Loop sound
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loop_sound: Option<bool>,
    
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
    
    /// Stack notification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack: Option<bool>,
    
    /// Hold notification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hold: Option<bool>,
    
    /// Wake up display
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wakeup: Option<bool>,
    
    /// Disable scrolling
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_scroll: Option<bool>,
    
    /// Scroll speed percentage
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scroll_speed: Option<u32>,
    
    /// Effect name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,
    
    /// Effect settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect_settings: Option<EffectSettings>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EffectSettings {
    pub speed: Option<u32>,
    pub palette: Option<String>,
    pub blend: Option<bool>,
}

/// Builder for creating notifications
pub struct NotificationBuilder {
    notification: Notification,
}

impl NotificationBuilder {
    pub fn new() -> Self {
        Self {
            notification: Notification {
                text: None,
                icon: None,
                color: None,
                duration: None,
                sound: None,
                rtttl: None,
                loop_sound: None,
                progress: None,
                progress_c: None,
                progress_bc: None,
                rainbow: None,
                stack: None,
                hold: None,
                wakeup: None,
                no_scroll: None,
                scroll_speed: None,
                effect: None,
                effect_settings: None,
            },
        }
    }
    
    pub fn text<S: Into<String>>(mut self, text: S) -> Self {
        self.notification.text = Some(text.into());
        self
    }
    
    pub fn icon(mut self, icon: u32) -> Self {
        self.notification.icon = Some(icon);
        self
    }
    
    pub fn color<C: Into<Color>>(mut self, color: C) -> Self {
        self.notification.color = Some(color.into());
        self
    }
    
    pub fn duration(mut self, duration: u32) -> Self {
        self.notification.duration = Some(duration);
        self
    }
    
    pub fn sound<S: Into<String>>(mut self, sound: S) -> Self {
        self.notification.sound = Some(sound.into());
        self
    }
    
    pub fn progress(mut self, progress: u8) -> Self {
        self.notification.progress = Some(progress.clamp(0, 100));
        self
    }
    
    pub fn hold(mut self, hold: bool) -> Self {
        self.notification.hold = Some(hold);
        self
    }
    
    pub fn wakeup(mut self, wakeup: bool) -> Self {
        self.notification.wakeup = Some(wakeup);
        self
    }
    
    pub fn build(self) -> Notification {
        self.notification
    }
}

impl Notification {
    pub fn builder() -> NotificationBuilder {
        NotificationBuilder::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_notification_builder() {
        let notification = Notification::builder()
            .text("Test")
            .icon(1234)
            .duration(10)
            .build();
        
        assert_eq!(notification.text, Some("Test".to_string()));
        assert_eq!(notification.icon, Some(1234));
        assert_eq!(notification.duration, Some(10));
    }
    
    #[test]
    fn test_progress_clamping() {
        let notification = Notification::builder()
            .progress(150)
            .build();
        
        assert_eq!(notification.progress, Some(100));
    }
}