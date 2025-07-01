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
