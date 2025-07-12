use crate::error::{AwtrixError, Result};
use crate::models::*;
use reqwest::Response;
use serde_json::Value;
// url::Url is used through reqwest

impl super::Client {
    /// Make a GET request to the specified endpoint
    pub async fn get(&self, endpoint: &str) -> Result<Response> {
        let url = self.build_url(endpoint)?;

        let response = self.client.get(url).send().await?;

        self.handle_response(response).await
    }

    /// Make a POST request with JSON payload
    pub async fn post_json<T: serde::Serialize>(
        &self,
        endpoint: &str,
        payload: &T,
    ) -> Result<Response> {
        let url = self.build_url(endpoint)?;

        let response = self.client.post(url).json(payload).send().await?;

        self.handle_response(response).await
    }

    /// Make a POST request without payload
    pub async fn post(&self, endpoint: &str) -> Result<Response> {
        let url = self.build_url(endpoint)?;

        let response = self.client.post(url).send().await?;

        self.handle_response(response).await
    }

    /// Handle HTTP response and check for errors
    async fn handle_response(&self, response: Response) -> Result<Response> {
        let status = response.status();

        if status.is_success() {
            Ok(response)
        } else {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());

            Err(AwtrixError::Api {
                message: error_text,
                code: status.as_u16(),
            })
        }
    }

    /// Parse JSON response
    pub async fn parse_json<T: serde::de::DeserializeOwned>(response: Response) -> Result<T> {
        let json = response.json::<T>().await?;
        Ok(json)
    }

    /// Get JSON response as Value
    pub async fn get_json_value(response: Response) -> Result<Value> {
        let json = response.json::<Value>().await?;
        Ok(json)
    }
}

// API method implementations
impl super::Client {
    /// Set device power state
    pub async fn set_power(&self, power_on: bool) -> Result<()> {
        let payload = serde_json::json!({ "power": power_on });
        self.post_json("/api/power", &payload).await?;
        Ok(())
    }

    /// Set sleep mode
    pub async fn set_sleep(&self, duration: u32) -> Result<()> {
        let payload = serde_json::json!({ "sleep": duration });
        self.post_json("/api/sleep", &payload).await?;
        Ok(())
    }

    /// Send notification
    pub async fn notify(&self, notification: Notification) -> Result<()> {
        self.post_json("/api/notify", &notification).await?;
        Ok(())
    }

    /// Dismiss current notification
    pub async fn dismiss_notification(&self) -> Result<()> {
        self.post("/api/notify/dismiss").await?;
        Ok(())
    }

    /// Get device statistics
    pub async fn get_stats(&self) -> Result<Stats> {
        let response = self.get("/api/stats").await?;
        Self::parse_json(response).await
    }

    /// Get device version
    pub async fn get_version(&self) -> Result<String> {
        let response = self.get("/version").await?;
        let text = response.text().await?;
        Ok(text)
    }

    /// Get available effects
    pub async fn get_effects(&self) -> Result<Vec<String>> {
        let response = self.get("/api/effects").await?;
        let json = Self::get_json_value(response).await?;

        if let Some(effects) = json.as_array() {
            Ok(effects
                .iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect())
        } else {
            Ok(vec![])
        }
    }

    /// Get available transitions
    pub async fn get_transitions(&self) -> Result<Vec<String>> {
        let response = self.get("/api/transitions").await?;
        let json = Self::get_json_value(response).await?;

        if let Some(transitions) = json.as_array() {
            Ok(transitions
                .iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect())
        } else {
            Ok(vec![])
        }
    }

    /// Get apps in loop
    pub async fn get_apps(&self) -> Result<LoopInfo> {
        let response = self.get("/api/loop").await?;
        Self::parse_json(response).await
    }

    /// Switch to next app
    pub async fn next_app(&self) -> Result<()> {
        self.post("/api/nextapp").await?;
        Ok(())
    }

    /// Switch to previous app
    pub async fn previous_app(&self) -> Result<()> {
        self.post("/api/previousapp").await?;
        Ok(())
    }

