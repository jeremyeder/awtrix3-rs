use crate::cli::*;
use crate::utils::parse_color;
use anyhow::Result;
use awtrix3::Settings;
use serde_json::Value;
use std::fs;

pub async fn execute(client: awtrix3::Client, command: SettingsCommands) -> Result<()> {
    match command {
        SettingsCommands::Get { key } => {
            let settings = client.get_settings().await?;
            
            if let Some(key) = key {
                display_specific_setting(&settings, &key)?;
            } else {
                display_all_settings(&settings);
            }
        }
        SettingsCommands::Set { key, value } => {
            // Get current settings
            let mut settings = client.get_settings().await?;
            
            // Update the specific setting
            update_setting(&mut settings, &key, &value)?;
            
            // Send updated settings
            client.update_settings(&settings).await?;
            
            println!("Setting '{}' updated to '{}'", key, value);
        }
        SettingsCommands::Import { file } => {
            // Validate file exists
            if !std::path::Path::new(&file).exists() {
                return Err(anyhow::anyhow!("Settings file not found: {}", file));
            }
            
            // Read and parse settings file
            let content = fs::read_to_string(&file)?;
            let imported_settings: Settings = serde_json::from_str(&content)?;
            
            // Apply imported settings
            client.update_settings(&imported_settings).await?;
            
            println!("Settings imported from: {}", file);
        }
        SettingsCommands::Export { output } => {
            let settings = client.get_settings().await?;
            
            // Generate filename if not provided
            let filename = output.unwrap_or_else(|| {
                let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
                format!("awtrix3_settings_{}.json", timestamp)
            });
            
            // Export settings as pretty JSON
            let json_content = serde_json::to_string_pretty(&settings)?;
            
            if filename == "-" {
                // Output to stdout
                println!("{}", json_content);
            } else {
                // Write to file
                fs::write(&filename, json_content)?;
                println!("Settings exported to: {}", filename);
            }
        }
        SettingsCommands::List => {
            display_settings_documentation();
        }
    }
    
    Ok(())
}

fn display_all_settings(settings: &Settings) {
    println!("Current AWTRIX3 Settings:");
    println!("");
    
    // Display settings
    if let Some(brightness) = settings.brightness {
        println!("  brightness: {} (0-255)", brightness);
    }
    
    if let Some(auto_brightness) = settings.auto_brightness {
        println!("  auto_brightness: {}", auto_brightness);
    }
    
    if let Some(auto_transition) = settings.auto_transition {
        println!("  auto_transition: {}", auto_transition);
    }
    
    if let Some(app_time) = settings.app_time {
        println!("  app_time: {} seconds", app_time);
    }
    
    if let Some(transition) = &settings.transition {
        println!("  transition: {}", transition);
    }
    
    if let Some(transition_time) = settings.transition_time {
        println!("  transition_time: {} ms", transition_time);
    }
    
    if let Some(text_color) = &settings.text_color {
        println!("  text_color: {}", text_color.to_hex());
    }
    
    if let Some(temp_unit) = &settings.temp_unit {
        println!("  temp_unit: {}", temp_unit);
    }
    
    if let Some(scroll_speed) = settings.scroll_speed {
        println!("  scroll_speed: {}%", scroll_speed);
    }
    
    // Time app settings
    if let Some(time_app) = &settings.time_app {
        println!("");
        println!("  Time App Settings:");
        if let Some(format) = time_app.format {
            println!("    time_app.format: {}", format);
        }
        if let Some(show_weekday) = time_app.show_weekday {
            println!("    time_app.show_weekday: {}", show_weekday);
        }
        if let Some(cal_header_color) = &time_app.cal_header_color {
            println!("    time_app.cal_header_color: {}", cal_header_color.to_hex());
        }
        if let Some(cal_body_color) = &time_app.cal_body_color {
            println!("    time_app.cal_body_color: {}", cal_body_color.to_hex());
        }
        if let Some(cal_text_color) = &time_app.cal_text_color {
            println!("    time_app.cal_text_color: {}", cal_text_color.to_hex());
        }
    }
    
    // Date app settings
    if let Some(date_app) = &settings.date_app {
        println!("");
        println!("  Date App Settings:");
        if let Some(enabled) = date_app.enabled {
            println!("    date_app.enabled: {}", enabled);
        }
        if let Some(format) = &date_app.format {
            println!("    date_app.format: {}", format);
        }
    }
}

fn display_specific_setting(settings: &Settings, key: &str) -> Result<()> {
    match key {
        "brightness" => {
            if let Some(value) = settings.brightness {
                println!("{}", value);
            } else {
                println!("Setting '{}' not set", key);
            }
        }
        "auto_brightness" => {
            if let Some(value) = settings.auto_brightness {
                println!("{}", value);
            } else {
                println!("Setting '{}' not set", key);
            }
        }
        "auto_transition" => {
            if let Some(value) = settings.auto_transition {
                println!("{}", value);
            } else {
                println!("Setting '{}' not set", key);
            }
        }
        "app_time" => {
            if let Some(value) = settings.app_time {
                println!("{}", value);
            } else {
                println!("Setting '{}' not set", key);
            }
        }
        "transition" => {
            if let Some(value) = &settings.transition {
                println!("{}", value);
            } else {
                println!("Setting '{}' not set", key);
            }
        }
        "transition_time" => {
            if let Some(value) = settings.transition_time {
                println!("{}", value);
            } else {
                println!("Setting '{}' not set", key);
            }
        }
        "text_color" => {
            if let Some(value) = &settings.text_color {
                println!("{}", value.to_hex());
            } else {
                println!("Setting '{}' not set", key);
            }
        }
        "temp_unit" => {
            if let Some(value) = &settings.temp_unit {
                println!("{}", value);
            } else {
                println!("Setting '{}' not set", key);
            }
        }
        "scroll_speed" => {
            if let Some(value) = settings.scroll_speed {
                println!("{}", value);
            } else {
                println!("Setting '{}' not set", key);
            }
        }
        "time_app.format" => {
            if let Some(time_app) = &settings.time_app {
                if let Some(value) = time_app.format {
                    println!("{}", value);
                } else {
                    println!("Setting '{}' not set", key);
                }
            } else {
                println!("Setting '{}' not set", key);
            }
        }
        "time_app.show_weekday" => {
            if let Some(time_app) = &settings.time_app {
                if let Some(value) = time_app.show_weekday {
                    println!("{}", value);
                } else {
                    println!("Setting '{}' not set", key);
                }
            } else {
                println!("Setting '{}' not set", key);
            }
        }
        _ => {
            return Err(anyhow::anyhow!("Unknown setting key: {}", key));
        }
    }
    
    Ok(())
}

