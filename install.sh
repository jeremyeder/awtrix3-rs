#!/bin/bash

# AWTRIX3 CLI Installation Script
# This script installs the latest version of awtrix3 CLI

set -e

# Default installation directory
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"
BIN_NAME="awtrix3"
REPO="jeremyeder/awtrix3-rs"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Detect OS and architecture
detect_platform() {
    local os
    local arch
    
    case "$(uname -s)" in
        Linux*)  os="linux" ;;
        Darwin*) os="macos" ;;
        CYGWIN*|MINGW*|MSYS*) os="windows" ;;
        *) log_error "Unsupported operating system: $(uname -s)"; exit 1 ;;
    esac
    
    case "$(uname -m)" in
        x86_64|amd64) arch="x86_64" ;;
        aarch64|arm64) arch="aarch64" ;;
        *) log_error "Unsupported architecture: $(uname -m)"; exit 1 ;;
    esac
    
    # Special handling for macOS ARM
    if [[ "$os" == "macos" && "$arch" == "aarch64" ]]; then
        PLATFORM="aarch64-apple-darwin"
    elif [[ "$os" == "macos" ]]; then
        PLATFORM="x86_64-apple-darwin"
    elif [[ "$os" == "linux" ]]; then
        PLATFORM="x86_64-unknown-linux-gnu"
    elif [[ "$os" == "windows" ]]; then
        PLATFORM="x86_64-pc-windows-msvc"
        BIN_NAME="awtrix3.exe"
    fi
    
    log_info "Detected platform: $PLATFORM"
}

