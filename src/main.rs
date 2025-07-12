use clap::Parser;
use colored::{Colorize, control};
use anyhow::Result;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod cli;
mod commands;
mod config;
mod utils;

use cli::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));
    
    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer())
        .init();
    
    // Parse CLI arguments
    let cli = Cli::parse();
    
    // Store json flag before moving cli
    let json_output = cli.json;
    
    // Set up colored output
    if !json_output {
        control::set_override(true);
    }
    
    // Load configuration
    let config = config::load_config()?;
    
    // Execute command
    match cli.execute(config).await {
        Ok(()) => Ok(()),
        Err(e) => {
            if !json_output {
                eprintln!("{} {}", "Error:".red().bold(), e);
            } else {
                eprintln!(r#"{{"error": "{}"}}"#, e);
            }
            std::process::exit(1);
        }
    }
}
