#![allow(unused)]
pub mod app;
pub mod error_template;
#[cfg(feature = "ssr")]
pub mod fileserv;
pub mod products_config;
pub mod stripe_retypes;

mod cart_state;
mod server;
mod state;
#[cfg(feature = "ssr")]
pub mod sync;

pub use cart_state::ShoppingCart;
pub use server::*;
pub use state::{AppState, StripeData};

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}

use app::PagerPropsBuilder_Error_Missing_required_field_page;
use leptos::{create_effect, Serializable, ServerFnError};
use leptos_router::FromFormData;
use products_config::{CfgProduct, CfgProducts};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use log::*;
use stripe_retypes::{DbCheckoutSession, DbCheckoutSessionStatus, DbProduct};
