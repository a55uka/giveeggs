pub mod json;

use std::collections::HashMap;
use std::error::Error;
use ntfy::{dispatcher::{self, Async}, Dispatcher, Payload, Priority};
use reqwest::{IntoUrl, Url};
use reqwest::header::{ACCEPT, COOKIE, USER_AGENT};
use tracing::{error, warn};

use crate::json::{Notification, ProductChange};

pub struct Eggs<'a> {
    client: reqwest::Client,
    dispatcher: Dispatcher<Async>,
    ntfy_id: &'a str,
    previous_products: HashMap<u64, json::Product>,
    pub url: Url,
    pub product_ids: Vec<u64>,
}

impl<'a> Eggs<'a> {
    pub async fn new<U: IntoUrl>(url: U, dispatcher_tuple: (U, &'a str), product_ids: Vec<u64>) -> Result<Self, Box<dyn Error>> {
        let url = url.into_url()?;
        let client = reqwest::Client::new();
        let dispatcher = dispatcher::builder(dispatcher_tuple.0.as_str()).build_async()?;
        // let previous_products = Self::fetch_products_helper(&client, &url, &product_ids).await?;
        let mut previous_products: HashMap<u64, json::Product> = Default::default();
        previous_products.insert(8314152616094, Default::default());

        Ok(Self {
            client,
            dispatcher,
            ntfy_id: dispatcher_tuple.1,
            previous_products,
            url,
            product_ids
        })
    }

    pub async fn notify_changes(&mut self) -> Result<(), Box<dyn Error>> {
        let products = self.fetch_products().await?;

        for (id, product) in &products {
            let previous_product = match self.previous_products.get(id) {
                Some(prod) => prod,
                None => break,
            };
            let changes = previous_product.compare_nfty_with(product).await?;

            for change in changes {
                let notification = match change {
                    ProductChange::VariantPriceChanged { variant_id, old_price, new_price } => Notification {
                        title: format!("Price Update for Variant {}", variant_id),
                        message: format!("Price changed from {} GBP to {} GBP", old_price, new_price),
                        priority: Priority::High,
                        tags: vec!["price".to_string(), "rotating_light".to_string()],
                    },
                    ProductChange::VariantAvailabilityChanged { variant_id, old_available, new_available } => Notification {
                        title: format!("Variant {} Availability Update", variant_id),
                        message: format!("Availability changed from {} to {}",
                                         if old_available { "In Stock" } else { "Out of Stock" },
                                         if new_available { "In Stock" } else { "Out of Stock" }),
                        priority: if new_available { Priority::Max } else { Priority::Low },
                        tags: vec!["availability".to_string(), "rotating_light".to_string()],
                    },
                    ProductChange::NewVariantAdded(ref variant) => Notification {
                        title: format!("New Variant Added: {}", variant.title),
                        message: format!("New variant added with price: {} GBP", variant.price),
                        priority: Priority::Default,
                        tags: vec!["variant".to_string(), "new".to_string()],
                    },
                    ProductChange::TitleChanged { old, new } => Notification {
                        title: "Product Title Changed".to_string(),
                        message: format!("Changed from '{}' to '{}'", old, new),
                        priority: Priority::Default,
                        tags: vec!["title".to_string(), "update".to_string()],
                    },
                    ProductChange::DescriptionChanged { old, new } => Notification {
                        title: "Product Description Changed".to_string(),
                        message: format!("Changed from:\n{}\nto:\n{}", old, new),
                        priority: Priority::Low,
                        tags: vec!["description".to_string(), "update".to_string()],
                    }
                };

                warn!("Notification sent: {:#?}", notification);
                
                let payload = Payload::new(self.ntfy_id)
                    .title(notification.title)
                    .message(notification.message)
                    .tags(notification.tags)
                    .priority(notification.priority)
                    .click(self.url.join("/products")?.join(&product.handle)?)
                    .markdown(true);

                self.dispatcher.send(&payload).await?;
            }
        }

        Ok(())
    }

    async fn fetch_products(&self) -> Result<HashMap<u64, json::Product>, Box<dyn Error>> {
        Self::fetch_products_helper(&self.client, &self.url, &self.product_ids).await
    }

    async fn fetch_products_helper(
        client: &reqwest::Client,
        url: &Url,
        product_ids: &[u64]
    ) -> Result<HashMap<u64, json::Product>, Box<dyn Error>> {
        let req = client.get(url.join("/products.json")?)
            .header(USER_AGENT, "Meow Meow I'm a cat (assuka_)")
            .header(COOKIE, "localization=GB;cart_currency=GBP;")
            .send().await?;
        
        if !req.status().is_success() {
            error!("Request failed: {:?}", req.status());
        }

        let json: json::Root = req.json().await?;

        let mut map: HashMap<u64, json::Product> = HashMap::with_capacity(product_ids.len());

        for id in product_ids {
            if let Some(product) = find_by_id(&json.products, *id) {
                map.insert(*id, product.clone());
            }
        }

        Ok(map)
    }
}

fn find_by_id(items: &[json::Product], target_id: u64) -> Option<&json::Product> {
    items.iter().find(|item| item.id == target_id)
}