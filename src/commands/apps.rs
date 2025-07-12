use crate::cli::AppCommands;
use anyhow::Result;

pub async fn execute(client: awtrix3::Client, command: AppCommands) -> Result<()> {
    match command {
        AppCommands::List => {
            let loop_info = client.get_apps().await?;
            println!("App Loop Status:");
            if let Some(current) = &loop_info.current {
                println!("  Current App: {}", current);
            } else {
                println!("  Current App: None");
            }

            if !loop_info.apps.is_empty() {
                println!("\nAvailable Apps:");
                for app in loop_info.apps {
                    let status = if app.enabled.unwrap_or(true) {
                        "enabled"
                    } else {
                        "disabled"
                    };
                    println!("  - {} ({})", app.name, status);
                }
            } else {
                println!("\nNo apps in loop");
            }
        }
        AppCommands::Next => {
            client.next_app().await?;
            println!("Switched to next app");
        }
        AppCommands::Previous => {
            client.previous_app().await?;
            println!("Switched to previous app");
        }
        AppCommands::Switch { name } => {
            client.switch_app(&name).await?;
            println!("Switched to app: {}", name);
        }
        AppCommands::Reorder { apps } => {
            // Split the comma-separated list
            let app_list: Vec<&str> = apps.split(',').map(|s| s.trim()).collect();

            // TODO: Implement reorder API call
            // For now, just show what would be reordered
            println!("Reordering apps to: {:?}", app_list);
            println!("(Reorder functionality not yet implemented in API client)");
        }
        AppCommands::Update { file: _file } => {
            // TODO: Implement app configuration update
            println!("App configuration update not yet implemented");
        }
    }

    Ok(())
}
