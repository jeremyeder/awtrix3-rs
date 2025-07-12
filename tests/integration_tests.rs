use awtrix3::{CustomApp, Notification, Settings};
use std::fs;

/// Test that all example notification files are valid JSON and parse correctly
#[test]
fn test_notification_examples() {
    let examples_dir = "examples/notifications";

    // Test each notification example
    let examples = ["weather.json", "download-progress.json", "alert.json"];

    for example in &examples {
        let path = format!("{}/{}", examples_dir, example);

        if let Ok(content) = fs::read_to_string(&path) {
            let notification: Result<Notification, _> = serde_json::from_str(&content);
            assert!(
                notification.is_ok(),
                "Example notification should parse correctly: {} - Error: {:?}",
                example,
                notification.err()
            );

            // Verify the notification has at least text
            if let Ok(notification) = notification {
                assert!(
                    notification.text.is_some(),
                    "Notification example should have text: {}",
                    example
                );
            }
        }
    }
}

/// Test that all custom app examples are valid JSON and parse correctly
#[test]
fn test_custom_app_examples() {
    let examples_dir = "examples/custom-apps";
    let examples = ["stock-ticker.json", "weather-app.json"];

    for example in &examples {
        let path = format!("{}/{}", examples_dir, example);

        if let Ok(content) = fs::read_to_string(&path) {
            let app: Result<CustomApp, _> = serde_json::from_str(&content);
            assert!(
                app.is_ok(),
                "Custom app example should parse correctly: {} - Error: {:?}",
                example,
                app.err()
            );
        }
    }
}

/// Test that all settings examples are valid JSON and parse correctly
#[test]
fn test_settings_examples() {
    let examples_dir = "examples/settings";
    let examples = ["gaming-setup.json", "bedroom-mode.json"];

    for example in &examples {
        let path = format!("{}/{}", examples_dir, example);

        if let Ok(content) = fs::read_to_string(&path) {
            let settings: Result<Settings, _> = serde_json::from_str(&content);
            assert!(
                settings.is_ok(),
                "Settings example should parse correctly: {} - Error: {:?}",
                example,
                settings.err()
            );
        }
    }
}

/// Test color creation and conversion
#[test]
fn test_color_functionality() {
    use awtrix3::Color;

    // Test color creation
    let red = Color::new(255, 0, 0);
    let green = Color::new(0, 255, 0);
    let blue = Color::new(0, 0, 255);

    // Test hex conversion
    assert_eq!(red.to_hex(), "#FF0000");
    assert_eq!(green.to_hex(), "#00FF00");
    assert_eq!(blue.to_hex(), "#0000FF");

    // Test hex parsing
    assert!(Color::from_hex("#FF0000").is_ok());
    assert!(Color::from_hex("#GG0000").is_err());
}

/// Test notification builder comprehensive functionality
#[test]
fn test_notification_builder_comprehensive() {
    use awtrix3::Color;

    let notification = Notification::builder()
        .text("Test Notification")
        .icon(1234)
        .color(Color::RED)
        .duration(10)
        .progress(50)
        .hold(true)
        .wakeup(true)
        .build();

    assert_eq!(notification.text, Some("Test Notification".to_string()));
    assert_eq!(notification.icon, Some(1234));
    assert_eq!(notification.duration, Some(10));
    assert_eq!(notification.progress, Some(50));
    assert_eq!(notification.hold, Some(true));
    assert_eq!(notification.wakeup, Some(true));
}

/// Test client creation
#[test]
fn test_client_creation() {
    use awtrix3::Client;

    // Test client creation with valid host
    let client = Client::new("192.168.1.100");
    assert!(client.is_ok());

    // Test client creation with hostname
    let client = Client::new("awtrix.local");
    assert!(client.is_ok());

    // The client should be created successfully even if host is unreachable
    // since we don't test connectivity during creation
}
