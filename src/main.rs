use std::error::Error;
use std::time::Duration;
use tokio::signal;
use tracing::{error, info};

use giveeggs::ProductMonitor;

const CHECK_INTERVAL: Duration = Duration::from_secs(50);
const ERROR_RETRY_INTERVAL: Duration = Duration::from_secs(20);

type BoxError = Box<dyn Error + Send + Sync>;

#[tokio::main]
async fn main() -> Result<(), BoxError> {
    tracing_subscriber::fmt::init();

    info!("Starting product monitor...");

    let worker = tokio::spawn(async {
        let mut monitor = match ProductMonitor::new(
            "https://eggdot.net/",
            "https://ntfy.sh",
            "shopify_eggdot-net",
            vec![8314152616094],
        ) {
            Ok(monitor) => monitor,
            Err(e) => {
                error!("Failed to create product monitor: {}", e);
                return 1;
            }
        };

        if let Err(e) = monitor.initialize().await {
            error!("Failed to initialize product monitor: {}", e);
            return 1;
        }

        loop {
            info!("Checking for product changes...");

            match monitor.check_for_changes().await {
                Ok(_) => {
                    info!(
                        "Check completed. Waiting {} seconds until next check...",
                        CHECK_INTERVAL.as_secs()
                    );
                    tokio::time::sleep(CHECK_INTERVAL).await;
                }
                Err(e) => {
                    error!("Error checking for changes: {}", e);
                    info!("Retrying in {} seconds...", ERROR_RETRY_INTERVAL.as_secs());
                    tokio::time::sleep(ERROR_RETRY_INTERVAL).await;
                }
            }
        }
    });

    tokio::select! {
        _ = worker => {
            info!("Worker task completed.");
        },
        _ = signal::ctrl_c() => {
            info!("Shutdown signal received. Cleaning up...");
        }
    }

    info!("Product monitor stopped gracefully.");

    Ok(())
}
