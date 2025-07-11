# Release v{VERSION}

## 🎉 What's New

### ✨ New Features
- Feature 1: Brief description
- Feature 2: Brief description
- Feature 3: Brief description

### 🐛 Bug Fixes
- Fix 1: Brief description
- Fix 2: Brief description

### 🔧 Improvements
- Improvement 1: Brief description
- Improvement 2: Brief description

### 📦 Dependencies
- Updated dependency X to version Y
- Added new dependency Z for feature A

## 📚 Documentation
- Updated installation guide
- Added new examples for feature X
- Improved API documentation

## 🚀 Installation

### Quick Install

**Unix/Linux/macOS:**
```bash
curl -sSL https://raw.githubusercontent.com/jeremyeder/awtrix3-rs/main/install.sh | bash
```

**Windows (PowerShell):**
```powershell
iwr -useb https://raw.githubusercontent.com/jeremyeder/awtrix3-rs/main/install.ps1 | iex
```

### Package Managers

**Homebrew:**
```bash
brew tap jeremyeder/awtrix3-rs
brew install awtrix3
```

**Cargo:**
```bash
cargo install awtrix3
```

**From Binary:**
Download the appropriate binary for your platform from the assets below.

## 🔄 Upgrade Instructions

If you're upgrading from a previous version:

1. **Via Package Manager:**
   ```bash
   # Homebrew
   brew upgrade awtrix3
   
   # Cargo
   cargo install awtrix3 --force
   ```

2. **Via Install Script:**
   ```bash
   # The install script will automatically replace the existing binary
   curl -sSL https://raw.githubusercontent.com/jeremyeder/awtrix3-rs/main/install.sh | bash
   ```

3. **Manual Upgrade:**
   - Download the new binary
   - Replace the existing binary in your PATH

## ⚠️ Breaking Changes

> **Note:** This section only applies if there are breaking changes in this release.

- Breaking change 1: Description and migration steps
- Breaking change 2: Description and migration steps

### Migration Guide

If you're upgrading from version X.Y.Z, please note the following changes:

1. **Configuration Changes:**
   - Old format: `old_setting = "value"`
   - New format: `new_setting = "value"`

2. **CLI Changes:**
   - Command `old-command` is now `new-command`
   - Option `--old-flag` is now `--new-flag`

## 🧪 What's Been Tested

This release has been tested on:
- ✅ Linux (Ubuntu 20.04+, Debian 11+, CentOS 8+)
- ✅ macOS (10.15+, Intel and Apple Silicon)
- ✅ Windows (Windows 10+, PowerShell 5.1+)

## 📊 Performance

Performance improvements in this release:
- Startup time: X% faster
- Memory usage: Y% reduction
- Binary size: Z KB (±N% from previous version)

## 🔐 Security

- Security audit completed with no critical issues
- Dependencies updated to latest secure versions
- No known vulnerabilities in this release

## 🙋 Getting Help

- 📖 **Documentation:** [docs.rs/awtrix3](https://docs.rs/awtrix3)
- 🐛 **Issues:** [GitHub Issues](https://github.com/jeremyeder/awtrix3-rs/issues)
- 💬 **Discussions:** [GitHub Discussions](https://github.com/jeremyeder/awtrix3-rs/discussions)

## 🎯 Quick Start

After installation, try these commands:

```bash
# Discover AWTRIX3 devices on your network
awtrix3 device discover

# Add a device
awtrix3 device add my-display 192.168.1.100 --default

# Send your first notification
awtrix3 notify "Hello, AWTRIX3!"

# Check device status
awtrix3 info stats
```

## 📈 What's Next

Looking ahead to the next release:
- Feature A: Brief description
- Feature B: Brief description
- Improvement C: Brief description

## 🤝 Contributors

Special thanks to all contributors who made this release possible:
- @contributor1 - Feature/fix description
- @contributor2 - Feature/fix description
- @contributor3 - Feature/fix description

## 📅 Release Timeline

- **Release Candidate:** {RC_DATE}
- **Final Release:** {RELEASE_DATE}
- **Next Release (Planned):** {NEXT_RELEASE_DATE}

---

**Full Changelog:** [v{PREVIOUS_VERSION}...v{VERSION}](https://github.com/jeremyeder/awtrix3-rs/compare/v{PREVIOUS_VERSION}...v{VERSION})

🚀 **Generated with [Claude Code](https://claude.ai/code)**