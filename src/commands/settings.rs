use crate::cli::*;
use anyhow::Result;

pub async fn execute(_client: awtrix3::Client, _command: SettingsCommands) -> Result<()> {
    println!("Settings command not yet implemented");
    Ok(())
}
