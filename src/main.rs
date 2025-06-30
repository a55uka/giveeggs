use std::error::Error;
use std::time::Duration;
use tokio::signal;
use tracing::{error, info};

use giveeggs::Eggs;

const WAIT: Duration = Duration::from_secs(50);
const FAIL_WAIT: Duration = Duration::from_secs(20);

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    info!("Starting checker...");

    let worker = tokio::spawn(async {
        let mut eggs =
            match Eggs::new("https://eggdot.net/", ("https://ntfy.sh", "shopify_eggdot-net"), vec![8314152616094]).await {
                Ok(eggs) => eggs,
                Err(e) => {
                    error!("Failed to create Eggs instance: {}", e);
                    return 1;
                },
            };

        loop {
            info!("Checking for new giveeggs...");
            let ntfy = eggs.notify_changes().await.unwrap();
            info!("Waiting {} seconds...", WAIT.as_secs());
            tokio::time::sleep(WAIT).await;
        }
    });

    tokio::select! {
        _ = worker => {},
        _ = signal::ctrl_c() => {
            info!("Shutdown signal received. Cleaning up...");
        }
    }

    info!("Giveeggs stopped gracefully.");

    Ok(())
}