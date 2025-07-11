# Installation Guide

This guide provides multiple ways to install the AWTRIX3 CLI tool.

## Quick Install (Recommended)

### Unix/Linux/macOS
```bash
curl -sSL https://raw.githubusercontent.com/jeremyeder/awtrix3-rs/main/install.sh | bash
```

### Windows (PowerShell)
```powershell
iwr -useb https://raw.githubusercontent.com/jeremyeder/awtrix3-rs/main/install.ps1 | iex
```

## Package Managers

### Homebrew (macOS/Linux)
```bash
# Add the tap
brew tap jeremyeder/awtrix3-rs

# Install the CLI
brew install awtrix3
```

### Cargo (Rust)
```bash
cargo install awtrix3
```

### npm (Node.js ecosystem)
```bash
npm install -g @awtrix3/cli
```

### Arch Linux (AUR)
```bash
# Using yay
yay -S awtrix3

# Using paru  
paru -S awtrix3
```

### Scoop (Windows)
```powershell
# Add bucket
scoop bucket add awtrix3 https://github.com/jeremyeder/awtrix3-rs-scoop

# Install
scoop install awtrix3
```

## Manual Installation

### Download Pre-built Binaries

1. Go to the [releases page](https://github.com/jeremyeder/awtrix3-rs/releases/latest)
2. Download the appropriate archive for your platform:
   - **Linux (x64)**: `awtrix3-v{version}-x86_64-unknown-linux-gnu.tar.gz`
   - **macOS (Intel)**: `awtrix3-v{version}-x86_64-apple-darwin.tar.gz`
   - **macOS (Apple Silicon)**: `awtrix3-v{version}-aarch64-apple-darwin.tar.gz`
   - **Windows (x64)**: `awtrix3-v{version}-x86_64-pc-windows-msvc.zip`

3. Extract the archive:
   ```bash
   # Linux/macOS
   tar -xzf awtrix3-v{version}-{platform}.tar.gz
   
   # Windows (PowerShell)
   Expand-Archive awtrix3-v{version}-{platform}.zip
   ```

4. Move the binary to a directory in your PATH:
   ```bash
   # Linux/macOS
   sudo mv awtrix3 /usr/local/bin/
   
   # Windows
   move awtrix3.exe C:\Windows\System32\
   ```

### Build from Source

Requirements:
- Rust 1.70+ with Cargo
- Git

```bash
# Clone the repository
git clone https://github.com/jeremyeder/awtrix3-rs.git
cd awtrix3-rs

# Build and install
cargo install --path .

# Or build without installing
cargo build --release
```

## Shell Completions

After installation, you can generate shell completions:

### Bash
```bash
awtrix3 completions bash > ~/.local/share/bash-completion/completions/awtrix3
```

### Zsh
```bash
awtrix3 completions zsh > ~/.zfunc/_awtrix3
# Add ~/.zfunc to your $fpath in ~/.zshrc if not already there
```

### Fish
```bash
awtrix3 completions fish > ~/.config/fish/completions/awtrix3.fish
```

### PowerShell (Windows)
```powershell
awtrix3 completions powershell > $PROFILE\..\awtrix3.ps1
# Add to your PowerShell profile to source the completions
```

## Verification

Verify the installation by running:

```bash
awtrix3 --version
awtrix3 --help
```

## Configuration

After installation, create your first device configuration:

```bash
# Discover devices on your network
awtrix3 device discover

# Add a device
awtrix3 device add my-display 192.168.1.100 --default

# Test the connection
awtrix3 device test my-display

# Send your first notification
awtrix3 notify "Hello, AWTRIX3!"
```

## Docker

You can also run the CLI in a Docker container:

```bash
# Build the image
docker build -t awtrix3-cli .

# Run a command
docker run --rm awtrix3-cli device discover

# Use with host networking for device discovery
docker run --rm --network host awtrix3-cli device discover
```

## Troubleshooting

### Permission Issues
If you encounter permission issues during installation:

```bash
# Create a local bin directory
mkdir -p ~/.local/bin

# Add to PATH in your shell profile (~/.bashrc, ~/.zshrc, etc.)
export PATH="$HOME/.local/bin:$PATH"

# Install to local directory
curl -sSL https://raw.githubusercontent.com/jeremyeder/awtrix3-rs/main/install.sh | INSTALL_DIR="$HOME/.local/bin" bash
```

### Windows Execution Policy
If you get an execution policy error on Windows:

```powershell
# Temporarily allow script execution
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser

# Run the installer
iwr -useb https://raw.githubusercontent.com/jeremyeder/awtrix3-rs/main/install.ps1 | iex

# Restore execution policy (optional)
Set-ExecutionPolicy -ExecutionPolicy Restricted -Scope CurrentUser
```

### Network Issues
If you're behind a corporate firewall or proxy:

```bash
# Set proxy for curl
export https_proxy=http://proxy.company.com:8080

# Or download manually and install
wget https://github.com/jeremyeder/awtrix3-rs/releases/latest/download/awtrix3-{platform}.tar.gz
```

## Updating

### Automatic Update
```bash
# Re-run the install script to get the latest version
curl -sSL https://raw.githubusercontent.com/jeremyeder/awtrix3-rs/main/install.sh | bash
```

### Package Manager Update
```bash
# Homebrew
brew upgrade awtrix3

# Cargo
cargo install awtrix3 --force

# npm
npm update -g @awtrix3/cli
```

### Manual Update
Download the latest release and replace the existing binary following the manual installation steps.

## Uninstallation

### Remove Binary
```bash
# Find the installation location
which awtrix3

# Remove the binary
sudo rm /usr/local/bin/awtrix3  # or wherever it's installed
```

### Remove Configuration
```bash
# Remove configuration directory
rm -rf ~/.config/awtrix3
```

### Package Manager Removal
```bash
# Homebrew
brew uninstall awtrix3

# Cargo
cargo uninstall awtrix3

# npm
npm uninstall -g @awtrix3/cli
```

## Support

If you encounter any issues during installation:

1. Check the [troubleshooting section](#troubleshooting)
2. Search existing [GitHub issues](https://github.com/jeremyeder/awtrix3-rs/issues)
3. Create a new issue with your system information and error details

## System Requirements

- **Operating System**: Linux, macOS, Windows
- **Architecture**: x86_64 (AMD64), ARM64/AArch64 (macOS only)
- **Network**: Access to AWTRIX3 devices on local network
- **Dependencies**: None (statically linked binary)