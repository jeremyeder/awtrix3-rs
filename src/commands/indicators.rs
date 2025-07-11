use crate::cli::*;
use anyhow::Result;

pub async fn execute(_client: awtrix3::Client, _args: IndicatorArgs) -> Result<()> {
    println!("Indicators command not yet implemented");
    Ok(())
}
