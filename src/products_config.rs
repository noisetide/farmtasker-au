use crate::stripe_retypes;
use serde::*;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CfgProduct {
    pub stripe_id: Option<String>,
    pub item_number: i64,
    pub name: String,
    pub description: String,
    pub price: stripe_retypes::DbPrice,
    pub stripe_images: Option<Vec<String>>, // urls of images
    pub local_images: Option<Vec<PathBuf>>, // paths to local image files
}