    /// Switch to specific app
    pub async fn switch_app(&self, name: &str) -> Result<()> {
        let payload = serde_json::json!({ "name": name });
        self.post_json("/api/switch", &payload).await?;
        Ok(())
    }

    /// Create or update custom app
    pub async fn create_custom_app(&self, name: &str, app: &CustomApp) -> Result<()> {
        let url = format!("/api/custom?name={}", name);
        self.post_json(&url, app).await?;
        Ok(())
    }

    /// Delete custom app
    pub async fn delete_custom_app(&self, name: &str) -> Result<()> {
        // Send empty payload to delete
        let url = format!("/api/custom?name={}", name);
        let empty = serde_json::json!({});
        self.post_json(&url, &empty).await?;
        Ok(())
    }

    /// Set mood lighting
    pub async fn set_mood_light(
        &self,
        brightness: Option<u8>,
        color: Option<Color>,
        kelvin: Option<u16>,
    ) -> Result<()> {
        let mut payload = serde_json::Map::new();

        if let Some(b) = brightness {
            payload.insert(
                "brightness".to_string(),
                serde_json::Value::Number(b.into()),
            );
        }

        if let Some(c) = color {
            payload.insert("color".to_string(), serde_json::to_value(c)?);
        }

        if let Some(k) = kelvin {
            payload.insert("kelvin".to_string(), serde_json::Value::Number(k.into()));
        }

        self.post_json("/api/moodlight", &payload).await?;
        Ok(())
    }

    /// Set indicator
    pub async fn set_indicator(&self, indicator: u8, color: Option<Color>) -> Result<()> {
        let endpoint = format!("/api/indicator{}", indicator);

        if let Some(color) = color {
            let payload = serde_json::json!({ "color": color });
            self.post_json(&endpoint, &payload).await?;
        } else {
            // Turn off indicator by sending empty object
            let payload = serde_json::json!({});
            self.post_json(&endpoint, &payload).await?;
        }

        Ok(())
    }

    /// Play sound
    pub async fn play_sound(&self, sound: &str) -> Result<()> {
        let payload = serde_json::json!({ "sound": sound });
        self.post_json("/api/sound", &payload).await?;
        Ok(())
    }

    /// Play RTTTL
    pub async fn play_rtttl(&self, rtttl: &str) -> Result<()> {
        let payload = serde_json::json!({ "rtttl": rtttl });
        self.post_json("/api/rtttl", &payload).await?;
        Ok(())
    }

    /// Play R2D2 sound
    pub async fn play_r2d2(&self) -> Result<()> {
        self.post("/api/r2d2").await?;
        Ok(())
    }

    /// Get current settings
    pub async fn get_settings(&self) -> Result<Settings> {
        let response = self.get("/api/settings").await?;
        Self::parse_json(response).await
    }

    /// Update settings
    pub async fn update_settings(&self, settings: &Settings) -> Result<()> {
        self.post_json("/api/settings", settings).await?;
        Ok(())
    }

    /// Reboot device
    pub async fn reboot(&self) -> Result<()> {
        self.post("/api/reboot").await?;
        Ok(())
    }

    /// Factory reset (erase all settings)
    pub async fn factory_reset(&self) -> Result<()> {
        self.post("/api/erase").await?;
        Ok(())
    }

    /// Reset settings to defaults
    pub async fn reset_settings(&self) -> Result<()> {
        self.post("/api/resetSettings").await?;
        Ok(())
    }

    /// Save current configuration
    pub async fn save_config(&self) -> Result<()> {
        self.post("/save").await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::*;

    #[tokio::test]
    async fn test_client_creation() {
        let client = super::super::Client::new("192.168.1.100").unwrap();
        assert_eq!(client.base_url().as_str(), "http://192.168.1.100/");
    }

    #[test]
    fn test_url_building() {
        let client = super::super::Client::new("192.168.1.100").unwrap();
        let url = client.build_url("/api/stats").unwrap();
        assert_eq!(url.as_str(), "http://192.168.1.100/api/stats");
    }
}
