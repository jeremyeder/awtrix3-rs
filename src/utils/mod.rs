pub mod discovery;

use anyhow::Result;

/// Parse a color from various formats - temporarily disabled due to module visibility issues
#[allow(dead_code)]
pub fn parse_color_placeholder(input: &str) -> Result<String> {
    // Simplified version without Color dependency
    Ok(input.to_string())
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
    fn test_format_output() {
        let data = serde_json::json!({"test": "value"});
        let result = format_output(&data, true).unwrap();
        assert!(result.contains("test"));
    }
}
