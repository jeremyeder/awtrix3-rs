use assert_cmd::Command;
use predicates::prelude::*;

/// Test basic CLI functionality and help system
#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("awtrix3").unwrap();
    
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("A modern CLI for controlling AWTRIX3"));
}

/// Test CLI version information
#[test]
fn test_cli_version() {
    let mut cmd = Command::cargo_bin("awtrix3").unwrap();
    
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("awtrix3"));
}

/// Test notification command help
#[test]
fn test_notify_help() {
    let mut cmd = Command::cargo_bin("awtrix3").unwrap();
    
    cmd.args(&["notify", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Send notifications"));
}

/// Test system command help
#[test]
fn test_system_help() {
    let mut cmd = Command::cargo_bin("awtrix3").unwrap();
    
    cmd.args(&["system", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("System commands"));
}

/// Test device command help
#[test]
fn test_device_help() {
    let mut cmd = Command::cargo_bin("awtrix3").unwrap();
    
    cmd.args(&["device", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Device discovery and management"));
}

/// Test settings command help
#[test]
fn test_settings_help() {
    let mut cmd = Command::cargo_bin("awtrix3").unwrap();
    
    cmd.args(&["settings", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Settings management"));
}

/// Test that settings list command works without device
#[test]
fn test_settings_list_no_device() {
    let mut cmd = Command::cargo_bin("awtrix3").unwrap();
    
    // Settings list should work without connecting to a device
    // since it just shows documentation
    cmd.args(&["settings", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Available AWTRIX3 Settings"));
}

/// Test error handling for missing device
#[test]
fn test_missing_device_error() {
    let mut cmd = Command::cargo_bin("awtrix3").unwrap();
    
    // Should fail gracefully when no device is specified
    cmd.args(&["notify", "test"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("No device specified"));
}

/// Test notification file parsing (without sending)
#[test]
fn test_notify_file_validation() {
    let mut cmd = Command::cargo_bin("awtrix3").unwrap();
    
    // Test with a non-existent file - should fail with file not found
    cmd.args(&["--device", "127.0.0.1", "notify", "--file", "nonexistent.json"])
        .assert()
        .failure();
}

/// Test shell completion generation
#[test]
fn test_shell_completions() {
    let shells = ["bash", "zsh", "fish", "powershell"];
    
    for shell in &shells {
        let mut cmd = Command::cargo_bin("awtrix3").unwrap();
        
        cmd.args(&["completions", shell])
            .assert()
            .success();
    }
}

/// Test invalid command arguments
#[test]
fn test_invalid_arguments() {
    let mut cmd = Command::cargo_bin("awtrix3").unwrap();
    
    // Test with invalid subcommand
    cmd.arg("invalid-command")
        .assert()
        .failure();
}

/// Test JSON output flag propagation
#[test]
fn test_json_flag() {
    let mut cmd = Command::cargo_bin("awtrix3").unwrap();
    
    // Test that global JSON flag is accepted (will fail on device connection)
    cmd.args(&["--json", "--device", "127.0.0.1", "notify", "test"])
        .assert()
        .failure(); // Expected to fail due to connection, but should parse args correctly
}