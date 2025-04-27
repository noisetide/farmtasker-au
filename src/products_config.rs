use crate::{fetch_local_product_info, stripe_retypes};
use serde::*;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CfgProducts(pub Vec<CfgProduct>);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CfgProduct {
    pub stripe_id: String,
    pub item_number: Option<i64>,
    pub name: String,
    pub description: Option<String>,
    pub price: Option<stripe_retypes::DbPrice>,
    pub images: Option<Vec<String>>, // urls of images from stripe (usually just one is available)
    pub local_images: Option<Vec<PathBuf>>, // paths to local image files
    pub metadata: Option<HashMap<String, String>>,
}
