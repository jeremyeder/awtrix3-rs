use clap::{Args, Subcommand, ValueEnum};

/// Power control arguments
#[derive(Args, Debug)]
pub struct PowerArgs {
    /// Power state (on/off)
    #[arg(value_enum)]
    pub state: PowerState,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum PowerState {
    On,
    Off,
}

/// Sleep mode arguments
#[derive(Args, Debug)]
pub struct SleepArgs {
    /// Sleep duration in seconds
    #[arg(short, long)]
    pub duration: u32,
}

/// System subcommands
#[derive(Subcommand, Debug)]
pub enum SystemCommands {
    /// Reboot the device
    Reboot,

    /// Update firmware
    Update {
        /// Firmware file path
        #[arg(short, long)]
        file: String,
    },

    /// Factory reset (WARNING: This will erase all settings)
    FactoryReset {
        /// Confirm the reset
        #[arg(long)]
        confirm: bool,
    },

    /// Reset settings to defaults
    ResetSettings {
        /// Confirm the reset
        #[arg(long)]
        confirm: bool,
    },

    /// Get device statistics
    Stats,

    /// Save current configuration
    Save,

    /// Create backup
    Backup {
        /// Output file
        #[arg(short, long)]
        output: Option<String>,
    },
}

/// Info subcommands
#[derive(Subcommand, Debug)]
pub enum InfoCommands {
    /// Get device version
    Version,

    /// List available effects
    Effects,

    /// List available transitions
    Transitions,

    /// Get current screen data
    Screen {
        /// Output format
        #[arg(short, long, value_enum, default_value = "text")]
        format: ScreenFormat,
    },
}

#[derive(Debug, Clone, ValueEnum)]
pub enum ScreenFormat {
    Text,
    Json,
    Raw,
}

/// App management subcommands
#[derive(Subcommand, Debug)]
pub enum AppCommands {
    /// List all apps
    List,

    /// Switch to next app
    Next,

    /// Switch to previous app
    Previous,

    /// Switch to specific app
    Switch {
        /// App name
        name: String,
    },

    /// Reorder apps
    Reorder {
        /// Comma-separated list of app names in desired order
        apps: String,
    },

    /// Update app configuration
    Update {
        /// Configuration file (JSON)
        #[arg(short, long)]
        file: String,
    },
}

/// Notification arguments
#[derive(Args, Debug)]
pub struct NotifyArgs {
    /// Notification text
    pub text: String,

    /// Icon ID
    #[arg(short, long)]
    pub icon: Option<u32>,

    /// Text color (hex or r,g,b)
    #[arg(short, long)]
    pub color: Option<String>,

    /// Duration in seconds
    #[arg(short = 't', long)]
    pub duration: Option<u32>,

    /// Sound to play
    #[arg(short, long)]
    pub sound: Option<String>,

    /// Progress bar (0-100)
    #[arg(short, long)]
    pub progress: Option<u8>,

    /// Hold notification
    #[arg(long)]
    pub hold: bool,

    /// Wake up display
    #[arg(short, long)]
    pub wakeup: bool,

    /// Stack with other notifications
    #[arg(long)]
    pub stack: bool,

    /// Disable scrolling
    #[arg(long)]
    pub no_scroll: bool,

    /// Read notification from JSON file
    #[arg(short, long, conflicts_with = "text")]
    pub file: Option<String>,

    /// Dismiss current notification
    #[arg(long)]
    pub dismiss: bool,
}

/// Custom app subcommands
#[derive(Subcommand, Debug)]
pub enum CustomCommands {
    /// Create or update a custom app
    Create {
        /// App name
        name: String,

        /// Display text
        #[arg(short, long)]
        text: Option<String>,

        /// Icon ID
        #[arg(short, long)]
        icon: Option<u32>,

        /// Duration
        #[arg(short, long)]
        duration: Option<u32>,

        /// Configuration file (JSON)
        #[arg(short, long)]
        file: Option<String>,
    },

    /// Delete a custom app
    Delete {
        /// App name
        name: String,
    },

    /// List custom apps
    List,

    /// Watch and update app from file
    Watch {
        /// App name
        name: String,

        /// Configuration file to watch
        #[arg(short, long)]
        file: String,

        /// Update interval in seconds
        #[arg(short, long, default_value = "60")]
        interval: u64,
    },
}

/// Display control subcommands
#[derive(Subcommand, Debug)]
pub enum DisplayCommands {
    /// Set mood lighting
    Mood {
        /// RGB color (r,g,b or hex)
        #[arg(short, long, group = "color_group")]
        color: Option<String>,

        /// Color temperature in Kelvin
        #[arg(short, long, group = "color_group")]
        kelvin: Option<u16>,

        /// Brightness (0-255)
        #[arg(short, long)]
        brightness: Option<u8>,
    },

    /// Live screen view
    Screen {
        /// Frames per second
        #[arg(long, default_value = "10")]
        fps: u8,

        /// Fullscreen mode
        #[arg(short, long)]
        fullscreen: bool,
    },

    /// Stream display to terminal
    Stream {
        /// Update interval in milliseconds
        #[arg(short, long, default_value = "100")]
        interval: u64,
    },
}

/// Sound control subcommands
#[derive(Subcommand, Debug)]
pub enum SoundCommands {
    /// Play a sound
    Play {
        /// Sound name or file
        sound: String,

        /// Loop the sound
        #[arg(short, long)]
        loop_sound: bool,
    },

    /// Play RTTTL
    Rtttl {
        /// RTTTL string
        rtttl: String,
    },

    /// Play R2D2 sound
    R2d2,

    /// List available sounds
    List,
}

/// Indicator arguments
#[derive(Args, Debug)]
pub struct IndicatorArgs {
    /// Indicator number (1-3) or 'all'
    pub indicator: String,

    /// Color (hex or r,g,b)
    #[arg(short, long)]
    pub color: Option<String>,

    /// Turn off indicator
    #[arg(long, conflicts_with = "color")]
    pub off: bool,
}

/// Settings subcommands
#[derive(Subcommand, Debug)]
pub enum SettingsCommands {
    /// Get current settings
    Get {
        /// Specific setting key
        key: Option<String>,
    },

    /// Set a setting
    Set {
        /// Setting key
        key: String,

        /// Setting value
        value: String,
    },

    /// Import settings from file
    Import {
        /// Settings file (JSON)
        file: String,
    },

    /// Export settings to file
    Export {
        /// Output file
        #[arg(short, long)]
        output: Option<String>,
    },

    /// List all available settings
    List,
}

/// Device management subcommands
#[derive(Subcommand, Debug)]
pub enum DeviceCommands {
    /// Discover devices on the network
    #[cfg(feature = "discovery")]
    Discover {
        /// Discovery timeout in seconds
        #[arg(short, long, default_value = "5")]
        timeout: u64,
    },

    /// Add a device to config
    Add {
        /// Device name
        name: String,

        /// Device host/IP
        host: String,

        /// Set as default device
        #[arg(short, long)]
        default: bool,
    },

    /// Remove a device from config
    Remove {
        /// Device name
        name: String,
    },

    /// List configured devices
    List,

    /// Test connection to a device
    Test {
        /// Device name or host
        device: Option<String>,
    },
}
