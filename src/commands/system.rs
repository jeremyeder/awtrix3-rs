use crate::cli::*;
use anyhow::Result;

pub async fn execute(_client: awtrix3::Client, _command: SystemCommands) -> Result<()> {
    println!("System command not yet implemented");
    Ok(())
}
