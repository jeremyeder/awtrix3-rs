# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Structure

This repository contains `awtrix3-rs`, a modern Rust CLI and library for controlling AWTRIX3 LED matrix displays. The project is located in the `awtrix3-rs/` subdirectory.

**Core Architecture:**
- `src/lib.rs` - Library entry point with public API exports (Client, models, error handling)
- `src/main.rs` - CLI application entry point with async tokio runtime
- `src/cli/` - Command-line interface using clap with comprehensive subcommands
- `src/client/` - HTTP client implementation for AWTRIX3 API
- `src/commands/` - Command handlers for each CLI subcommand (apps, custom, device, display, etc.)
- `src/models/` - Data structures for AWTRIX3 API (notifications, colors, effects, settings)
- `src/config/` - Configuration management with TOML support
- `src/utils/` - Utilities including mDNS device discovery

**Key Design Patterns:**
- Builder pattern for complex objects (Notification, CustomApp)
- Async/await throughout using tokio runtime
- Comprehensive error handling with anyhow and thiserror
- Structured CLI with nested subcommands for different device functions
- Both library and CLI interfaces for the same functionality

## Development Commands

**Building and Testing:**
```bash
cargo build                                    # Build in debug mode
cargo build --release                         # Build optimized release
cargo test                                    # Run all tests
cargo test --all-features                     # Run tests with all features
cargo test --doc --all-features               # Run documentation tests
```

**Code Quality:**
```bash
cargo fmt --all -- --check                    # Check code formatting
cargo clippy --all-targets --all-features -- -D warnings  # Run linter
cargo doc --no-deps --all-features            # Generate documentation
```

**Development Tools:**
```bash
cargo bench                                   # Run benchmarks
RUST_LOG=debug cargo run -- --device 192.168.1.100 info version  # Debug logging
```

**Installation:**
```bash
cargo install --path .                        # Install from source
```

## Configuration

The CLI uses a TOML configuration file at `~/.config/awtrix3/config.toml`:
```toml
default_device = "living_room"

[devices.living_room]
host = "192.168.1.100"
name = "Living Room Display"
```

## Key Dependencies

- **clap 4.5** - Command-line interface with derive macros
- **tokio 1.38** - Async runtime with multi-threading
- **reqwest 0.12** - HTTP client with rustls-tls
- **serde/serde_json** - Serialization for API communication
- **anyhow/thiserror** - Error handling
- **tracing** - Structured logging

## Testing Strategy

The project uses multiple testing approaches:
- Unit tests in individual modules
- Integration tests in `tests/` directory
- Property testing with proptest
- HTTP mocking with wiremock and mockito
- Benchmarks with criterion

## Release Configuration

Optimized for size and performance:
- LTO enabled
- Strip symbols
- Optimize for size (`opt-level = "z"`)
- Single codegen unit for maximum optimization

## CLI Usage Patterns

The CLI supports device resolution in this priority order:
1. `--device` command line argument
2. `AWTRIX_DEVICE` environment variable  
3. Default device from config file

All commands support JSON output with `--json` flag for scripting.