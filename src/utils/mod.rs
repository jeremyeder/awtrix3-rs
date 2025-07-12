pub mod discovery;

use anyhow::Result;
use awtrix3::Color;

/// Parse a color from various formats
pub fn parse_color(input: &str) -> Result<Color> {
    if input.starts_with('#') || input.len() == 6 {
        // Hex format
        Color::from_hex(input).map_err(|e| anyhow::anyhow!("Invalid hex color: {}", e))
    } else if input.contains(',') {
        // RGB format like "255,0,0"
        let parts: Vec<&str> = input.split(',').collect();
        if parts.len() != 3 {
            return Err(anyhow::anyhow!("RGB format must be r,g,b"));
        }
        
        let r = parts[0].trim().parse::<u8>()
            .map_err(|_| anyhow::anyhow!("Invalid red value"))?;
        let g = parts[1].trim().parse::<u8>()
            .map_err(|_| anyhow::anyhow!("Invalid green value"))?;
        let b = parts[2].trim().parse::<u8>()
            .map_err(|_| anyhow::anyhow!("Invalid blue value"))?;
        
        Ok(Color::new(r, g, b))
    } else {
        // Try named colors
        match input.to_lowercase().as_str() {
            "red" => Ok(Color::RED),
            "green" => Ok(Color::GREEN),
            "blue" => Ok(Color::BLUE),
            "white" => Ok(Color::WHITE),
            "black" => Ok(Color::BLACK),
            "yellow" => Ok(Color::YELLOW),
            "cyan" => Ok(Color::CYAN),
            "magenta" => Ok(Color::MAGENTA),
            "orange" => Ok(Color::ORANGE),
            "purple" => Ok(Color::PURPLE),
            _ => Err(anyhow::anyhow!("Unknown color: {}", input)),
        }
    }
}

/// Format output based on CLI preferences
pub fn format_output<T: serde::Serialize>(data: &T, json: bool) -> Result<String> {
    if json {
        Ok(serde_json::to_string_pretty(data)?)
    } else {
        // For non-JSON output, serialize to JSON and format it nicely
        let json_value: serde_json::Value = serde_json::to_value(data)?;
        Ok(format!("{:#}", json_value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_color_hex() {
        let color = parse_color("#FF0000").unwrap();
        assert_eq!(color, Color::RED);
        
        let color = parse_color("00FF00").unwrap();
        assert_eq!(color, Color::GREEN);
    }
    
    #[test]
    fn test_parse_color_rgb() {
        let color = parse_color("255,0,0").unwrap();
        assert_eq!(color, Color::RED);
        
        let color = parse_color("0, 255, 0").unwrap();
        assert_eq!(color, Color::GREEN);
    }
    
    #[test]
    fn test_parse_color_named() {
        let color = parse_color("red").unwrap();
        assert_eq!(color, Color::RED);
        
        let color = parse_color("GREEN").unwrap();
        assert_eq!(color, Color::GREEN);
    }
}