use crate::models::{Product, Variant};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub enum ProductChange {
    TitleChanged {
        old: String,
        new: String,
    },
    DescriptionChanged {
        old: String,
        new: String,
    },
    VariantPriceChanged {
        variant_id: i64,
        old_price: String,
        new_price: String,
    },
    VariantAvailabilityChanged {
        variant_id: i64,
        old_available: bool,
        new_available: bool,
    },
    NewVariantAdded(Variant),
}

impl Product {
    pub fn compare_with(&self, other: &Product) -> Vec<ProductChange> {
        let mut changes = Vec::new();

        self.compare_basic_attributes(other, &mut changes);
        self.compare_variants(other, &mut changes);

        changes
    }

    fn compare_basic_attributes(&self, other: &Product, changes: &mut Vec<ProductChange>) {
        if self.title != other.title {
            changes.push(ProductChange::TitleChanged {
                old: self.title.clone(),
                new: other.title.clone(),
            });
        }

        if self.body_html != other.body_html {
            changes.push(ProductChange::DescriptionChanged {
                old: self.body_html.clone(),
                new: other.body_html.clone(),
            });
        }
    }

    fn compare_variants(&self, other: &Product, changes: &mut Vec<ProductChange>) {
        let old_variants: HashMap<i64, &Variant> =
            self.variants.iter().map(|v| (v.id, v)).collect();
        let new_variants: HashMap<i64, &Variant> =
            other.variants.iter().map(|v| (v.id, v)).collect();

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
                        new_available: new_variant.available,
                    });
                }

                if old_variant.price != new_variant.price {
                    changes.push(ProductChange::VariantPriceChanged {
                        variant_id: old_variant.id,
                        old_price: old_variant.price.clone(),
                        new_price: new_variant.price.clone(),
                    });
                }
            }
        }
    }
}
