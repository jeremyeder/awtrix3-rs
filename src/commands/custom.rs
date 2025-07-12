use crate::cli::CustomCommands;
// use crate::utils::parse_color;
use anyhow::Result;
use awtrix3::CustomApp;
use std::fs;
use std::time::Duration;

pub async fn execute(client: awtrix3::Client, command: CustomCommands) -> Result<()> {
    match command {
        CustomCommands::Create {
            name,
            text,
            icon,
            duration,
            file,
        } => {
            let app = if let Some(file_path) = file {
                // Load from JSON file
                let content = fs::read_to_string(file_path)?;
                serde_json::from_str::<CustomApp>(&content)?
            } else {
                // Build from CLI arguments
                let mut app = CustomApp::new();
                app.text = text;
                app.icon = icon;
                app.duration = duration;
                app
            };

            // Create the app
            client.create_custom_app(&name, &app).await?;
            println!("Custom app '{}' created", name);
        }
        CustomCommands::Delete { name } => {
            client.delete_custom_app(&name).await?;
            println!("Custom app '{}' deleted", name);
        }
        CustomCommands::List => {
            println!("Custom app listing not yet implemented");
            println!("Note: AWTRIX3 API does not provide a direct endpoint to list custom apps.");
            println!(
                "Custom apps are managed through the web interface or by knowing their names."
            );
        }
        CustomCommands::Watch {
            name,
            file,
            interval,
        } => {
            // Validate file exists
            if !std::path::Path::new(&file).exists() {
                return Err(anyhow::anyhow!("Configuration file not found: {}", file));
            }

            println!(
                "Watching file '{}' for app '{}' (interval: {}s)",
                file, name, interval
            );
            println!("Press Ctrl+C to stop watching...");

            // TODO: Implement file watching
            // This would require:
            // 1. File system watching (using notify crate)
            // 2. Periodic updates based on interval
            // 3. JSON parsing and validation
            // 4. Signal handling for clean shutdown

            // For now, just simulate the behavior
            loop {
                match fs::read_to_string(&file) {
                    Ok(content) => match serde_json::from_str::<CustomApp>(&content) {
                        Ok(app) => match client.create_custom_app(&name, &app).await {
                            Ok(_) => println!("Updated app '{}' from file", name),
                            Err(e) => eprintln!("Failed to update app: {}", e),
                        },
                        Err(e) => eprintln!("Failed to parse JSON: {}", e),
                    },
                    Err(e) => eprintln!("Failed to read file: {}", e),
                }

                // Wait for the specified interval
                tokio::time::sleep(Duration::from_secs(interval)).await;

                // In a real implementation, we'd use file watching instead of polling
                // and handle Ctrl+C gracefully
            }
        }
    }

    Ok(())
}