fn update_setting(settings: &mut Settings, key: &str, value: &str) -> Result<()> {
    match key {
        "brightness" => {
            let val: u8 = value.parse().map_err(|_| anyhow::anyhow!("Invalid brightness value. Must be 0-255"))?;
            settings.brightness = Some(val);
        }
        "auto_brightness" => {
            let val: bool = value.parse().map_err(|_| anyhow::anyhow!("Invalid boolean value. Use 'true' or 'false'"))?;
            settings.auto_brightness = Some(val);
        }
        "auto_transition" => {
            let val: bool = value.parse().map_err(|_| anyhow::anyhow!("Invalid boolean value. Use 'true' or 'false'"))?;
            settings.auto_transition = Some(val);
        }
        "app_time" => {
            let val: u32 = value.parse().map_err(|_| anyhow::anyhow!("Invalid app_time value. Must be a positive number"))?;
            settings.app_time = Some(val);
        }
        "transition" => {
            settings.transition = Some(value.to_string());
        }
        "transition_time" => {
            let val: u32 = value.parse().map_err(|_| anyhow::anyhow!("Invalid transition_time value. Must be a positive number"))?;
            settings.transition_time = Some(val);
        }
        "text_color" => {
            let color = parse_color(value)?;
            settings.text_color = Some(color);
        }
        "temp_unit" => {
            settings.temp_unit = Some(value.to_string());
        }
        "scroll_speed" => {
            let val: u32 = value.parse().map_err(|_| anyhow::anyhow!("Invalid scroll_speed value. Must be a positive number"))?;
            settings.scroll_speed = Some(val);
        }
        "time_app.format" => {
            let val: u8 = value.parse().map_err(|_| anyhow::anyhow!("Invalid time format. Must be 0-5"))?;
            if val > 5 {
                return Err(anyhow::anyhow!("Time format must be 0-5"));
            }
            if settings.time_app.is_none() {
                settings.time_app = Some(awtrix3::models::settings::TimeAppSettings {
                    format: None,
                    show_weekday: None,
                    cal_header_color: None,
                    cal_body_color: None,
                    cal_text_color: None,
                });
            }
            if let Some(time_app) = &mut settings.time_app {
                time_app.format = Some(val);
            }
        }
        "time_app.show_weekday" => {
            let val: bool = value.parse().map_err(|_| anyhow::anyhow!("Invalid boolean value. Use 'true' or 'false'"))?;
            if settings.time_app.is_none() {
                settings.time_app = Some(awtrix3::models::settings::TimeAppSettings {
                    format: None,
                    show_weekday: None,
                    cal_header_color: None,
                    cal_body_color: None,
                    cal_text_color: None,
                });
            }
            if let Some(time_app) = &mut settings.time_app {
                time_app.show_weekday = Some(val);
            }
        }
        _ => {
            return Err(anyhow::anyhow!("Unknown setting key: {}. Use 'settings list' to see available settings", key));
        }
    }
    
    Ok(())
}

fn display_settings_documentation() {
    println!("Available AWTRIX3 Settings:");
    println!("");
    
    println!("📱 Display Settings:");
    println!("  brightness                  Matrix brightness (0-255)");
    println!("  auto_brightness             Automatic brightness control (true/false)");
    println!("  text_color                  Global text color (hex: #FF0000, rgb: 255,0,0, or name: red)");
    println!("");
    
    println!("🔄 App Management:");
    println!("  auto_transition             Automatic app switching (true/false)");
    println!("  app_time                    App display duration in seconds");
    println!("  transition                  Transition effect name (slide, fade, etc.)");
    println!("  transition_time             Transition duration in milliseconds");
    println!("");
    
    println!("🕐 Time App Settings:");
    println!("  time_app.format             Time format (0-5)");
    println!("  time_app.show_weekday       Show weekday (true/false)");
    println!("  time_app.cal_header_color   Calendar header color");
    println!("  time_app.cal_body_color     Calendar body color");
    println!("  time_app.cal_text_color     Calendar text color");
    println!("");
    
    println!("📅 Date App Settings:");
    println!("  date_app.enabled            Enable date app (true/false)");
    println!("  date_app.format             Date format string");
    println!("");
    
    println!("⚙️  Other Settings:");
    println!("  scroll_speed                Scroll speed percentage (0-200)");
    println!("  temp_unit                   Temperature unit (C/F)");
    println!("");
    
    println!("Example usage:");
    println!("  awtrix settings set brightness 150");
    println!("  awtrix settings set text_color \"#FF0000\"");
    println!("  awtrix settings set time_app.format 2");
    println!("  awtrix settings set auto_transition true");
}
