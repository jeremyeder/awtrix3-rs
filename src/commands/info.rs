use crate::cli::*;
use anyhow::Result;

pub async fn execute(_client: awtrix3::Client, _command: InfoCommands) -> Result<()> {
    println!("Info command not yet implemented");
    Ok(())
}