# Get latest release version
get_latest_version() {
    log_info "Fetching latest release information..."
    
    if command -v curl >/dev/null 2>&1; then
        VERSION=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
    elif command -v wget >/dev/null 2>&1; then
        VERSION=$(wget -qO- "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
    else
        log_error "Neither curl nor wget is available. Please install one of them."
        exit 1
    fi
    
    if [[ -z "$VERSION" ]]; then
        log_error "Failed to get latest version"
        exit 1
    fi
    
    # Remove 'v' prefix if present
    VERSION=${VERSION#v}
    log_info "Latest version: $VERSION"
}

# Download and install binary
download_and_install() {
    local tmpdir
    tmpdir=$(mktemp -d)
    local archive_name
    local download_url
    
    if [[ "$PLATFORM" == *"windows"* ]]; then
        archive_name="awtrix3-v${VERSION}-${PLATFORM}.zip"
    else
        archive_name="awtrix3-v${VERSION}-${PLATFORM}.tar.gz"
    fi
    
    download_url="https://github.com/$REPO/releases/download/v$VERSION/$archive_name"
    
    log_info "Downloading $archive_name..."
    
    cd "$tmpdir"
    
    if command -v curl >/dev/null 2>&1; then
        curl -LO "$download_url"
    elif command -v wget >/dev/null 2>&1; then
        wget "$download_url"
    else
        log_error "Neither curl nor wget is available"
        exit 1
    fi
    
    log_info "Extracting archive..."
    
    if [[ "$PLATFORM" == *"windows"* ]]; then
        if command -v unzip >/dev/null 2>&1; then
            unzip -q "$archive_name"
        else
            log_error "unzip is required to extract Windows archives"
            exit 1
        fi
    else
        tar -xzf "$archive_name"
    fi
    
    # Install binary
    log_info "Installing $BIN_NAME to $INSTALL_DIR..."
    
    if [[ ! -d "$INSTALL_DIR" ]]; then
        log_warn "$INSTALL_DIR does not exist. Creating directory..."
        sudo mkdir -p "$INSTALL_DIR"
    fi
    
    if [[ -w "$INSTALL_DIR" ]]; then
        cp "$BIN_NAME" "$INSTALL_DIR/"
        chmod +x "$INSTALL_DIR/$BIN_NAME"
    else
        sudo cp "$BIN_NAME" "$INSTALL_DIR/"
        sudo chmod +x "$INSTALL_DIR/$BIN_NAME"
    fi
    
    # Install shell completions if available
    if [[ -d "completions" ]]; then
        log_info "Installing shell completions..."
        install_completions
    fi
    
    # Cleanup
    cd - >/dev/null
    rm -rf "$tmpdir"
    
    log_success "$BIN_NAME installed successfully!"
}

# Install shell completions
install_completions() {
    local comp_dir
    
    # Bash completions
    if [[ -f "completions/awtrix3.bash" ]]; then
        for comp_dir in "/usr/share/bash-completion/completions" "/usr/local/share/bash-completion/completions" "$HOME/.local/share/bash-completion/completions"; do
            if [[ -d "$comp_dir" ]] || mkdir -p "$comp_dir" 2>/dev/null; then
                if [[ -w "$comp_dir" ]]; then
                    cp "completions/awtrix3.bash" "$comp_dir/awtrix3"
                else
                    sudo cp "completions/awtrix3.bash" "$comp_dir/awtrix3" 2>/dev/null || true
                fi
                log_info "Installed bash completions to $comp_dir"
                break
            fi
        done
    fi
    
    # Zsh completions
    if [[ -f "completions/_awtrix3" ]]; then
        for comp_dir in "/usr/share/zsh/site-functions" "/usr/local/share/zsh/site-functions" "$HOME/.local/share/zsh/site-functions"; do
            if [[ -d "$comp_dir" ]] || mkdir -p "$comp_dir" 2>/dev/null; then
                if [[ -w "$comp_dir" ]]; then
                    cp "completions/_awtrix3" "$comp_dir/"
                else
                    sudo cp "completions/_awtrix3" "$comp_dir/" 2>/dev/null || true
                fi
                log_info "Installed zsh completions to $comp_dir"
                break
            fi
        done
    fi
    
    # Fish completions
    if [[ -f "completions/awtrix3.fish" ]]; then
        for comp_dir in "/usr/share/fish/completions" "/usr/local/share/fish/completions" "$HOME/.config/fish/completions"; do
            if [[ -d "$comp_dir" ]] || mkdir -p "$comp_dir" 2>/dev/null; then
                if [[ -w "$comp_dir" ]]; then
                    cp "completions/awtrix3.fish" "$comp_dir/"
                else
                    sudo cp "completions/awtrix3.fish" "$comp_dir/" 2>/dev/null || true
                fi
                log_info "Installed fish completions to $comp_dir"
                break
            fi
        done
    fi
}

# Verify installation
verify_installation() {
    log_info "Verifying installation..."
    
    if command -v "$BIN_NAME" >/dev/null 2>&1; then
        local installed_version
        installed_version=$("$BIN_NAME" --version 2>/dev/null | head -n1 | awk '{print $NF}' || echo "unknown")
        log_success "$BIN_NAME $installed_version is now available in your PATH"
        
        log_info "Try running: $BIN_NAME --help"
        log_info "Or discover devices: $BIN_NAME device discover"
    else
        log_warn "$BIN_NAME was installed but is not in your PATH"
        log_info "You may need to add $INSTALL_DIR to your PATH or restart your shell"
        log_info "Run: export PATH=\"$INSTALL_DIR:\$PATH\""
    fi
}

# Main installation flow
main() {
    echo "AWTRIX3 CLI Installer"
    echo "===================="
    
    # Parse command line arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            --install-dir)
                INSTALL_DIR="$2"
                shift 2
                ;;
            --version)
                VERSION="$2"
                shift 2
                ;;
            --help)
                echo "Usage: $0 [--install-dir DIR] [--version VERSION]"
                echo ""
                echo "Options:"
                echo "  --install-dir DIR    Install directory (default: /usr/local/bin)"
                echo "  --version VERSION    Specific version to install (default: latest)"
                echo "  --help              Show this help message"
                exit 0
                ;;
            *)
                log_error "Unknown option: $1"
                echo "Use --help for usage information"
                exit 1
                ;;
        esac
    done
    
    # Check if running as root (not recommended)
    if [[ $EUID -eq 0 ]]; then
        log_warn "Running as root is not recommended"
        log_warn "Consider running as a regular user with sudo access"
        read -p "Continue anyway? (y/N) " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            exit 1
        fi
    fi
    
    detect_platform
    
    if [[ -z "$VERSION" ]]; then
        get_latest_version
    else
        log_info "Installing version: $VERSION"
    fi
    
    download_and_install
    verify_installation
    
    echo ""
    log_success "Installation complete!"
    echo ""
    echo "Next steps:"
    echo "1. Add a device: $BIN_NAME device add <name> <host>"
    echo "2. Send a notification: $BIN_NAME notify \"Hello, World!\""
    echo "3. View all commands: $BIN_NAME --help"
    echo ""
    echo "Documentation: https://github.com/$REPO"
}

main "$@"