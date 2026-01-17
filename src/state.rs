use crate::products_config::CfgProducts;
use crate::stripe_retypes;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppState {
    pub stripe_data: Option<StripeData>,
    pub products_config: Option<CfgProducts>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StripeData {
    pub products: Vec<stripe_retypes::DbProduct>,
    pub customers: Vec<stripe_retypes::DbCustomer>,
    pub checkout_sessions: Vec<stripe_retypes::DbCheckoutSession>,
    pub default_shipping_rate_id: String,
    pub free_shipping_rate_id: String,
}
