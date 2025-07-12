use crate::cli::*;
use anyhow::Result;
use dialoguer::Confirm;
use std::fs;
use std::path::Path;

pub async fn execute(client: awtrix3::Client, command: SystemCommands) -> Result<()> {
    match command {
        SystemCommands::Stats => {
            let stats = client.get_stats().await?;
            display_stats(&stats);
        }
        SystemCommands::Reboot => {
            if confirm_destructive_action("reboot the device")? {
                client.reboot().await?;
                println!("Device reboot initiated");
            } else {
                println!("Reboot cancelled");
            }
        }
        SystemCommands::FactoryReset { confirm } => {
            if confirm
                || confirm_destructive_action(
                    "perform FACTORY RESET (this will erase ALL settings and data)",
                )?
            {
                client.factory_reset().await?;
                println!("Factory reset initiated - device will restart with default settings");
            } else {
                println!("Factory reset cancelled");
            }
        }
        SystemCommands::ResetSettings { confirm } => {
            if confirm || confirm_destructive_action("reset settings to defaults")? {
                client.reset_settings().await?;
                println!("Settings reset to defaults");
            } else {
                println!("Settings reset cancelled");
            }
        }
        SystemCommands::Save => {
            client.save_config().await?;
            println!("Configuration saved");
        }
        SystemCommands::Update { file } => {
            // Validate file exists
            if !Path::new(&file).exists() {
                return Err(anyhow::anyhow!("Firmware file not found: {}", file));
            }

            // TODO: Implement firmware upload
            // This requires multipart file upload which isn't implemented in the client yet
            println!("Firmware update not yet implemented");
            println!("Would upload: {}", file);
        }
        SystemCommands::Backup { output } => {
            // Generate default filename if none provided
            let filename = output.unwrap_or_else(|| {
                let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
                format!("awtrix3_backup_{}.json", timestamp)
            });

            // TODO: Implement backup by getting all settings/config
            // For now, we'll get stats as a starting point
            let stats = client.get_stats().await?;
            let backup_data = serde_json::to_string_pretty(&stats)?;

            fs::write(&filename, backup_data)?;
            println!("Backup saved to: {}", filename);
            println!(
                "Note: Currently only saves device statistics. Full backup implementation pending."
            );
        }
    }

    Ok(())
}

fn display_stats(stats: &awtrix3::models::response::Stats) {
    println!("Device Statistics:");

    // Basic info
    println!(
        "  Uptime: {} seconds ({:.1} hours)",
        stats.uptime,
        stats.uptime as f64 / 3600.0
    );
    println!("  WiFi Signal: {} dBm", stats.wifi_signal);
    println!("  Free Memory: {} bytes", stats.heap);
    println!("  Matrix: {}", if stats.matrix { "ON" } else { "OFF" });

    // Current app
    if let Some(app) = &stats.current_app {
        println!("  Current App: {}", app);
    }

    // Sensors
    if let Some(temp) = stats.temperature {
        println!("  Temperature: {:.1}°C", temp);
    }

    if let Some(humidity) = stats.humidity {
        println!("  Humidity: {:.1}%", humidity);
    }

    if let Some(ldr) = stats.ldr {
        println!("  Light Sensor (LDR): {}", ldr);
    }

    if let Some(lux) = stats.lux {
        println!("  Light Level: {:.1} lux", lux);
    }

    if let Some(battery) = stats.battery {
        println!("  Battery: {}%", battery);
    }

    // Indicators
    if let Some(indicators) = &stats.indicators {
        println!("  Indicators:");
        println!(
            "    1: {}",
            if indicators.indicator1 { "ON" } else { "OFF" }
        );
        println!(
            "    2: {}",
            if indicators.indicator2 { "ON" } else { "OFF" }
        );
        println!(
            "    3: {}",
            if indicators.indicator3 { "ON" } else { "OFF" }
        );
    }
}

fn confirm_destructive_action(action: &str) -> Result<bool> {
    println!("⚠️  WARNING: This will {}", action);
    let confirmed = Confirm::new()
        .with_prompt("Are you sure you want to continue?")
        .default(false)
        .interact()?;

    Ok(confirmed)
}
