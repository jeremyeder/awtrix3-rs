use crate::cli::*;
use anyhow::Result;

pub async fn execute(_client: awtrix3::Client, _command: SoundCommands) -> Result<()> {
    println!("Sound command not yet implemented");
    Ok(())
}
