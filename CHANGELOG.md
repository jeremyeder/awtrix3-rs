# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Comprehensive release process and distribution system
- Multi-platform installation scripts (Unix/Linux/macOS and Windows)
- Homebrew formula for macOS/Linux package management
- GitHub Actions workflows for automated releases
- API documentation deployment via GitHub Pages
- Pre-release validation workflow with comprehensive testing
- Shell completion support for bash, zsh, fish, and PowerShell
- Professional installation documentation (INSTALL.md)
- Release preparation checklist for maintainers (RELEASE.md)

### Changed
- Enhanced README.md with comprehensive installation instructions
- Improved documentation structure and organization

## [1.0.0] - 2025-07-11

### Added
- Initial project setup
- Complete AWTRIX3 HTTP API client implementation (100% coverage)
- CLI with all 35 endpoints supported
- Device discovery via mDNS
- Configuration file support
- Shell completion generation
- Comprehensive error handling
- Async/await support throughout
- JSON output mode for scripting
- Color parsing utilities
- Builder patterns for complex structures
- Production-ready Cargo.toml with optimizations
- Comprehensive test suite
- CI/CD pipeline with GitHub Actions
- Documentation and examples

### Features

#### Core Library
- `Client` - HTTP client for AWTRIX3 devices
- `ClientBuilder` - Flexible client configuration
- Complete model definitions for all API types
- Strong typing with Rust's type system
- Connection pooling and retry logic

#### CLI Commands
- `power` - Power management (on/off)
- `sleep` - Sleep mode control
- `system` - System operations (reboot, update, stats, etc.)
- `info` - Device information queries
- `app` - App management (list, switch, reorder)
- `notify` - Send notifications with full feature support
- `custom` - Custom app management
- `display` - Display control (mood light, indicators)
- `sound` - Sound playback (files, RTTTL, R2D2)
- `indicator` - LED indicator control
- `settings` - Device settings management
- `device` - Device discovery and configuration

#### Advanced Features
- Device profiles in configuration
- Automatic device discovery
- Watch mode for continuous updates
- Batch operations
- Progress bars and colored output
- Shell completions for all major shells
- JSON mode for scripting integration

### Dependencies
- `clap` 4.5 - CLI framework with derive macros
- `tokio` 1.38 - Async runtime
- `reqwest` 0.12 - HTTP client with rustls
- `serde` 1.0 - Serialization framework
- `anyhow` 1.0 - Error handling
- `thiserror` 1.0 - Custom error types
- `colored` 2.1 - Terminal colors
- `indicatif` 0.17 - Progress bars
- `tabled` 0.15 - Table formatting
- `tracing` 0.1 - Structured logging
- `mdns-sd` 0.11 - Device discovery (optional)

### Documentation
- Comprehensive README with examples
- API documentation with rustdoc
- Contributing guidelines
- MIT/Apache dual licensing
- Changelog following Keep a Changelog format

### CI/CD
- Multi-platform testing (Linux, Windows, macOS)
- Code formatting with rustfmt
- Linting with clippy
- Security auditing
- Code coverage reporting
- MSRV (Minimum Supported Rust Version) checking
- Documentation building
- Release automation

[Unreleased]: https://github.com/jeremyeder/awtrix3-rs/compare/v0.1.0...HEAD