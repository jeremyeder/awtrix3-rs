# AWTRIX3 CLI Examples

This directory contains example configurations and usage patterns for the AWTRIX3 Rust CLI.

## Directory Structure

- `notifications/` - Example notification configurations
- `custom-apps/` - Example custom app configurations  
- `settings/` - Example device settings profiles

## Notification Examples

### Basic Weather Notification
```bash
cargo run -- notify --file examples/notifications/weather.json
```

### Download Progress with Progress Bar
```bash
cargo run -- notify --file examples/notifications/download-progress.json
```

### Alert with Sound and Hold
```bash
cargo run -- notify --file examples/notifications/alert.json
```

## Custom App Examples

### Stock Ticker App
```bash
cargo run -- custom create "StockTicker" --file examples/custom-apps/stock-ticker.json
```

### Weather App with Progress
```bash
cargo run -- custom create "WeatherApp" --file examples/custom-apps/weather-app.json
```

## Settings Examples

### Gaming Setup (Bright, Fast)
```bash
cargo run -- settings import examples/settings/gaming-setup.json
```

### Bedroom Mode (Dim, Slow)
```bash
cargo run -- settings import examples/settings/bedroom-mode.json
```

## CLI Usage Examples

### Device Management
```bash
# Discover devices on network
cargo run -- device discover --timeout 10

# Add devices
cargo run -- device add living-room 192.168.1.100 --default
cargo run -- device add bedroom awtrix-bedroom.local

# List configured devices
cargo run -- device list

# Test connectivity
cargo run -- device test living-room
```

### Quick Notifications
```bash
# Simple text notification
cargo run -- notify "Hello, World!"

# Notification with icon and color
cargo run -- notify "Temperature: 23°C" --icon 2422 --color "#FFD700"

# Progress notification
cargo run -- notify "Download" --progress 75 --icon 1234

# Alert with sound
cargo run -- notify "System Alert!" --color red --sound error --wakeup
```

### App Management
```bash
# List available apps
cargo run -- app list

# Switch between apps
cargo run -- app next
cargo run -- app previous
cargo run -- app switch "Weather"

# Create custom apps
cargo run -- custom create "MyApp" --text "Custom Text" --icon 1234
cargo run -- custom delete "MyApp"
```

### System Management
```bash
# Get device statistics
cargo run -- system stats

# Control device power
cargo run -- power on
cargo run -- power off
cargo run -- sleep --duration 300

# Device maintenance
cargo run -- system reboot
cargo run -- system backup --output backup.json
```

### Display Control
```bash
# Mood lighting
cargo run -- display mood --color "#FF6B35" --brightness 200
cargo run -- display mood --kelvin 2700 --brightness 150

# LED indicators
cargo run -- indicator 1 --color "red"
cargo run -- indicator all --off
```

### Sound Control
```bash
# Play sounds
cargo run -- sound play "notification"
cargo run -- sound r2d2

# Play RTTTL melodies
cargo run -- sound rtttl "Mario:d=4,o=5,b=100:16e6,16e6,32p,8e6,16c6,8e6,8g6"
```

### Settings Management
```bash
# View current settings
cargo run -- settings get
cargo run -- settings get brightness

# Modify settings
cargo run -- settings set brightness 150
cargo run -- settings set text_color "#FF0000"
cargo run -- settings set time_app.format 2

# Export/import settings
cargo run -- settings export --output my-settings.json
cargo run -- settings import my-settings.json

# List available settings
cargo run -- settings list
```

## Configuration Tips

### Device Configuration
Create `~/.config/awtrix3/config.toml`:
```toml
default_device = "living_room"

[devices.living_room]
host = "192.168.1.100"
name = "Living Room Display"
timeout = 30
retries = 3

[devices.bedroom]
host = "awtrix-bedroom.local"
name = "Bedroom Display"
timeout = 30
retries = 3
```

### Shell Completions
```bash
# Generate shell completions
cargo run -- completions bash > ~/.local/share/bash-completion/completions/awtrix3
cargo run -- completions zsh > ~/.zfunc/_awtrix3
cargo run -- completions fish > ~/.config/fish/completions/awtrix3.fish
```

### JSON Output Mode
Add `--json` flag for scripting:
```bash
# Get settings as JSON
cargo run -- --json settings get > current-settings.json

# Device status as JSON
cargo run -- --json device list > device-status.json
```

## Advanced Examples

### Batch Operations
```bash
# Multiple notifications
for file in examples/notifications/*.json; do
    cargo run -- notify --file "$file"
    sleep 2
done

# Settings profiles
cargo run -- settings import examples/settings/gaming-setup.json
# ... use gaming setup
cargo run -- settings import examples/settings/bedroom-mode.json
```

### File Watching
```bash
# Watch file for continuous updates
cargo run -- custom watch "LiveData" --file data.json --interval 30
```

### Automation Integration
```bash
#!/bin/bash
# Home automation script
TEMP=$(sensors | grep "Core 0" | awk '{print $3}' | tr -d '+°C')
cargo run -- notify "CPU Temp: ${TEMP}°C" --icon 1234 --color "#FF6B35"
```