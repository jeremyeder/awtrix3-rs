use crate::cli::*;
use crate::utils::parse_color;
use anyhow::Result;

pub async fn execute(client: awtrix3::Client, args: IndicatorArgs) -> Result<()> {
    // Parse which indicators to control
    let indicators = if args.indicator == "all" {
        vec![1, 2, 3]
    } else {
        // Parse single indicator number
        let num: u8 = args.indicator.parse()
            .map_err(|_| anyhow::anyhow!("Invalid indicator number. Use 1-3 or 'all'"))?;
        
        if num < 1 || num > 3 {
            return Err(anyhow::anyhow!("Indicator number must be 1-3"));
        }
        
        vec![num]
    };
    
    // Parse color if provided (None if turning off)
    let color = if args.off {
        None
    } else if let Some(color_str) = args.color {
        Some(parse_color(&color_str)?)
    } else {
        return Err(anyhow::anyhow!("Must specify either --color or --off"));
    };
    
    // Apply to each indicator
    for indicator in indicators {
        client.set_indicator(indicator, color.clone()).await?;
        
        if args.off {
            println!("Indicator {} turned OFF", indicator);
        } else if let Some(c) = &color {
            println!("Indicator {} set to color {}", indicator, c.to_hex());
        }
    }
    
    Ok(())
}
