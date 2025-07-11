use crate::cli::CustomCommands;
use anyhow::Result;

pub async fn execute(_client: awtrix3::Client, _command: CustomCommands) -> Result<()> {
    println!("Custom command not yet implemented");
    Ok(())
}
