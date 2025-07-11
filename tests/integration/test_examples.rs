use std::fs;
use awtrix3::{Notification, CustomApp, Settings};

/// Test that all example notification files are valid JSON and parse correctly
#[test]
fn test_notification_examples() {
    let examples_dir = "examples/notifications";
    
    // Ensure examples directory exists
    assert!(
        std::path::Path::new(examples_dir).exists(),
        "Examples directory should exist: {}", examples_dir
    );
    
    // Test each notification example
    let examples = ["weather.json", "download-progress.json", "alert.json"];
    
    for example in &examples {
        let path = format!("{}/{}", examples_dir, example);
        let content = fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("Failed to read example file: {}", path));
        
        let notification: Result<Notification, _> = serde_json::from_str(&content);
        assert!(
            notification.is_ok(),
            "Example notification should parse correctly: {} - Error: {:?}",
            example,
            notification.err()
        );
        
        // Verify the notification has at least text
        let notification = notification.unwrap();
        assert!(
            notification.text.is_some(),
            "Notification example should have text: {}", example
        );
    }
}

/// Test that all custom app examples are valid JSON and parse correctly
#[test]
fn test_custom_app_examples() {
    let examples_dir = "examples/custom-apps";
    
    let examples = ["stock-ticker.json", "weather-app.json"];
    
    for example in &examples {
        let path = format!("{}/{}", examples_dir, example);
        let content = fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("Failed to read custom app example: {}", path));
        
        let app: Result<CustomApp, _> = serde_json::from_str(&content);
        assert!(
            app.is_ok(),
            "Custom app example should parse correctly: {} - Error: {:?}",
            example,
            app.err()
        );
        
        // Verify the app has text
        let app = app.unwrap();
        assert!(
            app.text.is_some(),
            "Custom app example should have text: {}", example
        );
    }
}

/// Test that all settings examples are valid JSON and parse correctly
#[test]
fn test_settings_examples() {
    let examples_dir = "examples/settings";
    
    let examples = ["gaming-setup.json", "bedroom-mode.json"];
    
    for example in &examples {
        let path = format!("{}/{}", examples_dir, example);
        let content = fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("Failed to read settings example: {}", path));
        
        let settings: Result<Settings, _> = serde_json::from_str(&content);
        assert!(
            settings.is_ok(),
            "Settings example should parse correctly: {} - Error: {:?}",
            example,
            settings.err()
        );
        
        // Verify the settings have at least brightness
        let settings = settings.unwrap();
        assert!(
            settings.brightness.is_some(),
            "Settings example should have brightness: {}", example
        );
    }
}

/// Test that examples can be serialized back to JSON
#[test]
fn test_example_serialization_roundtrip() {
    // Test notification serialization
    let notification_content = fs::read_to_string("examples/notifications/weather.json")
        .expect("Weather notification example should exist");
    
    let notification: Notification = serde_json::from_str(&notification_content)
        .expect("Weather notification should parse");
    
    let serialized = serde_json::to_string_pretty(&notification)
        .expect("Notification should serialize");
    
    let reparsed: Notification = serde_json::from_str(&serialized)
        .expect("Serialized notification should parse");
    
    // Verify key fields are preserved
    assert_eq!(notification.text, reparsed.text);
    assert_eq!(notification.icon, reparsed.icon);
    assert_eq!(notification.duration, reparsed.duration);
}

/// Test color parsing in examples
#[test]
fn test_example_color_parsing() {
    use awtrix3::models::color::Color;
    
    // Test that color hex strings in examples are valid
    let weather_content = fs::read_to_string("examples/notifications/weather.json")
        .expect("Weather notification example should exist");
    
    let notification: Notification = serde_json::from_str(&weather_content)
        .expect("Weather notification should parse");
    
    if let Some(color) = notification.color {
        // Verify color can be converted to hex and back
        let hex = color.to_hex();
        assert!(hex.starts_with('#'));
        assert_eq!(hex.len(), 7); // #RRGGBB format
        
        // Verify color can be parsed from hex
        let parsed_color = Color::from_hex(&hex);
        assert!(parsed_color.is_ok());
    }
}