use crate::{models::Product, product_comparison::ProductChange, BoxError};
use ntfy::{Payload, Priority};
use reqwest::Url;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Notification {
    pub title: String,
    pub message: String,
    pub priority: Priority,
    pub tags: Vec<String>,
}

pub struct NotificationBuilder;

impl NotificationBuilder {
    pub fn build_from_change(
        change: &ProductChange,
        product: &Product,
        base_url: &Url,
    ) -> Result<(Notification, Url), BoxError> {
        let notification = match change {
            ProductChange::VariantPriceChanged {
                variant_id,
                old_price,
                new_price,
            } => Notification {
                title: format!("Price Update for Variant {}", variant_id),
                message: format!("Price changed from {} GBP to {} GBP", old_price, new_price),
                priority: Priority::High,
                tags: vec!["price".to_string(), "loudspeaker".to_string()],
            },
            ProductChange::VariantAvailabilityChanged {
                variant_id,
                old_available,
                new_available,
            } => Notification {
                title: format!("Variant {} Availability Update", variant_id),
                message: format!(
                    "Availability changed from {} to {}",
                    if *old_available {
                        "In Stock"
                    } else {
                        "Out of Stock"
                    },
                    if *new_available {
                        "In Stock"
                    } else {
                        "Out of Stock"
                    }
                ),
                priority: if *new_available {
                    Priority::Max
                } else {
                    Priority::Low
                },
                tags: vec!["availability".to_string(), "rotating_light".to_string()],
            },
            ProductChange::NewVariantAdded(variant) => Notification {
                title: format!("New Variant Added: {}", variant.title),
                message: format!("New variant added with price: {} GBP", variant.price),
                priority: Priority::Default,
                tags: vec!["variant".to_string(), "new".to_string()],
            },
            ProductChange::TitleChanged { old, new } => Notification {
                title: "Product Title Changed".to_string(),
                message: format!("Changed from '{}' to '{}'", old, new),
                priority: Priority::Default,
                tags: vec!["title".to_string(), "womans_hat".to_string()],
            },
            ProductChange::DescriptionChanged { old, new } => Notification {
                title: "Product Description Changed".to_string(),
                message: format!("Changed from:\n{}\nto:\n{}", html2md::rewrite_html(old, false), html2md::rewrite_html(new, false)),
                priority: Priority::Low,
                tags: vec!["description".to_string(), "womans_clothes".to_string()],
            },
        };

        let product_url = base_url.join("/products")?.join(&product.handle)?;
        Ok((notification, product_url))
    }

    pub fn build_payload(notification: Notification, topic: &str, product_url: Url) -> Payload {
        Payload::new(topic)
            .title(notification.title)
            .message(notification.message)
            .tags(notification.tags)
            .priority(notification.priority)
            .click(product_url)
            .markdown(true)
    }
}
