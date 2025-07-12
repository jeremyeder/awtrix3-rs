use crate::cli::*;
use anyhow::Result;

pub async fn execute(client: awtrix3::Client, command: InfoCommands) -> Result<()> {
    match command {
        InfoCommands::Version => {
            let version = client.get_version().await?;
            println!("AWTRIX3 Firmware Version: {}", version);
        }
        InfoCommands::Effects => {
            let effects = client.get_effects().await?;
            println!("Available Effects:");
            for (i, effect) in effects.iter().enumerate() {
                println!("  {}. {}", i + 1, effect);
            }
        }
        InfoCommands::Transitions => {
            let transitions = client.get_transitions().await?;
            println!("Available Transitions:");
            for (i, transition) in transitions.iter().enumerate() {
                println!("  {}. {}", i + 1, transition);
            }
        }
        InfoCommands::Screen { format } => match format {
            ScreenFormat::Text => {
                println!("Screen information (text format not yet implemented)");
            }
            ScreenFormat::Json => {
                println!("Screen information (JSON format not yet implemented)");
            }
            ScreenFormat::Raw => {
                println!("Screen information (raw format not yet implemented)");
            }
        },
    }
    Ok(())
}
