use crate::cli::*;
// use crate::utils::parse_color;
use anyhow::Result;

pub async fn execute(client: awtrix3::Client, command: DisplayCommands) -> Result<()> {
    match command {
        DisplayCommands::Mood {
            color,
            kelvin,
            brightness,
        } => {
            // Validate kelvin range if provided
            if let Some(k) = kelvin {
                if k < 2000 || k > 6500 {
                    return Err(anyhow::anyhow!(
                        "Color temperature must be between 2000K and 6500K"
                    ));
                }
            }

            // Brightness is u8, so no validation needed (0-255 is enforced by type)

            // Parse color if provided
            let parsed_color = if let Some(color_str) = color {
                Some(awtrix3::Color::from_hex(&color_str)?)
            } else {
                None
            };

            // Call API
            client
                .set_mood_light(brightness, parsed_color, kelvin)
                .await?;

            // Provide feedback
            match (parsed_color, kelvin, brightness) {
                (Some(c), None, Some(b)) => println!(
                    "Mood light set to color {} with brightness {}",
                    c.to_hex(),
                    b
                ),
                (None, Some(k), Some(b)) => {
                    println!("Mood light set to {}K with brightness {}", k, b)
                }
                (Some(c), None, None) => println!("Mood light set to color {}", c.to_hex()),
                (None, Some(k), None) => println!("Mood light set to {}K", k),
                (None, None, Some(b)) => println!("Mood light brightness set to {}", b),
                _ => println!("Mood light updated"),
            }
        }
        DisplayCommands::Screen { fps, fullscreen } => {
            // Validate FPS range
            if fps > 60 {
                return Err(anyhow::anyhow!("FPS must be 60 or less"));
            }

            println!("Screen viewing not yet implemented");
            println!(
                "Would show screen at {} FPS{}",
                fps,
                if fullscreen { " (fullscreen)" } else { "" }
            );

            // TODO: Implement live screen viewing
            // This would require:
            // 1. Continuous GET requests to /api/screen
            // 2. Terminal rendering of the matrix data
            // 3. FPS timing control
            // 4. Keyboard interrupt handling
        }
        DisplayCommands::Stream { interval } => {
            println!("Screen streaming not yet implemented");
            println!("Would stream screen updates every {}ms", interval);

            // TODO: Implement screen streaming
            // Similar to screen viewing but continuous
        }
    }

    Ok(())
}
