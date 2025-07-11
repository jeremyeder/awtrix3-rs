use serde::{Deserialize, Serialize, Serializer, Deserializer};
use std::fmt;
use crate::error::{AwtrixError, Result};

/// Represents a color in RGB format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    /// Create a new color from RGB values
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
    
    /// Create a color from a hex string (e.g., "#FF0000" or "FF0000")
    pub fn from_hex(hex: &str) -> Result<Self> {
        let hex = hex.trim_start_matches('#');
        
        if hex.len() != 6 {
            return Err(AwtrixError::InvalidColor(format!(
                "Hex color must be 6 characters, got: {}", hex
            )));
        }
        
        let r = u8::from_str_radix(&hex[0..2], 16)
            .map_err(|_| AwtrixError::InvalidColor(format!("Invalid hex color: {}", hex)))?;
        let g = u8::from_str_radix(&hex[2..4], 16)
            .map_err(|_| AwtrixError::InvalidColor(format!("Invalid hex color: {}", hex)))?;
        let b = u8::from_str_radix(&hex[4..6], 16)
            .map_err(|_| AwtrixError::InvalidColor(format!("Invalid hex color: {}", hex)))?;
        
        Ok(Self { r, g, b })
    }
    
    /// Convert to hex string
    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
    
    /// Convert to RGB array
    pub fn to_rgb_array(&self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

impl From<[u8; 3]> for Color {
    fn from(rgb: [u8; 3]) -> Self {
        Self::new(rgb[0], rgb[1], rgb[2])
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self::new(r, g, b)
    }
}

impl From<&str> for Color {
    fn from(s: &str) -> Self {
        Self::from_hex(s).unwrap_or(Self::new(255, 255, 255))
    }
}

impl From<String> for Color {
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}

// Custom serialization to support both array and hex formats
impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize as RGB array for AWTRIX3 API
        self.to_rgb_array().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum ColorFormat {
            Array([u8; 3]),
            Hex(String),
        }
        
        match ColorFormat::deserialize(deserializer)? {
            ColorFormat::Array(rgb) => Ok(Color::from(rgb)),
            ColorFormat::Hex(hex) => Color::from_hex(&hex)
                .map_err(|e| serde::de::Error::custom(e.to_string())),
        }
    }
}

// Predefined colors
impl Color {
    pub const WHITE: Self = Self { r: 255, g: 255, b: 255 };
    pub const BLACK: Self = Self { r: 0, g: 0, b: 0 };
    pub const RED: Self = Self { r: 255, g: 0, b: 0 };
    pub const GREEN: Self = Self { r: 0, g: 255, b: 0 };
    pub const BLUE: Self = Self { r: 0, g: 0, b: 255 };
    pub const YELLOW: Self = Self { r: 255, g: 255, b: 0 };
    pub const CYAN: Self = Self { r: 0, g: 255, b: 255 };
    pub const MAGENTA: Self = Self { r: 255, g: 0, b: 255 };
    pub const ORANGE: Self = Self { r: 255, g: 165, b: 0 };
    pub const PURPLE: Self = Self { r: 128, g: 0, b: 128 };
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_color_from_hex() {
        let color = Color::from_hex("#FF0000").unwrap();
        assert_eq!(color, Color::RED);
        
        let color = Color::from_hex("00FF00").unwrap();
        assert_eq!(color, Color::GREEN);
    }
    
    #[test]
    fn test_color_to_hex() {
        assert_eq!(Color::RED.to_hex(), "#FF0000");
        assert_eq!(Color::GREEN.to_hex(), "#00FF00");
        assert_eq!(Color::BLUE.to_hex(), "#0000FF");
    }
    
    #[test]
    fn test_color_serialization() {
        let color = Color::RED;
        let json = serde_json::to_string(&color).unwrap();
        assert_eq!(json, "[255,0,0]");
    }
    
    #[test]
    fn test_color_deserialization() {
        let color: Color = serde_json::from_str("[255,0,0]").unwrap();
        assert_eq!(color, Color::RED);
        
        let color: Color = serde_json::from_str("\"#00FF00\"").unwrap();
        assert_eq!(color, Color::GREEN);
    }
}