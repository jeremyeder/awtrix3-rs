use crate::cli::*;
use anyhow::Result;

pub async fn execute(_command: DeviceCommands, _config: crate::config::Config) -> Result<()> {
    println!("Device command not yet implemented");
    Ok(())
}
