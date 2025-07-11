use crate::cli::*;
use crate::config::{self, Config, DeviceConfig};
use anyhow::Result;
use std::time::Duration;

pub async fn execute(command: DeviceCommands, mut config: Config) -> Result<()> {
    match command {
        #[cfg(feature = "discovery")]
        DeviceCommands::Discover { timeout } => {
            println!("Discovering AWTRIX3 devices on network...");
            println!("Timeout: {} seconds", timeout);
            
            let devices = crate::utils::discovery::discover_devices(Duration::from_secs(timeout)).await?;
            
            if devices.is_empty() {
                println!("No AWTRIX3 devices found on the network.");
                println!("Make sure devices are powered on and connected to the same network.");
            } else {
                println!("Found {} device(s):", devices.len());
                for device in devices {
                    println!("  📱 {} ({}:{})", device.name, device.host, device.port);
                    
                    // Test connectivity
                    if let Ok(client) = awtrix3::Client::new(&device.host) {
                        match client.get_version().await {
                            Ok(version) => println!("     ✅ Online - Version: {}", version.trim()),
                            Err(_) => println!("     ❌ Cannot connect to API"),
                        }
                    } else {
                        println!("     ❌ Invalid host");
                    }
                }
                println!("");
                println!("To add a device, use: awtrix device add <name> <host>");
            }
        }
        
        #[cfg(not(feature = "discovery"))]
        DeviceCommands::Discover { .. } => {
            println!("❌ Device discovery is not available.");
            println!("This binary was compiled without discovery support.");
            println!("To enable discovery, rebuild with: cargo build --features discovery");
        }
        
        DeviceCommands::Add { name, host, default } => {
            // Test connectivity first
            println!("Testing connection to {}...", host);
            
            match awtrix3::Client::new(&host) {
                Ok(client) => {
                    match client.get_version().await {
                        Ok(version) => {
                            println!("✅ Connected successfully - Version: {}", version.trim());
                            
                            // Add device to config
                            let device_config = DeviceConfig {
                                host: host.clone(),
                                name: format!("AWTRIX3 {}", name),
                                timeout: 30,
                                retries: 3,
                            };
                            
                            config.devices.insert(name.clone(), device_config);
                            
                            if default {
                                config.default_device = Some(name.clone());
                                println!("Set '{}' as default device", name);
                            }
                            
                            // Save config
                            config::save_config(&config)?;
                            println!("Device '{}' added successfully", name);
                        }
                        Err(e) => {
                            return Err(anyhow::anyhow!(
                                "Cannot connect to AWTRIX3 API at {}: {}", host, e
                            ));
                        }
                    }
                }
                Err(e) => {
                    return Err(anyhow::anyhow!(
                        "Invalid host '{}': {}", host, e
                    ));
                }
            }
        }
        
        DeviceCommands::Remove { name } => {
            if config.devices.contains_key(&name) {
                config.devices.remove(&name);
                
                // Handle default device removal
                if config.default_device.as_ref() == Some(&name) {
                    if config.devices.is_empty() {
                        config.default_device = None;
                        println!("No devices remaining - cleared default device");
                    } else {
                        // Set first remaining device as default
                        let new_default = config.devices.keys().next().unwrap().clone();
                        config.default_device = Some(new_default.clone());
                        println!("Set '{}' as new default device", new_default);
                    }
                }
                
                // Save config
                config::save_config(&config)?;
                println!("Device '{}' removed successfully", name);
            } else {
                return Err(anyhow::anyhow!("Device '{}' not found in configuration", name));
            }
        }
        
        DeviceCommands::List => {
            if config.devices.is_empty() {
                println!("No devices configured.");
                println!("Use 'awtrix device discover' to find devices or 'awtrix device add' to add manually.");
                return Ok(());
            }
            
            println!("Configured AWTRIX3 Devices:");
            println!("");
            
            for (name, device) in &config.devices {
                let is_default = config.default_device.as_ref() == Some(name);
                let default_marker = if is_default { " (default)" } else { "" };
                
                println!("📱 {} - {}{}", name, device.host, default_marker);
                println!("   Name: {}", device.name);
                
                // Test connectivity
                print!("   Status: ");
                match awtrix3::Client::new(&device.host) {
                    Ok(client) => {
                        match client.get_version().await {
                            Ok(version) => println!("✅ Online - Version: {}", version.trim()),
                            Err(_) => println!("❌ Offline or unreachable"),
                        }
                    }
                    Err(_) => println!("❌ Invalid configuration"),
                }
                
                println!("");
            }
        }
        
        DeviceCommands::Test { device } => {
            let device_config = if let Some(device_name) = device {
                // Test specific device
                config.devices.get(&device_name)
                    .ok_or_else(|| anyhow::anyhow!("Device '{}' not found in configuration", device_name))?
            } else {
                // Test default device
                let default_name = config.default_device
                    .as_ref()
                    .ok_or_else(|| anyhow::anyhow!("No default device configured"))?;
                
                config.devices.get(default_name)
                    .ok_or_else(|| anyhow::anyhow!("Default device '{}' not found in configuration", default_name))?
            };
            
            println!("Testing device: {}", device_config.name);
            println!("Host: {}", device_config.host);
            println!("");
            
            let start = std::time::Instant::now();
            
            match awtrix3::Client::new(&device_config.host) {
                Ok(client) => {
                    println!("✅ Client created successfully");
                    
                    // Test API endpoints
                    match client.get_version().await {
                        Ok(version) => {
                            let response_time = start.elapsed();
                            println!("✅ Version API: {} ({}ms)", version.trim(), response_time.as_millis());
                        }
                        Err(e) => {
                            println!("❌ Version API failed: {}", e);
                            return Ok(());
                        }
                    }
                    
                    // Test stats
                    let stats_start = std::time::Instant::now();
                    match client.get_stats().await {
                        Ok(_) => {
                            let response_time = stats_start.elapsed();
                            println!("✅ Stats API: OK ({}ms)", response_time.as_millis());
                        }
                        Err(e) => {
                            println!("⚠️  Stats API: {}", e);
                        }
                    }
                    
                    println!("");
                    println!("✅ Device test completed successfully");
                    println!("Total response time: {}ms", start.elapsed().as_millis());
                }
                Err(e) => {
                    println!("❌ Failed to create client: {}", e);
                }
            }
        }
    }
    
    Ok(())
}
