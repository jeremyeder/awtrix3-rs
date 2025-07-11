# Release Process

This document outlines the release process for awtrix3-rs.

## Pre-Release Checklist

### Code Quality
- [ ] All tests pass (`cargo test`)
- [ ] No compilation warnings (`cargo check`)
- [ ] Documentation builds without warnings (`cargo doc`)
- [ ] Code is formatted (`cargo fmt --check`)
- [ ] Linting passes (`cargo clippy -- -D warnings`)
- [ ] Examples in documentation work correctly

### Version Management
- [ ] Update version in `Cargo.toml`
- [ ] Update version in `Cargo.lock` (`cargo update`)
- [ ] Update version references in documentation
- [ ] Update `CHANGELOG.md` with new version and changes

### Release Notes
- [ ] Prepare release notes highlighting:
  - New features
  - Bug fixes
  - Breaking changes (if any)
  - Performance improvements
  - Dependencies updates
- [ ] Include installation/upgrade instructions
- [ ] Add any migration notes for breaking changes

### Testing
- [ ] Test installation scripts on clean systems:
  - [ ] Linux (install.sh)
  - [ ] macOS (install.sh + Homebrew formula)
  - [ ] Windows (install.ps1)
- [ ] Test pre-built binaries on target platforms
- [ ] Verify shell completions work correctly
- [ ] Test core functionality with real AWTRIX3 device

## Release Process

### 1. Create Release Candidate

```bash
# Update version to release candidate (e.g., 1.0.0-rc.1)
cargo edit --manifest-path Cargo.toml --set-version 1.0.0-rc.1

# Commit and create RC tag
git add .
git commit -m "Release candidate 1.0.0-rc.1"
git tag -a v1.0.0-rc.1 -m "Release candidate 1.0.0-rc.1"
git push origin v1.0.0-rc.1
```

### 2. Validate Release Candidate

- [ ] GitHub Actions build succeeds for all platforms
- [ ] Release artifacts are created and uploaded
- [ ] Installation scripts work with RC binaries
- [ ] No critical issues reported during RC testing period

### 3. Create Final Release

```bash
# Update to final version
cargo edit --manifest-path Cargo.toml --set-version 1.0.0

# Update changelog
echo "## [1.0.0] - $(date +%Y-%m-%d)" >> CHANGELOG_NEW.md
echo "" >> CHANGELOG_NEW.md
echo "### Added" >> CHANGELOG_NEW.md
echo "- Initial release" >> CHANGELOG_NEW.md
echo "" >> CHANGELOG_NEW.md
cat CHANGELOG.md >> CHANGELOG_NEW.md
mv CHANGELOG_NEW.md CHANGELOG.md

# Commit and create release tag
git add .
git commit -m "🎉 Release v1.0.0

🚀 Generated with [Claude Code](https://claude.ai/code)

Co-Authored-By: Claude <noreply@anthropic.com>"
git tag -a v1.0.0 -m "Release v1.0.0"
git push origin main
git push origin v1.0.0
```

### 4. Post-Release Tasks

- [ ] Verify crates.io publication succeeded
- [ ] Update Homebrew formula with new version and checksums
- [ ] Update documentation links and badges
- [ ] Announce release on relevant channels
- [ ] Update issue/PR templates if needed

## Hotfix Process

For critical bug fixes that need immediate release:

1. Create hotfix branch from release tag:
   ```bash
   git checkout -b hotfix/1.0.1 v1.0.0
   ```

2. Apply minimal fix and update version:
   ```bash
   cargo edit --manifest-path Cargo.toml --set-version 1.0.1
   ```

3. Test thoroughly and create pull request to main

4. After merge, tag and release:
   ```bash
   git tag -a v1.0.1 -m "Hotfix v1.0.1"
   git push origin v1.0.1
   ```

## Release Automation

The release process is automated via GitHub Actions:

- **Trigger**: Push of version tag (e.g., `v1.0.0`)
- **Builds**: Cross-platform binaries for Linux, macOS, Windows
- **Artifacts**: 
  - Compressed binaries with shell completions
  - SHA256 checksums
  - Installation scripts
  - Homebrew formula
- **Publishing**: Automatic publication to crates.io
- **Documentation**: API docs deployed to GitHub Pages

## Emergency Procedures

### Yanking a Release

If a critical issue is discovered after release:

```bash
# Yank from crates.io (prevents new installs, doesn't break existing)
cargo yank --vers 1.0.0

# Optionally un-yank after fix
cargo yank --vers 1.0.0 --undo
```

### Rolling Back

1. Create new version with fix
2. Do not delete GitHub releases (breaks existing links)
3. Mark problematic release as "pre-release" in GitHub
4. Update documentation to point to safe version

## Version Numbering

We follow [Semantic Versioning](https://semver.org/):

- **MAJOR** (1.0.0): Breaking changes
- **MINOR** (1.1.0): New features, backward compatible
- **PATCH** (1.0.1): Bug fixes, backward compatible
- **Pre-release** (1.0.0-rc.1): Release candidates, alphas, betas

## Security Releases

For security-related releases:

1. Follow responsible disclosure practices
2. Coordinate with security team if applicable
3. Include security advisory in release notes
4. Consider backporting fixes to older supported versions
5. Notify users through multiple channels

## Dependencies

### Required Tools
- `cargo-edit` for version management: `cargo install cargo-edit`
- `git` for version control
- Access to GitHub repository with push permissions
- crates.io API token (for manual publishing)

### Secrets Required
- `CARGO_REGISTRY_TOKEN`: For crates.io publishing
- `GITHUB_TOKEN`: For release creation (automatically provided)

## Support Matrix

Current supported platforms for releases:

| Platform | Architecture | Status |
|----------|-------------|--------|
| Linux    | x86_64      | ✅ Tier 1 |
| macOS    | x86_64      | ✅ Tier 1 |
| macOS    | ARM64       | ✅ Tier 1 |
| Windows  | x86_64      | ✅ Tier 1 |

Tier 1: Full support, automated testing, pre-built binaries