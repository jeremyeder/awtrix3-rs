use crate::cli::NotifyArgs;
use crate::utils::parse_color;
use anyhow::Result;
use awtrix3::Notification;
use std::fs;

pub async fn execute(client: awtrix3::Client, args: NotifyArgs) -> Result<()> {
    if args.dismiss {
        // Dismiss current notification
        client.dismiss_notification().await?;
        println!("Notification dismissed");
    } else if let Some(file_path) = args.file {
        // Load notification from JSON file
        let content = fs::read_to_string(file_path)?;
        let notification: Notification = serde_json::from_str(&content)?;
        
        client.notify(notification).await?;
        println!("Notification sent from file");
    } else {
        // Build notification from CLI arguments
        let notification = build_notification_from_args(args)?;
        let text_preview = notification.text.clone();
        
        client.notify(notification).await?;
        println!("Notification sent: {}", text_preview.as_deref().unwrap_or("(no text)"));
    }
    
    Ok(())
}

fn build_notification_from_args(args: NotifyArgs) -> Result<Notification> {
    let mut builder = Notification::builder();
    
    // Set text
    builder = builder.text(args.text);
    
    // Set icon if provided
    if let Some(icon) = args.icon {
        builder = builder.icon(icon);
    }
    
    // Set color if provided
    if let Some(color_str) = args.color {
        let color = parse_color(&color_str)?;
        builder = builder.color(color);
    }
    
    // Set duration if provided
    if let Some(duration) = args.duration {
        builder = builder.duration(duration);
    }
    
    // Set sound if provided
    if let Some(sound) = args.sound {
        builder = builder.sound(sound);
    }
    
    // Set progress if provided (validate 0-100)
    if let Some(progress) = args.progress {
        if progress > 100 {
            return Err(anyhow::anyhow!("Progress must be between 0 and 100"));
        }
        builder = builder.progress(progress);
    }
    
    // Set flags
    if args.hold {
        builder = builder.hold(true);
    }
    
    if args.wakeup {
        builder = builder.wakeup(true);
    }
    
    Ok(builder.build())
}