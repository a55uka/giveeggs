# Product Monitor for Shopify

A monitoring tool that tracks product (egg) changes on Shopify instances and sends notifications through ntfy

## Features

- Monitors product details on Shopify instances
- Detects various types of changes:
    - Price changes
    - Product availability
    - New variants (of products)
    - Title changes
    - Description updates
- Sends real-time notifications via ntfy
- Configurable monitoring intervals

## Installation

### Building from Source

1. Clone the repository:

```bash
git clone https://github.com/yourusername/giveeggs.git
cd giveeggs
```

2. Build the application:

```bash
cargo build --release
```

The compiled binary will be available at `target/release/giveeggs`.

## Usage

### Configuration

Currently, configuration is done through the source code in `main.rs`.

(i swear ill move config to dotenv)

- The Shopify store URL
- Product IDs to monitor
- Notification topic
- Check intervals

Example configuration in `main.rs`:

```rust
let mut monitor = ProductMonitor::new(
    "https://eggdot.net/",            // Shopify store URL
    "https://ntfy.sh",                // ntfy service URL
    "shopify_eggdot-net",             // ntfy topic
    vec![8314152616094]               // Product IDs to monitor
).await?;

monitor.initialize().await?; // DO ERROR CHECKING GRR

monitor.check_for_changes().await?; // Loop thisss
```

### Notifications

Notifications are sent to ntfy.sh and can be received through anything that can receive ntfy subscriptions, this includes but not limited to;
- ntfy.sh webapp
- ntfy mobile app

It is also possible to use a different ntfy server

## Architecture Overview

The application is structured into several modules:

- `models.rs` - Data structures for Shopify products
- `product_comparison.rs` - Logic for detecting changes between product states
- `notification.rs` - Handles formatting and sending notifications
- `client.rs` - API client for communicating with Shopify
- `lib.rs` - Core monitoring functionality
- `main.rs` - Application entry point and configuration

<sup><sup><em>:3 mew mrp<em/><sub/><sub/>