use crate::cli::AppCommands;
use anyhow::Result;

pub async fn execute(_client: awtrix3::Client, _command: AppCommands) -> Result<()> {
    println!("App command not yet implemented");
    Ok(())
}