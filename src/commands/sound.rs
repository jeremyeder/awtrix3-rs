use crate::cli::*;
use anyhow::Result;

pub async fn execute(client: awtrix3::Client, command: SoundCommands) -> Result<()> {
    match command {
        SoundCommands::Play { sound, loop_sound } => {
            client.play_sound(&sound).await?;
            
            if loop_sound {
                println!("Playing sound '{}' (looped)", sound);
                println!("Note: Loop functionality depends on server implementation");
            } else {
                println!("Playing sound '{}'", sound);
            }
        }
        SoundCommands::Rtttl { rtttl } => {
            // Validate RTTTL format (basic check)
            if !rtttl.contains(':') {
                return Err(anyhow::anyhow!("Invalid RTTTL format. Expected format: 'name:d=4,o=5,b=140:notes'"));
            }
            
            client.play_rtttl(&rtttl).await?;
            println!("Playing RTTTL: {}", rtttl.split(':').next().unwrap_or("Unknown"));
        }
        SoundCommands::R2d2 => {
            client.play_r2d2().await?;
            println!("Playing R2D2 sound");
        }
        SoundCommands::List => {
            println!("Available sounds:");
            println!("  Built-in sounds:");
            println!("    - notification");
            println!("    - success");
            println!("    - error");
            println!("    - warning");
            println!("    - beep");
            println!("  Special:");
            println!("    - r2d2 (use 'sound r2d2' command)");
            println!("    - rtttl (use 'sound rtttl' command)");
            println!("");
            println!("Note: Actual available sounds depend on device configuration.");
            println!("Custom sounds can be uploaded to the device via web interface.");
        }
    }
    
    Ok(())
}
