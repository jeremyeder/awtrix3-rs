# Contributing to awtrix3-rs

Thank you for your interest in contributing to awtrix3-rs! This document provides guidelines and information for contributors.

## Code of Conduct

This project adheres to a code of conduct. By participating, you are expected to uphold this code. Please report unacceptable behavior to the project maintainers.

## Getting Started

### Prerequisites

- Rust 1.74 or later
- Git
- An AWTRIX3 device for testing (optional but recommended)

### Setting up the Development Environment

1. Fork the repository on GitHub
2. Clone your fork:
   ```bash
   git clone https://github.com/yourusername/awtrix3-rs.git
   cd awtrix3-rs
   ```
3. Add the upstream repository:
   ```bash
   git remote add upstream https://github.com/jeremyeder/awtrix3-rs.git
   ```
4. Install dependencies:
   ```bash
   cargo build
   ```
5. Run tests to ensure everything works:
   ```bash
   cargo test
   ```

## Development Workflow

### Making Changes

1. Create a new branch for your feature or bugfix:
   ```bash
   git checkout -b feature/your-feature-name
   ```
2. Make your changes
3. Add tests for your changes
4. Ensure all tests pass:
   ```bash
   cargo test
   ```
5. Format your code:
   ```bash
   cargo fmt
   ```
6. Run clippy to check for issues:
   ```bash
   cargo clippy -- -D warnings
   ```
7. Commit your changes with a descriptive commit message

### Commit Messages

Use conventional commit format:
- `feat:` for new features
- `fix:` for bug fixes
- `docs:` for documentation changes
- `test:` for test additions/changes
- `refactor:` for code refactoring
- `style:` for formatting changes
- `chore:` for maintenance tasks

Example:
```
feat: add support for custom app deletion

- Implement DELETE /api/custom endpoint
- Add CLI command for deleting custom apps
- Include tests for deletion functionality
```

### Testing

- Write unit tests for new functionality
- Write integration tests for API endpoints
- Use `cargo test` to run the test suite
- Aim for high test coverage

#### Testing with a Real Device

If you have an AWTRIX3 device:

1. Set the device IP in environment variable:
   ```bash
   export AWTRIX_DEVICE=192.168.1.100
   ```
2. Run integration tests:
   ```bash
   cargo test --features integration-tests
   ```

### Documentation

- Document all public APIs with rustdoc comments
- Include examples in documentation
- Update README.md if adding new features
- Use `cargo doc --open` to view generated documentation

## Code Style

### Rust Guidelines

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` to format code
- Address all `cargo clippy` warnings
- Use descriptive variable and function names
- Add documentation for all public items

### Project-Specific Guidelines

- Use `anyhow::Result` for functions that can fail
- Prefer async/await over blocking operations
- Use builder patterns for complex structures
- Include comprehensive error handling
- Write integration tests for all CLI commands

## Pull Request Process

1. Ensure your branch is up to date with main:
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```
2. Push your branch to your fork:
   ```bash
   git push origin feature/your-feature-name
   ```
3. Create a pull request on GitHub
4. Ensure the PR description clearly describes the changes
5. Link any related issues
6. Ensure all CI checks pass

### PR Checklist

- [ ] Tests pass locally
- [ ] Code is formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Documentation is updated if needed
- [ ] CHANGELOG.md is updated (for significant changes)
- [ ] Breaking changes are clearly marked

## Issue Guidelines

### Bug Reports

Include:
- OS and Rust version
- Steps to reproduce
- Expected vs actual behavior
- Error messages or logs
- AWTRIX3 firmware version (if applicable)

### Feature Requests

Include:
- Use case description
- Proposed API (if applicable)
- Examples of usage
- Any alternative solutions considered

## Release Process

Releases are managed by maintainers:

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Create a git tag
4. GitHub Actions will build and publish releases

## Getting Help

- Open an issue for bugs or feature requests
- Start a discussion for questions or ideas
- Join our community chat (if available)

## Recognition

Contributors will be recognized in:
- The project's contributors list
- Release notes for significant contributions
- The project README (for major contributors)

## License

By contributing, you agree that your contributions will be licensed under the same license as the project (MIT OR Apache-2.0).

Thank you for contributing to awtrix3-rs!