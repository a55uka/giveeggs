use std::error::Error;
use std::time::Duration;
use tracing::info;
use giveeggs::Eggs;

const WAIT: Duration = Duration::from_secs(50);

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    info!("Starting checker...");

    let mut eggs = Eggs::new("https://eggdot.net/", ("https://ntfy.sh", "shopify_eggdot-net"), vec![8314152616094]).await?;

    loop {
        info!("Checking for new giveeggs...");
        eggs.notify_changes().await?;
        info!("Waiting {} seconds...", WAIT.as_secs());
        tokio::time::sleep(WAIT).await;
    }
    Ok(())
}
