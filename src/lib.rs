pub mod client;
pub mod models;
pub mod notification;
pub mod product_comparison;

use ntfy::{
    Dispatcher,
    dispatcher::{self, Async},
};
use reqwest::{IntoUrl, Url};
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use tracing::{info, warn};

use crate::{client::ShopifyClient, models::Product, notification::NotificationBuilder};

type BoxError = Box<dyn Error + Send + Sync>;

pub struct ProductMonitor {
    client: ShopifyClient,
    dispatcher: Arc<Dispatcher<Async>>,
    ntfy_topic: String,
    previous_products: HashMap<u64, Product>,
    base_url: Url,
    product_ids: Vec<u64>,
}

impl ProductMonitor {
    pub fn new<U: IntoUrl>(
        base_url: U,
        ntfy_url: U,
        ntfy_topic: impl Into<String>,
        product_ids: Vec<u64>,
    ) -> Result<Self, BoxError> {
        let base_url = base_url.into_url()?;
        let ntfy_topic = ntfy_topic.into();

        let client = ShopifyClient::new(base_url.clone());
        let dispatcher = Arc::new(dispatcher::builder(ntfy_url.as_str()).build_async()?);

        let previous_products = HashMap::new();

        Ok(Self {
            client,
            dispatcher,
            ntfy_topic,
            previous_products,
            base_url,
            product_ids,
        })
    }

    pub async fn initialize(&mut self) -> Result<(), BoxError> {
        info!("Fetching initial product state...");
        self.previous_products = self.client.fetch_products(&self.product_ids).await?;
        // self.previous_products = HashMap::new(); // Testing notifs
        // self.previous_products.insert(8314152616094, Product {
        //     id: 8314152616094,
        //     title: "".to_string(),
        //     handle: "".to_string(),
        //     body_html: "".to_string(),
        //     published_at: "".to_string(),
        //     created_at: "".to_string(),
        //     updated_at: "".to_string(),
        //     vendor: "".to_string(),
        //     product_type: "".to_string(),
        //     tags: vec![],
        //     variants: vec![],
        //     images: vec![],
        //     options: vec![],
        // });
        info!(
            "Initial state loaded for {} products",
            self.previous_products.len()
        );
        Ok(())
    }

    pub async fn check_for_changes(&mut self) -> Result<(), BoxError> {
        let current_products = self.client.fetch_products(&self.product_ids).await?;

        for (id, current_product) in &current_products {
            if let Some(previous_product) = self.previous_products.get(id) {
                let changes = previous_product.compare_with(current_product);

                for change in &changes {
                    self.send_notification(change, current_product).await?;
                }
            } else {
                info!("New product detected: {}", id);
            }
        }

        self.previous_products = current_products;

        Ok(())
    }

    async fn send_notification(
        &self,
        change: &product_comparison::ProductChange,
        product: &Product,
    ) -> Result<(), BoxError> {
        let (notification, product_url) =
            NotificationBuilder::build_from_change(change, product, &self.base_url)?;

        warn!("Sending notification: {:#?}", notification);

        let payload = NotificationBuilder::build_payload(notification, &self.ntfy_topic, product_url);

        let dispatcher = Arc::clone(&self.dispatcher);
        dispatcher.send(&payload).await?;
        Ok(())
    }
}
