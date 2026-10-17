use crate::cli::*;
// use crate::utils::parse_color;
use anyhow::Result;

pub async fn execute(client: awtrix3::Client, args: IndicatorArgs) -> Result<()> {
    // Parse which indicators to control
    let indicators = if args.indicator == "all" {
        vec![1, 2, 3]
    } else {
        // Parse single indicator number
        let num: u8 = args
            .indicator
            .parse()
            .map_err(|_| anyhow::anyhow!("Invalid indicator number. Use 1-3 or 'all'"))?;

        if !(1..=3).contains(&num) {
            return Err(anyhow::anyhow!("Indicator number must be 1-3"));
        }

        vec![num]
    };

    // Parse color if provided (None if turning off)
    let color = if args.off {
        None
    } else if let Some(color_str) = args.color {
        Some(awtrix3::Color::from_hex(&color_str)?)
    } else {
        return Err(anyhow::anyhow!("Must specify either --color or --off"));
    };

    // Apply to each indicator
    for indicator in indicators {
        client.set_indicator(indicator, color).await?;

        if args.off {
            println!("Indicator {indicator} turned OFF");
        } else if let Some(c) = &color {
            let color_hex = c.to_hex();
            println!("Indicator {indicator} set to color {color_hex}");
        }
    }

    Ok(())
}
