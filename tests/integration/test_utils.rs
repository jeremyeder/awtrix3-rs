use awtrix3::utils::parse_color;
use awtrix3::models::color::Color;

/// Test comprehensive color parsing scenarios
#[test]
fn test_color_parsing_comprehensive() {
    // Test hex colors
    assert!(parse_color("#FF0000").is_ok());
    assert!(parse_color("#00FF00").is_ok());
    assert!(parse_color("#0000FF").is_ok());
    assert!(parse_color("FF0000").is_ok()); // Without #
    
    // Test RGB colors
    assert!(parse_color("255,0,0").is_ok());
    assert!(parse_color("0,255,0").is_ok());
    assert!(parse_color("0, 0, 255").is_ok()); // With spaces
    
    // Test named colors
    assert!(parse_color("red").is_ok());
    assert!(parse_color("RED").is_ok());
    assert!(parse_color("Green").is_ok());
    assert!(parse_color("blue").is_ok());
    assert!(parse_color("white").is_ok());
    assert!(parse_color("black").is_ok());
    
    // Test invalid colors
    assert!(parse_color("invalid").is_err());
    assert!(parse_color("").is_err());
    assert!(parse_color("#GG0000").is_err()); // Invalid hex
    assert!(parse_color("256,0,0").is_err()); // RGB out of range
    assert!(parse_color("255,0").is_err()); // Incomplete RGB
}

/// Test color conversion round-trip
#[test]
fn test_color_conversion_roundtrip() {
    let test_colors = [
        ("#FF0000", Color::RED),
        ("#00FF00", Color::GREEN),
        ("#0000FF", Color::BLUE),
        ("#FFFFFF", Color::WHITE),
        ("#000000", Color::BLACK),
    ];
    
    for (hex, expected) in &test_colors {
        // Parse from hex
        let parsed = parse_color(hex).expect("Should parse valid hex color");
        assert_eq!(parsed, *expected);
        
        // Convert back to hex
        let back_to_hex = parsed.to_hex();
        assert_eq!(back_to_hex, *hex);
    }
}

/// Test RGB parsing with edge cases
#[test]
fn test_rgb_parsing_edge_cases() {
    // Valid edge cases
    assert!(parse_color("0,0,0").is_ok()); // Black
    assert!(parse_color("255,255,255").is_ok()); // White
    assert!(parse_color(" 128 , 128 , 128 ").is_ok()); // Gray with extra spaces
    
    // Invalid edge cases
    assert!(parse_color("256,0,0").is_err()); // R > 255
    assert!(parse_color("0,256,0").is_err()); // G > 255
    assert!(parse_color("0,0,256").is_err()); // B > 255
    assert!(parse_color("-1,0,0").is_err()); // Negative values
    assert!(parse_color("0,0").is_err()); // Missing component
    assert!(parse_color("0,0,0,0").is_err()); // Too many components
}

/// Test named color coverage
#[test]
fn test_named_color_coverage() {
    let named_colors = [
        "red", "green", "blue", "white", "black",
        "yellow", "cyan", "magenta", "orange", "purple"
    ];
    
    for color_name in &named_colors {
        let result = parse_color(color_name);
        assert!(
            result.is_ok(),
            "Named color '{}' should be supported",
            color_name
        );
        
        // Test case insensitivity
        let uppercase_result = parse_color(&color_name.to_uppercase());
        assert!(
            uppercase_result.is_ok(),
            "Named color '{}' should be case insensitive",
            color_name
        );
        
        // Results should be the same regardless of case
        assert_eq!(result.unwrap(), uppercase_result.unwrap());
    }
}

/// Test hex color validation
#[test]
fn test_hex_color_validation() {
    // Valid hex formats
    let valid_hex = [
        "#000000", "#FFFFFF", "#FF0000", "#00FF00", "#0000FF",
        "000000", "FFFFFF", "ff0000", "00ff00", "0000ff" // Without #
    ];
    
    for hex in &valid_hex {
        assert!(
            parse_color(hex).is_ok(),
            "Hex color '{}' should be valid",
            hex
        );
    }
    
    // Invalid hex formats
    let invalid_hex = [
        "#", "#12345", "#1234567", "#GGGGGG", "GGGGGG",
        "#12", "#1234", "invalid", "#xyz123"
    ];
    
    for hex in &invalid_hex {
        assert!(
            parse_color(hex).is_err(),
            "Hex color '{}' should be invalid",
            hex
        );
    }
}