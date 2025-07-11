use crate::cli::*;
use anyhow::Result;

pub async fn execute(_client: awtrix3::Client, _command: DisplayCommands) -> Result<()> {
    println!("Display command not yet implemented");
    Ok(())
}
