# awtrix3-rs

[![Crates.io](https://img.shields.io/crates/v/awtrix3.svg)](https://crates.io/crates/awtrix3)
[![Documentation](https://docs.rs/awtrix3/badge.svg)](https://docs.rs/awtrix3)
[![CI](https://github.com/jeremyeder/awtrix3-rs/workflows/CI/badge.svg)](https://github.com/jeremyeder/awtrix3-rs/actions)
[![Coverage](https://codecov.io/gh/jeremyeder/awtrix3-rs/branch/main/graph/badge.svg)](https://codecov.io/gh/jeremyeder/awtrix3-rs)
[![License](https://img.shields.io/crates/l/awtrix3.svg)](LICENSE)

A modern, async Rust CLI and library for controlling AWTRIX3 LED matrix displays.

## Features

- 🚀 **Fast** - Written in Rust for optimal performance
- 🔧 **Complete** - 100% API coverage with all 35 endpoints
- 🛡️ **Safe** - Memory safe with strong typing
- 📦 **Standalone** - Single binary, no runtime dependencies
- 🔍 **Discoverable** - Automatic device discovery via mDNS
- 🎨 **User-friendly** - Intuitive CLI with helpful error messages
- 🔄 **Async** - Built on tokio for efficient concurrent operations
- 📝 **Well-documented** - Comprehensive documentation and examples

## Installation

### Quick Install (Recommended)

**Unix/Linux/macOS:**
```bash
curl -sSL https://raw.githubusercontent.com/jeremyeder/awtrix3-rs/main/install.sh | bash
```

**Windows (PowerShell):**
```powershell
iwr -useb https://raw.githubusercontent.com/jeremyeder/awtrix3-rs/main/install.ps1 | iex
```

### Package Managers

**Homebrew (macOS/Linux):**
```bash
brew tap jeremyeder/awtrix3-rs
brew install awtrix3
```

**Cargo (Rust):**
```bash
cargo install awtrix3
```

**npm (Node.js ecosystem):**
```bash
npm install -g @awtrix3/cli
```

### Manual Installation

**Pre-built binaries:**
Download pre-built binaries from the [releases page](https://github.com/jeremyeder/awtrix3-rs/releases).

**From source:**
```bash
git clone https://github.com/jeremyeder/awtrix3-rs
cd awtrix3-rs
cargo install --path .
```

📋 **See [INSTALL.md](INSTALL.md) for detailed installation instructions and troubleshooting.**

## Quick Start

### Basic Usage

```bash
# Send a notification
awtrix notify "Hello, World!" --icon 1234 --color "#FF0000"

# Control display power
awtrix power on
awtrix power off

# Set mood lighting
awtrix display mood --kelvin 2700 --brightness 150

# Manage apps
awtrix app list
awtrix app switch Weather
awtrix app next
```

### Configuration

Create a configuration file at `~/.config/awtrix3/config.toml`:

```toml
default_device = "living_room"

[devices.living_room]
host = "192.168.1.100"
name = "Living Room Display"

[devices.bedroom]
host = "awtrix-bedroom.local"
name = "Bedroom Display"
```

Then use device names in commands:

```bash
awtrix --device bedroom notify "Good night!"
```

### Advanced Examples

#### Complex Notifications

```bash
# With progress bar
awtrix notify "Download Progress" --progress 75 --icon 123

# With sound and effects
awtrix notify "Alert!" --sound "notification" --color "#FF0000" --wakeup

# From JSON file
cat << EOF > notification.json
{
  "text": "Custom Notification",
  "icon": 487,
  "color": [255, 0, 0],
  "duration": 10,
  "effect": "fade",
  "progress": 50
}
EOF
awtrix notify --file notification.json
```

#### Custom Apps

```bash
# Create a custom app
awtrix custom create "MyApp" --text "Custom Text" --icon 5678 --duration 30

# Update custom app from file
awtrix custom watch "StockTicker" --file stocks.json --interval 60
```

#### Device Discovery

```bash
# Discover AWTRIX3 devices on your network
awtrix device discover

# Add discovered device
awtrix device add kitchen 192.168.1.105 --default

# Test connection
awtrix device test kitchen
```

## Library Usage

Use awtrix3-rs as a library in your Rust projects:

```rust
use awtrix3::{Client, Notification};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client
    let client = Client::new("192.168.1.100")?;
    
    // Send notification
    client.notify(
        Notification::builder()
            .text("Temperature: 23°C")
            .icon(487)
            .color("#00FF00")
            .duration(10)
            .build()
    ).await?;
    
    // Control power
    client.set_power(true).await?;
    
    // Get stats
    let stats = client.get_stats().await?;
    println!("Temperature: {}°C", stats.temperature);
    
    Ok(())
}
```

## API Coverage

This CLI provides 100% coverage of all AWTRIX3 HTTP API endpoints:

- ✅ Power & Sleep Control
- ✅ System Management (reboot, update, reset)
- ✅ Information Queries (stats, version, effects)
- ✅ App Management
- ✅ Notifications
- ✅ Custom Apps
- ✅ Display Control (mood light, indicators)
- ✅ Sound Playback
- ✅ Settings Management

## Shell Completions

Generate shell completions for your shell:

```bash
# Bash
awtrix completions bash > ~/.local/share/bash-completion/completions/awtrix

# Zsh
awtrix completions zsh > ~/.zfunc/_awtrix

# Fish
awtrix completions fish > ~/.config/fish/completions/awtrix.fish

# PowerShell
awtrix completions powershell > _awtrix.ps1
```

## Contributing

Contributions are welcome! Please read our [Contributing Guide](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

### Development

```bash
# Clone the repository
git clone https://github.com/jeremyeder/awtrix3-rs
cd awtrix3-rs

# Run tests
cargo test

# Run with verbose logging
RUST_LOG=debug cargo run -- --device 192.168.1.100 info version

# Run benchmarks
cargo bench

# Generate documentation
cargo doc --open
```

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Acknowledgments

- Built for [AWTRIX3](https://github.com/Blueforcer/awtrix3) by Blueforcer
- Inspired by the need for a fast, reliable CLI tool for AWTRIX3 devices
- Thanks to all contributors and the Rust community

## Links

- [Documentation](https://docs.rs/awtrix3)
- [Crates.io](https://crates.io/crates/awtrix3)
- [GitHub Repository](https://github.com/jeremyeder/awtrix3-rs)
- [AWTRIX3 Project](https://github.com/Blueforcer/awtrix3)