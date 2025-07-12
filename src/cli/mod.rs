use crate::config::Config;
use anyhow::Result;
use clap::{Parser, Subcommand};

mod args;
pub use args::*;

/// AWTRIX3 CLI - Control your LED matrix display
#[derive(Parser, Debug)]
#[command(name = "awtrix")]
#[command(about = "A modern CLI for controlling AWTRIX3 LED matrix displays")]
#[command(version)]
#[command(author)]
pub struct Cli {
    /// Device name or IP address (can be set in config)
    #[arg(short, long, global = true)]
    pub device: Option<String>,

    /// Output JSON for scripting
    #[arg(short, long, global = true)]
    pub json: bool,

    /// Enable verbose logging
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Disable colored output
    #[arg(long, global = true)]
    pub no_color: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Power management
    Power(PowerArgs),

    /// Sleep mode
    Sleep(SleepArgs),

    /// System commands (reboot, update, stats, etc.)
    System {
        #[command(subcommand)]
        command: SystemCommands,
    },

    /// Display information (version, effects, transitions, etc.)
    Info {
        #[command(subcommand)]
        command: InfoCommands,
    },

    /// App management
    App {
        #[command(subcommand)]
        command: AppCommands,
    },

    /// Send notifications
    Notify(NotifyArgs),

    /// Custom app management
    Custom {
        #[command(subcommand)]
        command: CustomCommands,
    },

    /// Display control (mood light, screen, indicators)
    Display {
        #[command(subcommand)]
        command: DisplayCommands,
    },

    /// Sound control
    Sound {
        #[command(subcommand)]
        command: SoundCommands,
    },

    /// Indicator control
    Indicator(IndicatorArgs),

    /// Settings management
    Settings {
        #[command(subcommand)]
        command: SettingsCommands,
    },

    /// Device discovery and management
    Device {
        #[command(subcommand)]
        command: DeviceCommands,
    },

    /// Generate shell completions
    Completions {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: clap_complete::Shell,
    },
}

impl Cli {
    pub async fn execute(self, config: Config) -> Result<()> {
        // Get device host
        let device_host = self.get_device_host(&config)?;

        // Create client
        let client = awtrix3::Client::new(&device_host)?;

        // Execute command
        match self.command {
            Commands::Power(args) => crate::commands::power::execute(client, args).await,
            Commands::Sleep(args) => crate::commands::power::sleep(client, args).await,
            Commands::System { command } => crate::commands::system::execute(client, command).await,
            Commands::Info { command } => crate::commands::info::execute(client, command).await,
            Commands::App { command } => crate::commands::apps::execute(client, command).await,
            Commands::Notify(args) => crate::commands::notify::execute(client, args).await,
            Commands::Custom { command } => crate::commands::custom::execute(client, command).await,
            Commands::Display { command } => {
                crate::commands::display::execute(client, command).await
            }
            Commands::Sound { command } => crate::commands::sound::execute(client, command).await,
            Commands::Indicator(args) => crate::commands::indicators::execute(client, args).await,
            Commands::Settings { command } => {
                crate::commands::settings::execute(client, command).await
            }
            Commands::Device { command } => crate::commands::device::execute(command, config).await,
            Commands::Completions { shell } => {
                Self::generate_completions(shell);
                Ok(())
            }
        }
    }

    fn get_device_host(&self, config: &Config) -> Result<String> {
        // Priority: CLI arg > env var > config file
        if let Some(device) = &self.device {
            // Check if it's a device name from config
            if let Some(device_config) = config.devices.get(device) {
                Ok(device_config.host.clone())
            } else {
                // Assume it's a direct host/IP
                Ok(device.clone())
            }
        } else if let Ok(device) = std::env::var("AWTRIX_DEVICE") {
            Ok(device)
        } else if let Some(default) = &config.default_device {
            if let Some(device_config) = config.devices.get(default) {
                Ok(device_config.host.clone())
            } else {
                Err(anyhow::anyhow!(
                    "Default device '{}' not found in config",
                    default
                ))
            }
        } else {
            Err(anyhow::anyhow!(
                "No device specified. Use --device, set AWTRIX_DEVICE env var, or configure a default device"
            ))
        }
    }

    fn generate_completions(shell: clap_complete::Shell) {
        use clap::CommandFactory;
        use clap_complete::generate;
        use std::io;

        let mut cmd = Self::command();
        let name = cmd.get_name().to_string();
        generate(shell, &mut cmd, name, &mut io::stdout());
    }
}
