use anyhow::Result;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Default device to use if none specified
    pub default_device: Option<String>,

    /// Device configurations
    #[serde(default)]
    pub devices: HashMap<String, DeviceConfig>,

    /// CLI preferences
    #[serde(default)]
    pub preferences: Preferences,
}

/// Device-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceConfig {
    /// Device hostname or IP address
    pub host: String,

    /// Human-readable device name
    pub name: String,

    /// Timeout for requests in seconds
    #[serde(default = "default_timeout")]
    pub timeout: u64,

    /// Number of retry attempts
    #[serde(default = "default_retries")]
    pub retries: u32,
}

/// CLI preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preferences {
    /// Default output format
    #[serde(default = "default_format")]
    pub default_format: String,

    /// Enable colored output by default
    #[serde(default = "default_color")]
    pub colored_output: bool,

    /// Log level
    #[serde(default = "default_log_level")]
    pub log_level: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default_device: None,
            devices: HashMap::new(),
            preferences: Preferences::default(),
        }
    }
}

impl Default for Preferences {
    fn default() -> Self {
        Self {
            default_format: default_format(),
            colored_output: default_color(),
            log_level: default_log_level(),
        }
    }
}

fn default_timeout() -> u64 {
    30
}
fn default_retries() -> u32 {
    3
}
fn default_format() -> String {
    "table".to_string()
}
fn default_color() -> bool {
    true
}
fn default_log_level() -> String {
    "info".to_string()
}

/// Load configuration from file or create default
pub fn load_config() -> Result<Config> {
    let config_path = get_config_path()?;

    if config_path.exists() {
        let content = std::fs::read_to_string(&config_path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    } else {
        // Create default config file
        let config = Config::default();
        save_config(&config)?;
        Ok(config)
    }
}

/// Save configuration to file
pub fn save_config(config: &Config) -> Result<()> {
    let config_path = get_config_path()?;

    // Create config directory if it doesn't exist
    if let Some(parent) = config_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let content = toml::to_string_pretty(config)?;
    std::fs::write(config_path, content)?;

    Ok(())
}

/// Get the configuration file path
fn get_config_path() -> Result<PathBuf> {
    if let Some(proj_dirs) = ProjectDirs::from("", "awtrix3", "awtrix3") {
        Ok(proj_dirs.config_dir().join("config.toml"))
    } else {
        // Fallback to current directory
        Ok(PathBuf::from("config.toml"))
    }
}

/// Add a device to the configuration
pub fn add_device(
    name: String,
    host: String,
    device_name: String,
    set_default: bool,
) -> Result<()> {
    let mut config = load_config()?;

    let device_config = DeviceConfig {
        host,
        name: device_name,
        timeout: default_timeout(),
        retries: default_retries(),
    };

    config.devices.insert(name.clone(), device_config);

    if set_default || config.default_device.is_none() {
        config.default_device = Some(name);
    }

    save_config(&config)?;
    Ok(())
}

/// Remove a device from the configuration
pub fn remove_device(name: &str) -> Result<()> {
    let mut config = load_config()?;

    config.devices.remove(name);

    // Clear default if it was the removed device
    if config.default_device.as_deref() == Some(name) {
        config.default_device = None;
    }

    save_config(&config)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert!(config.devices.is_empty());
        assert!(config.default_device.is_none());
        assert_eq!(config.preferences.default_format, "table");
    }

    #[test]
    fn test_config_serialization() {
        let mut config = Config::default();
        config.default_device = Some("test".to_string());

        let device = DeviceConfig {
            host: "192.168.1.100".to_string(),
            name: "Test Device".to_string(),
            timeout: 30,
            retries: 3,
        };
        config.devices.insert("test".to_string(), device);

        let toml = toml::to_string(&config).unwrap();
        let parsed: Config = toml::from_str(&toml).unwrap();

        assert_eq!(config.default_device, parsed.default_device);
        assert_eq!(config.devices.len(), parsed.devices.len());
    }
}
