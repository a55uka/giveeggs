use std::collections::HashMap;
use std::error::Error;
use ntfy::Priority;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub products: Vec<Product>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    pub id: u64,
    pub title: String,
    pub handle: String,
    #[serde(rename = "body_html")]
    pub body_html: String,
    #[serde(rename = "published_at")]
    pub published_at: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    pub vendor: String,
    #[serde(rename = "product_type")]
    pub product_type: String,
    pub tags: Vec<String>,
    pub variants: Vec<Variant>,
    pub images: Vec<Image>,
    pub options: Vec<ProductOptions>,
}

impl Product {
    pub async fn compare_nfty_with(&self, other: &Product) -> Result<Vec<ProductChange>, Box<dyn Error>> {
        let mut changes = Vec::new();

        if self.title != other.title {
            changes.push(ProductChange::TitleChanged {
                old: self.title.clone(),
                new: other.title.clone()
            });
        }

        if self.body_html != other.body_html {
            changes.push(ProductChange::DescriptionChanged {
                old: self.body_html.clone(),
                new: other.body_html.clone()
            });
        }

        let old_variants: HashMap<i64, &Variant> = self.variants.iter().map(|v| (v.id, v)).collect();
        let new_variants: HashMap<i64, &Variant> = other.variants.iter().map(|v| (v.id, v)).collect();

        for new_variant in &other.variants {
            if !old_variants.contains_key(&new_variant.id) {
                changes.push(ProductChange::NewVariantAdded(new_variant.clone()));
            }
        }

        for old_variant in &self.variants {
            if let Some(new_variant) = new_variants.get(&old_variant.id) {
                if old_variant.available != new_variant.available {
                    changes.push(ProductChange::VariantAvailabilityChanged {
                        variant_id: old_variant.id,
                        old_available: old_variant.available,
                        new_available: new_variant.available
                    });
                }

                if old_variant.price != new_variant.price {
                    changes.push(ProductChange::VariantPriceChanged {
                        variant_id: old_variant.id,
                        old_price: old_variant.price.clone(),
                        new_price: new_variant.price.clone()
                    });
                }
            }
        }

        Ok(changes)
    }
}

#[derive(Debug, Serialize)]
pub enum ProductChange {
    TitleChanged { old: String, new: String },
    DescriptionChanged { old: String, new: String },
    VariantPriceChanged { variant_id: i64, old_price: String, new_price: String },
    VariantAvailabilityChanged { variant_id: i64, old_available: bool, new_available: bool },
    NewVariantAdded(Variant),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Notification {
    pub title: String,
    pub message: String,
    pub priority: Priority,
    pub tags: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Variant {
    pub id: i64,
    pub title: String,
    pub option1: Option<String>,
    pub option2: Option<String>,
    pub option3: Option<String>,
    pub sku: String,
    #[serde(rename = "requires_shipping")]
    pub requires_shipping: bool,
    pub taxable: bool,
    #[serde(rename = "featured_image")]
    pub featured_image: Option<Image>,
    pub available: bool,
    pub price: String,
    pub grams: i64,
    #[serde(rename = "compare_at_price")]
    pub compare_at_price: Option<String>,
    pub position: i64,
    #[serde(rename = "product_id")]
    pub product_id: i64,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub id: i64,
    #[serde(rename = "created_at")]
    pub created_at: String,
    pub position: i64,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "product_id")]
    pub product_id: i64,
    #[serde(rename = "variant_ids")]
    pub variant_ids: Vec<i64>,
    pub src: String,
    pub width: i64,
    pub height: i64,
    pub alt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductOptions {
    pub name: String,
    pub position: i64,
    pub values: Vec<String>,
}
