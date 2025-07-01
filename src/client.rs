use crate::{BoxError, models::{Product, Root}};
use reqwest::header::{COOKIE, USER_AGENT};
use reqwest::{Client, Url};
use std::collections::HashMap;
use tracing::error;

pub struct ShopifyClient {
    client: Client,
    base_url: Url,
}

impl ShopifyClient {
    pub fn new(base_url: Url) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }

    pub async fn fetch_products(
        &self,
        product_ids: &[u64],
    ) -> Result<HashMap<u64, Product>, BoxError> {
        let endpoint = self.base_url.join("/products.json")?;

        let response = self
            .client
            .get(endpoint)
            .header(USER_AGENT, "Meow Meow I'm a cat (assuka_)")
            .header(COOKIE, "localization=GB;cart_currency=GBP;")
            .send()
            .await?;

        if !response.status().is_success() {
            error!("Request failed: {:?}", response.status());
            return Err(format!("Request failed with status: {}", response.status()).into());
        }

        let root: Root = response.json().await?;

        let mut products_map = HashMap::with_capacity(product_ids.len());
        for id in product_ids {
            if let Some(product) = find_product_by_id(&root.products, *id) {
                products_map.insert(*id, product.clone());
            }
        }

        Ok(products_map)
    }
}

fn find_product_by_id(products: &[Product], target_id: u64) -> Option<&Product> {
    products.iter().find(|product| product.id == target_id)
}
