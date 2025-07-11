use crate::cli::NotifyArgs;
use anyhow::Result;

pub async fn execute(client: awtrix3::Client, args: NotifyArgs) -> Result<()> {
    if args.dismiss {
        println!("Dismissing current notification");
        // TODO: client.dismiss_notification().await?;
    } else {
        println!("Sending notification: {}", args.text);
        // TODO: Build notification from args and send
        // let notification = build_notification_from_args(args)?;
        // client.notify(notification).await?;
    }
    
    Ok(())
}