use crate::cli::{PowerArgs, PowerState, SleepArgs};
use anyhow::Result;

pub async fn execute(client: awtrix3::Client, args: PowerArgs) -> Result<()> {
    let power_on = match args.state {
        PowerState::On => true,
        PowerState::Off => false,
    };
    
    // Call the API
    client.set_power(power_on).await?;
    
    // Provide user feedback
    println!("Power state set to: {}", if power_on { "ON" } else { "OFF" });
    
    Ok(())
}

pub async fn sleep(client: awtrix3::Client, args: SleepArgs) -> Result<()> {
    // Validate duration is positive
    if args.duration == 0 {
        return Err(anyhow::anyhow!("Sleep duration must be greater than 0 seconds"));
    }
    
    // Call the API
    client.set_sleep(args.duration).await?;
    
    // Provide user feedback
    println!("Sleep mode set for {} seconds", args.duration);
    
    Ok(())
}