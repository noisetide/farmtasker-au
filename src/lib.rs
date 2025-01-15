#![allow(unused)]
pub mod app;
pub mod error_template;
#[cfg(feature = "ssr")]
pub mod fileserv;
pub mod products_config;
pub mod stripe_retypes;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}
#[leptos::server(
    name = GetStripeKey
)]
pub async fn get_stripe_key() -> Result<String, leptos::ServerFnError> {
    unimplemented!();
}

#[leptos::server(
      name = StripeStater,
)]
pub async fn stripe_stater() -> Result<StripeData, leptos::ServerFnError> {
    let state = match leptos::use_context::<Option<crate::AppState>>() {
        Some(ok) => {
            // leptos::logging::log!("GOT context AppState");
            ok
        }
        None => {
            // leptos::logging::log!("No context AppState");
            None
        }
    };
    let axum::extract::State(appstate): axum::extract::State<crate::AppState> =
        leptos_axum::extract_with_state(match &state {
            Some(x) => x,
            None => &AppState {
                stripe_data: None,
                products_config: None,
            },
        })
        .await?;

    // log::info!("Server data: {:#?}", appstate.stripe_data.clone());
    match appstate.stripe_data {
        Some(ok) => {
            // info!("StripeData Loaded...");
            Ok(ok)
        }
        None => {
            // error!("No StripeData!");
            return Err(leptos::ServerFnError::ServerError(
                "StripeData not found".into(),
            ));
        }
    }
}

#[leptos::server(
      name = AppStateStater,
)]
pub async fn appstate_stater() -> Result<AppState, leptos::ServerFnError> {
    let state = match leptos::use_context::<Option<crate::AppState>>() {
        Some(ok) => {
            // leptos::logging::log!("GOT context AppState");
            ok
        }
        None => {
            // leptos::logging::log!("No context AppState");
            None
        }
    };

    let axum::extract::State(appstate): axum::extract::State<crate::AppState> =
        leptos_axum::extract_with_state(match &state {
            Some(x) => x,
            None => &AppState {
                stripe_data: None,
                products_config: None,
            },
        })
        .await?;

    Ok(appstate)
}

use app::PagerPropsBuilder_Error_Missing_required_field_page;
use leptos::{create_effect, Serializable, ServerFnError};
use leptos_router::FromFormData;
use products_config::{CfgProduct, CfgProducts};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppState {
    pub stripe_data: Option<StripeData>,
    pub products_config: Option<CfgProducts>,
}

use std::collections::HashMap;
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct ShoppingCart(HashMap<String, u8>);

impl ShoppingCart {
    pub fn add_single_product(&mut self, product_id: &String, add_limit: u8) {
        // If the product is already in the cart
        if let Some(quantity) = self.0.get_mut(product_id) {
            // Ensure the quantity doesn't exceed 20
            if *quantity < add_limit {
                *quantity += 1;
            }
        } else {
            // If the product is not in the cart, add it with a quantity of 1
            self.0.insert(product_id.clone(), 1);
        }
    }
    pub fn remove_single_product(&mut self, product_id: &String) {
        // If the product is in the cart, adjust its quantity
        if let Some(quantity) = self.0.get_mut(product_id) {
            if *quantity > 1 {
                *quantity -= 1; // Decrease quantity by 1
            } else {
                self.0.remove(&product_id.clone()); // If quantity is 1, remove the product
            }
        }
    }
    pub fn total_quantity(self) -> u64 {
        self.0.values().map(|&v| v as u64).sum()
    }

    pub fn calculate_total_price(&self, stripe_data: &[DbProduct]) -> i64 {
        let mut total_price: i64 = 0;

        // Iterate over the shopping cart (product_id, quantity)
        for (product_id, &quantity) in &self.0 {
            // Find the corresponding product in stripe_data
            if let Some(product) = stripe_data.iter().find(|p| p.id == *product_id) {
                // Check if the product has a default price and if it's active
                if let Some(price) = &product.default_price {
                    if price.active {
                        // Get the unit_amount from the price, default to 0 if it's not present
                        if let Some(unit_amount) = price.unit_amount {
                            // Multiply the unit price by the quantity and add it to the total
                            total_price += unit_amount * quantity as i64;
                        }
                    }
                }
            }
        }

        total_price
    }

    pub fn delete_product(&mut self, product_id: String) {
        self.0.remove(&product_id);
    }
}

impl From<Vec<stripe_retypes::DbCheckoutSessionItem>> for ShoppingCart {
    fn from(value: Vec<stripe_retypes::DbCheckoutSessionItem>) -> Self {
        let mut cart = ShoppingCart::default();
        for item in value {
            cart.0.insert(
                item.id.to_string(),
                item.quantity.unwrap_or_default().try_into().unwrap(),
            );
        }
        cart
    }
}

impl Default for ShoppingCart {
    fn default() -> Self {
        ShoppingCart(HashMap::<String, u8>::new())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StripeData {
    pub products: Vec<stripe_retypes::DbProduct>,
    pub customers: Vec<stripe_retypes::DbCustomer>,
    pub checkout_sessions: Vec<stripe_retypes::DbCheckoutSession>,
    pub default_shipping_rate_id: String,
    pub free_shipping_rate_id: String,
}

/// Find if there is existing session by id
#[leptos::server(name = CheckoutSessionMatches)]
pub async fn find_checkout_session_matches(
    checkout_sessionid: String,
) -> Result<bool, ServerFnError> {
    use stripe::*;
    let client = Client::new(match std::env::var("STRIPE_KEY") {
        Ok(ok) => ok,
        Err(err) => {
            log::error!("{:#?}", err);
            return Err(ServerFnError::ServerError(err.to_string()));
        }
    });

    let stripe_data: StripeData = stripe_stater().await?;

    Ok(stripe_data.checkout_sessions.iter().any(|session| {
        session.id == checkout_sessionid
            && session
                .status
                .clone()
                .map_or(false, |status| status == DbCheckoutSessionStatus::Open)
    }))
}

/// Creates new checkout session via stripe API using shopping cart items from client
#[leptos::server(name = NewCheckoutSession)]
pub async fn new_checkout_session(
    shopping_cart: HashMap<String, u8>, // shopping_cart input from storage
    checkout_sessionid: String,         // browser checkout_sessionid input from storage
) -> Result<DbCheckoutSession, ServerFnError> {
    let mut cart = ShoppingCart::default();
    cart.0 = shopping_cart;
    let shopping_cart = cart;

    use stripe::*;
    let client = Client::new(match std::env::var("STRIPE_KEY") {
        Ok(ok) => ok,
        Err(err) => {
            log::error!("{:#?}", err);
            return Err(ServerFnError::ServerError(err.to_string()));
        }
    });
    let stripe_data: StripeData = stripe_stater().await?;

    let base_url = match std::env::var("DEVPORT") {
        Ok(port) => "http://localhost:4444",
        Err(_) => "https://farmtasker.au",
    };

    let cancel_url = format!("{:#}/shop/cart", base_url);
    let success_url = format!("{:#}/success", base_url);

    let mut params = stripe::CreateCheckoutSession::new();
    params.cancel_url = Some(&cancel_url);
    params.success_url = Some(&success_url);
    params.customer = None;
    params.customer_creation = Some(stripe::CheckoutSessionCustomerCreation::IfRequired);
    params.shipping_address_collection =
        Some(stripe::CreateCheckoutSessionShippingAddressCollection {
            allowed_countries: vec![
                stripe::CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Au,
            ],
        });

    let total_price: i64 = shopping_cart.calculate_total_price(&stripe_data.products);

    let is_cart_under: bool = total_price < 30000;

    params.shipping_options = if is_cart_under {
        Some(vec![CreateCheckoutSessionShippingOptions {
            /// The ID of the Shipping Rate to use for this shipping option.
            shipping_rate: Some(stripe_data.default_shipping_rate_id),

            /// Parameters to be passed to Shipping Rate creation for this shipping option.
            shipping_rate_data: None,
        }])
    } else {
        Some(vec![CreateCheckoutSessionShippingOptions {
            /// The ID of the Shipping Rate to use for this shipping option.
            shipping_rate: Some(stripe_data.free_shipping_rate_id),

            /// Parameters to be passed to Shipping Rate creation for this shipping option.
            shipping_rate_data: None,
        }])
    };
    params.consent_collection = Some(CreateCheckoutSessionConsentCollection {
        payment_method_reuse_agreement: Some(CreateCheckoutSessionConsentCollectionPaymentMethodReuseAgreement {
            position: CreateCheckoutSessionConsentCollectionPaymentMethodReuseAgreementPosition::Hidden,
        }),
        ..Default::default()
    });

    // Collect additional information from your customer using custom fields.
    //
    // Up to 3 fields are supported.
    // params.custom_fields = Some(vec![CreateCheckoutSessionCustomFields {
    //     // Configuration for `type=dropdown` fields.
    //     dropdown: Some(CreateCheckoutSessionCustomFieldsDropdown {
    //         options: vec![CreateCheckoutSessionCustomFieldsDropdownOptions {
    //             // The label for the option, displayed to the customer.
    //             //
    //             // Up to 100 characters.
    //             label: String::from("I understand that I live within the delivery route "),

    //             // The value for this option, not displayed to the customer, used by your integration to reconcile the option selected by the customer.
    //             //
    //             // Must be unique to this option, alphanumeric, and up to 100 characters.
    //             value: String::from("deliverycollection"),
    //         }],
    //     }),
    //     // dropdown: None,

    //     // The label for the field, displayed to the customer.
    //     label: CreateCheckoutSessionCustomFieldsLabel {
    //         custom: String::from("How to collect your order?"),
    //         type_: CreateCheckoutSessionCustomFieldsLabelType::Custom,
    //     },

    //     // Configuration for `type=numeric` fields.
    //     // numeric: Some(CreateCheckoutSessionCustomFieldsNumeric {
    //     //     ..Default::default()
    //     // }),
    //     numeric: None,

    //     // Whether the customer is required to complete the field before completing the Checkout Session.
    //     //
    //     // Defaults to `false`.
    //     optional: Some(false),

    //     // Configuration for `type=text` fields.
    //     // text: Some(CreateCheckoutSessionCustomFieldsText {}),
    //     text: None,

    //     // The type of the field.
    //     type_: CreateCheckoutSessionCustomFieldsType::Dropdown,

    //     // String of your choice that your integration can use to reconcile this field.
    //     //
    //     // Must be unique to this field, alphanumeric, and up to 200 characters.
    //     key: String::from("deliveryconsent"),
    //     // ..Default::default()
    // }]);

    params.custom_text = Some(CreateCheckoutSessionCustomText {
        shipping_address: Some(CreateCheckoutSessionCustomTextShippingAddress {
            message: "We make deliveries only within Tasmania Derwent Valley or Hobart area."
                .to_string(),
        }),
        after_submit: Some(CreateCheckoutSessionCustomTextAfterSubmit {
            message: "We make deliveries only within Tasmania Derwent Valley or Hobart area."
                .to_string(),
        }),
        ..Default::default()
    });
    params.phone_number_collection =
        Some(stripe::CreateCheckoutSessionPhoneNumberCollection { enabled: true });
    params.ui_mode = Some(stripe::CheckoutSessionUiMode::Hosted);
    params.mode = Some(stripe::CheckoutSessionMode::Payment);
    params.billing_address_collection =
        Some(stripe::CheckoutSessionBillingAddressCollection::Required);
    params.currency = Some(stripe::Currency::AUD);

    let mut line_items_vec = Vec::new();

    for (product_id, quantity) in &shopping_cart.0 {
        if let Some(product) = stripe_data.products.iter().find(|p| p.id == *product_id) {
            let line_item = CreateCheckoutSessionLineItems {
                adjustable_quantity: Some(CreateCheckoutSessionLineItemsAdjustableQuantity {
                    enabled: true,
                    maximum: Some(20),
                    minimum: Some(1),
                }),
                quantity: Some((*quantity).into()),
                price: Some(product.default_price.clone().expect("NO PRICE!").id),
                ..Default::default()
            };
            line_items_vec.push(line_item);
        }
    }
    params.line_items = Some(line_items_vec);
    params.expand = &["line_items", "line_items.data.price.product"];

    let new_session = stripe::CheckoutSession::create(&client, params).await?;

    info!(
        "Created NEW checkout session: {:#?}, for {:#?} $AUD. (Created: {:#?} / Expires at: {:#?} )",
        &new_session.id,
        new_session.amount_total.unwrap_or(0).clone() as f64 / 100.0,
        &new_session.created,
        &new_session.expires_at
    );

    stripe_sync();

    leptos_axum::redirect(match &new_session.url.clone() {
        Some(url) => url,
        None => "/cancel",
    });

    Ok(new_session.into())
}

#[leptos::server(
    name = RefreshLocalProductInfo,
    endpoint = "refresh_local_products_info", // WORKING BUT TODO IMPLEMENT AUTHENTIFICATION
)]
pub async fn refresh_local_product_info(rewrite: bool) -> Result<String, leptos::ServerFnError> {
    use std::fs::File;
    use std::io::Read;
    use std::io::Write;
    use std::path::Path;
    use stripe::*;

    tracing::info!("");
    tracing::info!("Refreshing Local CfgProducts...");

    // Retrieve the LEPTOS_SITE_ROOT environment variable for path of the data file
    let site_root = std::env::var("LEPTOS_SITE_ROOT").unwrap_or_else(|_| "site".to_string());
    let assets_dir = std::env::var("LEPTOS_ASSETS_DIR").unwrap_or_else(|_| "public".to_string());

    // Config file paths
    let products_config_file_path = Path::new(&site_root).join("products_config.json");
    let products_config_public_file_path = Path::new(&assets_dir).join("products_config.json");

    if products_config_file_path.exists() {
        if rewrite {
            // If the file exists delete it, then recreate new one by serializing StripeData::new_fetch()
            let stripe_data = StripeData::new_fetch().await?;
            let stripe_products_config = StripeData::derive_products_config(stripe_data);

            // MAKE SCAN OF ASSET IMAGES IN DIR
            // Attach them to the local_images in CfgProduct
            let updated_stripe_products_config: CfgProducts =
                add_images_to_products_config(stripe_products_config.clone()).await?;

            write_products_config(updated_stripe_products_config, true).await
        } else {
            // Refresh file with new products from stripe api
            let stripe_data = StripeData::new_fetch().await?;
            let stripe_products_config: CfgProducts =
                StripeData::derive_products_config(stripe_data);

            let local_products_config: CfgProducts = fetch_local_product_info().await?;

            let mut h: Vec<CfgProduct> = Vec::new();
            // Add all local products to the vector
            for p in &local_products_config.0 {
                h.push(p.clone()); // Push existing products
            }

            // Add missing Stripe products to the vector
            for s in &stripe_products_config.0 {
                if !local_products_config
                    .0
                    .iter()
                    .any(|p| p.stripe_id == s.stripe_id)
                {
                    h.push(s.clone()); // Push only missing Stripe products
                }
            }

            // Create new products config by adding missing products from stripedata to existing config if the local config is missing the products by id
            // MAKE SCAN OF ASSET IMAGES IN DIR
            // Attach them to the local_images in CfgProduct
            let updated_products_config: CfgProducts =
                add_images_to_products_config(CfgProducts(h)).await?;

            // Write serialized local data updated with new products from stripe api
            write_products_config(updated_products_config, true).await
        }
    } else {
        // If the file doesn't exist, just create new one by serializing StripeData::new_fetch()
        let stripe_data = StripeData::new_fetch().await?;
        let stripe_products_config: CfgProducts = StripeData::derive_products_config(stripe_data);

        // MAKE SCAN OF ASSET IMAGES IN DIR
        // Attach them to the local_images in CfgProduct
        let updated_stripe_products_config: CfgProducts =
            add_images_to_products_config(stripe_products_config.clone()).await?;

        write_products_config(updated_stripe_products_config.clone(), true).await
    }
}

/// Fetches the Product Info from local automatically deserialized json file
/// If file doesn't exist it serializes a new file from products data inside StripeData
/// Returns Vec of Products parameters like name and price and their images
#[leptos::server(
    name = FetchLocalProductInfo,
    endpoint = "fetch_products_config"
)]
pub async fn fetch_local_product_info() -> Result<CfgProducts, leptos::ServerFnError> {
    use std::fs::File;
    use std::io::Read;
    use std::io::Write;
    use std::path::Path;
    use stripe::*;

    info!("");
    info!("Fetching Local CfgProducts...");

    // Retrieve the LEPTOS_SITE_ROOT environment variable for path of the data file
    let site_root = std::env::var("LEPTOS_SITE_ROOT").unwrap_or_else(|_| "site".to_string());
    let assets_dir = std::env::var("LEPTOS_ASSETS_DIR").unwrap_or_else(|_| "public".to_string());

    let products_config_file_path = Path::new(&site_root).join("products_config.json");
    let products_config_public_file_path = Path::new(&assets_dir).join("products_config.json");

    let final_products_config: CfgProducts = if products_config_file_path.exists() {
        // if file exists then just read it into CfgProducts from leptos site-root only and return fetched data
        let products_config_file_contents = std::fs::read_to_string(products_config_file_path)?;
        let products_config: CfgProducts = serde_json::from_str(&products_config_file_contents)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        for p in &products_config.0 {
            tracing::info!(
                "CONFIG: #{:?} {:?} - #{:?} AUD ",
                p.item_number.unwrap_or(-1),
                p.name,
                p.price.clone().unwrap().unit_amount.unwrap() as f64 / 100.0
            );
        }

        products_config
    } else {
        // If the file doesn't exist, create new one by serializing StripeData::new_fetch()
        let stripe_data = StripeData::new_fetch().await?;
        let stripe_products_config: CfgProducts = StripeData::derive_products_config(stripe_data);

        write_products_config(stripe_products_config.clone(), true).await?;

        stripe_products_config
    };

    tracing::info!("CONFIG TOTAL: {:?}", final_products_config.clone().0.len());

    Ok(final_products_config)
}

/// Adds images to CfgProducts from assets
#[leptos::server(
    name = AddImagesToProductsConfig
)]
async fn add_images_to_products_config(
    products_config: CfgProducts,
) -> Result<CfgProducts, ServerFnError> {
    use std::fs::File;
    use std::io::Read;
    use std::io::Write;
    use std::path::Path;

    // Retrieve the LEPTOS_SITE_ROOT environment variable for path of the data file
    let site_root = std::env::var("LEPTOS_SITE_ROOT").unwrap_or_else(|_| "site".to_string());

    let products_assets_dir = format!("{}/", site_root);

    &products_config.0.clone().iter().map(|n| {
        assert_eq!(n.images.is_some(), true);
        println!("{:?}", n.item_number.unwrap());
    });

    let updated_products_config = products_config;

    Ok(updated_products_config)
}

/// Writes the config file of CfgProducts
#[leptos::server(
    name = WriteProductsConfig
)]
pub async fn write_products_config(
    products_config: CfgProducts,
    rewrite: bool,
) -> Result<String, leptos::ServerFnError> {
    use std::fs::File;
    use std::io::Read;
    use std::io::Write;
    use std::path::Path;

    // Retrieve the LEPTOS_SITE_ROOT environment variable for path of the data file
    let site_root = std::env::var("LEPTOS_SITE_ROOT").unwrap_or_else(|_| "site".to_string());
    let assets_dir = std::env::var("LEPTOS_ASSETS_DIR").unwrap_or_else(|_| "public".to_string());

    // Config file paths
    let products_config_file_path = Path::new(&site_root).join("products_config.json");
    let products_config_public_file_path = Path::new(&assets_dir).join("products_config.json");

    std::fs::create_dir_all(&site_root); // safe measure if dir doesn't exist, create it
    std::fs::create_dir_all(&assets_dir);

    if rewrite {
        // remove existing file
        std::fs::remove_file(products_config_file_path.clone());
        std::fs::remove_file(products_config_public_file_path.clone());
        info!("Removed products config file.")
    }

    let json_data = serde_json::to_string_pretty(&products_config)?;
    std::fs::write(&products_config_file_path, json_data.clone())?; // write to target/site/
    std::fs::write(&products_config_public_file_path, json_data.clone())?; // write to public/
    info!(
        "Written products config file with synced data at: {}",
        products_config_file_path.display()
    );
    info!(
        "Written products config file with synced data at: {}",
        products_config_public_file_path.display()
    );
    Ok("Ok".into())
}

// Fetches the data from stripe api and serializes it into StripeData struct returning it
use log::*;
use stripe_retypes::{DbCheckoutSession, DbCheckoutSessionStatus, DbProduct};
#[leptos::server(
    name = FetchStripeData,
    // endpoint = "fetch_stripe_data",
)]
pub async fn fetch_stripe_data() -> Result<StripeData, leptos::ServerFnError> {
    info!("New StripeData fetch api call to Stripe...");

    use stripe::*;
    let client = Client::new(match std::env::var("STRIPE_KEY") {
        Ok(ok) => ok,
        Err(err) => {
            log::error!("{:#?}", err);
            return Err(ServerFnError::ServerError(err.to_string()));
        }
    });

    // Products
    let mut product_list_params = ListProducts::new();
    product_list_params.active = Some(true);
    product_list_params.expand = &["data.default_price"];
    product_list_params.limit = Some(100);
    let list_of_products_from_stripe_api = match Product::list(&client, &product_list_params).await
    {
        Ok(list) => list,
        Err(err) => {
            log::error!("{:#?}", err);
            return Err(ServerFnError::ServerError(err.to_string()));
        }
    };
    // tracing::info!("{:#?}", list_of_products_from_stripe_api); // DEBUG product info

    // Customers
    let mut customer_list_params = ListCustomers::new();
    customer_list_params.limit = Some(20); // WARN INFO important to not forget to change at some point
    let list_of_customers_from_stripe_api =
        match Customer::list(&client, &customer_list_params).await {
            Ok(list) => list,
            Err(err) => {
                log::error!("{:#?}", err);
                return Err(ServerFnError::ServerError(err.to_string()));
            }
        };

    // Checkout Sessions
    let mut checkout_session_list_params = ListCheckoutSessions::new();
    checkout_session_list_params.expand = &["data.line_items"];
    let list_of_checkout_sessions_from_stripe_api =
        match CheckoutSession::list(&client, &checkout_session_list_params).await {
            Ok(list) => list,
            Err(err) => {
                log::error!("{:#?}", err);
                return Err(ServerFnError::ServerError(err.to_string()));
            }
        };

    // Shipping Rates
    let mut shipping_rates_list_params = ListShippingRates::new();
    shipping_rates_list_params.active = Some(true);
    let list_of_shipping_rates_from_stripe_api: List<ShippingRate> =
        match ShippingRate::list(&client, &shipping_rates_list_params).await {
            Ok(list) => list,
            Err(err) => {
                log::error!("{:#?}", err);
                return Err(ServerFnError::ServerError(err.to_string()));
            }
        };

    let default_shipping_rate_id: String = match {
        list_of_shipping_rates_from_stripe_api
            .data
            .iter()
            .find(|rate| {
                if let Some(fixed_amount) = &rate.fixed_amount {
                    if fixed_amount.amount > 0 && fixed_amount.currency == Currency::AUD {
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            })
    } {
        Some(first_shipping_rate) => {
            // info!(
            //     "Found Default Shipping Rate ID: {:#?} for {:#?}$",
            //     first_shipping_rate.id.to_string().clone(),
            //     first_shipping_rate.fixed_amount.clone().unwrap().amount as f64 / 100.0,
            // );
            first_shipping_rate.id.to_string()
        }
        None => {
            let mut create_default_shipping_rate_params = CreateShippingRate {
                delivery_estimate: Some(CreateShippingRateDeliveryEstimate {
                    maximum: Some(CreateShippingRateDeliveryEstimateMaximum {
                        unit: CreateShippingRateDeliveryEstimateMaximumUnit::Day,
                        value: 4,
                    }),
                    minimum: Some(CreateShippingRateDeliveryEstimateMinimum {
                        unit: CreateShippingRateDeliveryEstimateMinimumUnit::Day,
                        value: 7,
                    }),
                }),
                display_name: "Default Created Shipping Rate",
                expand: &[],
                fixed_amount: Some(CreateShippingRateFixedAmount {
                    amount: 1000, // 10$AUD
                    currency: Currency::AUD,
                    currency_options: None,
                }),
                metadata: None,
                tax_behavior: None,
                tax_code: None,
                type_: Some(ShippingRateType::FixedAmount),
            };
            info!("Creating New Default Shipping Rate.");
            let shipping_rate =
                match ShippingRate::create(&client, create_default_shipping_rate_params).await {
                    Ok(rate) => rate,
                    Err(err) => {
                        log::error!("{:#?}", err);
                        return Err(ServerFnError::ServerError(err.to_string()));
                    }
                };
            shipping_rate.id.to_string()
        }
    };

    let free_shipping_rate_id: String = match {
        list_of_shipping_rates_from_stripe_api
            .data
            .iter()
            .find(|rate| {
                if let Some(fixed_amount) = &rate.fixed_amount {
                    if fixed_amount.amount == 0 && fixed_amount.currency == Currency::AUD {
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            })
    } {
        Some(free_shipping_rate) => {
            // info!(
            //     "Found Free Shipping Rate ID: {:#?} for {:#?}$",
            //     free_shipping_rate.id.to_string().clone(),
            //     free_shipping_rate.fixed_amount.clone().unwrap().amount as f64 / 100.0
            // );
            free_shipping_rate.id.to_string()
        }
        None => {
            let mut create_free_shipping_rate_params = CreateShippingRate {
                delivery_estimate: Some(CreateShippingRateDeliveryEstimate {
                    maximum: Some(CreateShippingRateDeliveryEstimateMaximum {
                        unit: CreateShippingRateDeliveryEstimateMaximumUnit::Day,
                        value: 7,
                    }),
                    minimum: Some(CreateShippingRateDeliveryEstimateMinimum {
                        unit: CreateShippingRateDeliveryEstimateMinimumUnit::Day,
                        value: 4,
                    }),
                }),
                display_name: "Free Created Shipping Rate",
                expand: &[],
                fixed_amount: Some(CreateShippingRateFixedAmount {
                    amount: 0,
                    currency: Currency::AUD,
                    currency_options: None,
                }),
                metadata: None,
                tax_behavior: None,
                tax_code: None,
                type_: Some(ShippingRateType::FixedAmount),
            };
            info!("Creating New Free Shipping Rate.");
            let shipping_rate =
                match ShippingRate::create(&client, create_free_shipping_rate_params).await {
                    Ok(rate) => rate,
                    Err(err) => {
                        log::error!("{:#?}", err);
                        return Err(ServerFnError::ServerError(err.to_string()));
                    }
                };
            shipping_rate.id.to_string()
        }
    };

    // info!("Default Shipping Rate ID: {:#?}", default_shipping_rate_id);
    // info!("Free Shipping Rate ID: {:#?}", free_shipping_rate_id);
    // leptos::logging::log!("\n");

    Ok(StripeData::new(
        list_of_products_from_stripe_api,
        list_of_customers_from_stripe_api,
        list_of_checkout_sessions_from_stripe_api,
        default_shipping_rate_id,
        free_shipping_rate_id,
    ))
}

#[cfg(feature = "ssr")]
pub mod sync {
    #![allow(unused)]

    use super::*;
    use axum::{
        response::{ErrorResponse, IntoResponse},
        Extension, Json,
    };
    use leptos::ServerFnError;
    use log::*;
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    use stripe::*;
    use stripe_retypes::*;

    impl StripeData {
        pub fn new(
            products: List<Product>,
            customers: List<Customer>,
            checkout_sessions: List<CheckoutSession>,
            default_shipping_rate_id: String,
            free_shipping_rate_id: String,
        ) -> Self {
            StripeData {
                products: products.data.into_iter().map(|x| x.into()).collect(),
                customers: customers.data.into_iter().map(|x| x.into()).collect(),
                checkout_sessions: checkout_sessions
                    .data
                    .into_iter()
                    .map(|x| x.into())
                    .collect(),
                default_shipping_rate_id,
                free_shipping_rate_id,
            }
        }
        pub fn derive_products_config(self) -> CfgProducts {
            let mut v = CfgProducts(Vec::new());
            for p in self.products {
                v.0.push(CfgProduct {
                    item_number: match p.metadata {
                        Some(ref x) => x.get("item_number").cloned().and_then(|i| i.parse().ok()),
                        _ => None,
                    },
                    stripe_id: Some(p.id),
                    name: p.name,
                    price: p.default_price,
                    description: p.description,
                    local_images: None,
                    images: p.images,
                    metadata: p.metadata,
                })
            }
            v
        }
        pub async fn new_fetch() -> Result<Self, ServerFnError> {
            fetch_stripe_data().await
        }
    }

    impl CfgProducts {
        pub async fn new_fetch_local() -> Result<Self, ServerFnError> {
            fetch_local_product_info().await
        }
        pub async fn fetch_reset() -> Result<String, ServerFnError> {
            refresh_local_product_info(true).await
        }
        pub async fn fetch_update() -> Result<String, ServerFnError> {
            refresh_local_product_info(false).await
        }
    }

    impl From<Vec<stripe::CheckoutSessionItem>> for ShoppingCart {
        fn from(value: Vec<stripe::CheckoutSessionItem>) -> Self {
            let mut cart = ShoppingCart::default();
            value.into_iter().map(|item| {
                cart.0
                    .insert(item.id.to_string(), item.quantity.unwrap_or_default() as u8)
            });
            cart
        }
    }

    impl From<CheckoutSession> for DbCheckoutSession {
        fn from(value: CheckoutSession) -> Self {
            DbCheckoutSession {
                id: value.id.to_string(),
                amount_subtotal: value.amount_subtotal,
                amount_total: value.amount_total,
                cancel_url: value.cancel_url,
                created: Some(value.created),
                customer: match value.customer {
                    Some(x) => x.into_object().map(|x| x.into()),
                    _ => None,
                },
                customer_email: value.customer_email,
                expires_at: Some(value.expires_at),
                line_items: match value.line_items {
                    Some(x) => Some(x.data.into_iter().map(|x| x.into()).collect()),
                    None => None,
                },
                livemode: value.livemode,
                metadata: value.metadata,
                mode: match value.mode {
                    CheckoutSessionMode::Payment => DbCheckoutSessionMode::Payment,
                    CheckoutSessionMode::Setup => DbCheckoutSessionMode::Setup,
                    CheckoutSessionMode::Subscription => DbCheckoutSessionMode::Subscription,
                },
                payment_status: match value.payment_status {
                    CheckoutSessionPaymentStatus::Paid => DbCheckoutSessionPaymentStatus::Paid,
                    CheckoutSessionPaymentStatus::Unpaid => DbCheckoutSessionPaymentStatus::Unpaid,
                    CheckoutSessionPaymentStatus::NoPaymentRequired => {
                        DbCheckoutSessionPaymentStatus::NoPaymentRequired
                    }
                },
                status: match value.status {
                    Some(x) => Some(match x {
                        CheckoutSessionStatus::Open => DbCheckoutSessionStatus::Open,
                        CheckoutSessionStatus::Expired => DbCheckoutSessionStatus::Expired,
                        CheckoutSessionStatus::Complete => DbCheckoutSessionStatus::Complete,
                    }),
                    None => None,
                },
                success_url: value.success_url,
                url: value.url,
            }
        }
    }

    impl From<CheckoutSessionItem> for DbCheckoutSessionItem {
        fn from(value: CheckoutSessionItem) -> Self {
            DbCheckoutSessionItem {
                id: value.id.to_string(),
                amount_discount: value.amount_discount,
                amount_subtotal: value.amount_subtotal,
                amount_total: value.amount_total,
                description: value.description,
                price: match value.price {
                    Some(x) => Some(x.into()),
                    None => None,
                },
                quantity: value.quantity,
            }
        }
    }

    impl From<Product> for DbProduct {
        fn from(value: Product) -> Self {
            DbProduct {
                id: value.id.to_string(),
                active: value.active.unwrap_or(false),
                created: value.created,
                default_price: match value.default_price {
                    Some(x) => x.into_object().map(|x| x.into()),
                    _ => None,
                },
                description: value.description,
                images: value.images,
                local_images: None,
                metadata: value.metadata,
                name: value.name.unwrap_or_default(),
                // package_dimensions: value.package_dimensions,
                unit_label: value.unit_label,
                updated: value.updated,
                url: value.url,
            }
        }
    }

    impl From<PriceBillingScheme> for DbPriceBillingScheme {
        fn from(value: PriceBillingScheme) -> Self {
            match value {
                PriceBillingScheme::PerUnit => DbPriceBillingScheme::PerUnit,
                PriceBillingScheme::Tiered => DbPriceBillingScheme::Tiered,
            }
        }
    }

    impl From<RecurringAggregateUsage> for DbRecurringAggregateUsage {
        fn from(value: RecurringAggregateUsage) -> Self {
            match value {
                RecurringAggregateUsage::LastDuringPeriod => {
                    DbRecurringAggregateUsage::LastDuringPeriod
                }
                RecurringAggregateUsage::LastEver => DbRecurringAggregateUsage::LastEver,
                RecurringAggregateUsage::Max => DbRecurringAggregateUsage::Max,
                RecurringAggregateUsage::Sum => DbRecurringAggregateUsage::Sum,
            }
        }
    }

    impl From<CustomUnitAmount> for DbCustomUnitAmount {
        fn from(value: CustomUnitAmount) -> Self {
            DbCustomUnitAmount {
                maximum: value.maximum,
                minimum: value.minimum,
                preset: value.preset,
            }
        }
    }

    impl From<RecurringInterval> for DbRecurringInterval {
        fn from(value: RecurringInterval) -> Self {
            match value {
                RecurringInterval::Day => DbRecurringInterval::Day,
                RecurringInterval::Month => DbRecurringInterval::Month,
                RecurringInterval::Week => DbRecurringInterval::Week,
                RecurringInterval::Year => DbRecurringInterval::Year,
            }
        }
    }

    impl From<Recurring> for DbRecurring {
        fn from(value: Recurring) -> Self {
            DbRecurring {
                aggregate_usage: value.aggregate_usage.map(|x| x.into()),
                interval: value.interval.into(),
                interval_count: value.interval_count,
                trial_period_days: value.trial_period_days,
                usage_type: value.usage_type.into(),
            }
        }
    }

    impl From<RecurringUsageType> for DbRecurringUsageType {
        fn from(value: RecurringUsageType) -> Self {
            match value {
                RecurringUsageType::Licensed => DbRecurringUsageType::Licensed,
                RecurringUsageType::Metered => DbRecurringUsageType::Metered,
            }
        }
    }

    impl From<PriceType> for DbPriceType {
        fn from(value: PriceType) -> Self {
            match value {
                PriceType::OneTime => DbPriceType::OneTime,
                PriceType::Recurring => DbPriceType::Recurring,
            }
        }
    }

    impl From<Price> for DbPrice {
        fn from(value: Price) -> Self {
            DbPrice {
                id: value.id.to_string(),
                active: value.active.unwrap_or(false),
                billing_scheme: value.billing_scheme.map(|x| x.into()),
                created: value.created,
                // currency: value.currency,
                // currency_options: value.currency_options,
                custom_unit_amount: value.custom_unit_amount.map(|x| x.into()),
                livemode: value.livemode.unwrap_or(false),
                lookup_key: value.lookup_key,
                metadata: value.metadata,
                nickname: value.nickname,
                product: value
                    .product
                    .unwrap_or_default()
                    .into_object()
                    .map(|x| x.id.to_string()),
                recurring: value.recurring.map(|x| x.into()),
                // tiers: value.tiers,
                // tiers_mode: value.tiers_mode,
                // transform_quantity: value.transform_quantity,
                type_: value.type_.map(|x| x.into()),
                unit_amount: value.unit_amount,
                unit_amount_decimal: value.unit_amount_decimal,
            }
        }
    }

    // impl Into<Price> for DbPrice {
    //     fn into(self) -> Price {
    //         Price
    //     }
    // }

    impl Object for DbPrice {
        type Id = String;

        fn id(&self) -> Self::Id {
            self.id.clone()
        }

        fn object(&self) -> &'static str {
            "dbprice"
        }
    }

    impl From<Address> for DbAddress {
        fn from(value: Address) -> Self {
            DbAddress {
                city: value.city,
                country: value.country,
                line1: value.line1,
                line2: value.line2,
                postal_code: value.postal_code,
                state: value.state,
            }
        }
    }

    impl From<Shipping> for DbShipping {
        fn from(value: Shipping) -> Self {
            DbShipping {
                address: value.address.map(|x| x.into()),
                carrier: value.carrier,
                name: value.name,
                phone: value.phone,
                tracking_number: value.tracking_number,
            }
        }
    }

    impl From<Customer> for DbCustomer {
        fn from(value: Customer) -> Self {
            DbCustomer {
                id: value.id.to_string(),
                address: value.address.map(|x| x.into()),
                balance: value.balance,
                // cash_balance: value.cash_balance,
                created: value.created,
                // currency: value.currency,
                // default_source: value.default_source.unwrap_or_default().into_object(),
                // delinquent: value.delinquent,
                description: value.description,
                // discount: value.discount,
                email: value.email,
                livemode: value.livemode.unwrap_or(false),
                metadata: value.metadata,
                name: value.name,
                phone: value.phone,
                shipping: value.shipping.map(|x| x.into()),
                // sources: value.sources,
            }
        }
    }
}
#[server (
    name = RedirectToUrl,
)]
pub async fn redirect_to_url(url: String) -> Result<(), leptos::ServerFnError> {
    leptos_axum::redirect(&url);
    Ok(())
}

/// Synchronizes state of data between the stripe api and the runtime state of StripeData struct
use leptos::*;
#[server (
    name = StripeSync,
    // endpoint = "sync", // WORKING BUT TODO IMPLEMENT AUTHENTIFICATION
)]
pub async fn stripe_sync() -> Result<serde_json::Value, leptos::ServerFnError> {
    use axum::http::HeaderMap;
    use log::*;
    use stripe::*;

    let state = match leptos::use_context::<Option<crate::AppState>>() {
        Some(ok) => {
            // leptos::logging::log!("GOT context AppState");
            ok
        }
        None => {
            // leptos::logging::log!("No context AppState");
            None
        }
    };
    let axum::extract::State(mut appstate): axum::extract::State<crate::AppState> =
        leptos_axum::extract_with_state(match &state {
            Some(x) => x,
            None => &AppState {
                stripe_data: None,
                products_config: None,
            },
        })
        .await?;

    info!("v----Starting sync of local StripeData with Stripe API----v");

    let new_stripedata: Option<StripeData> = match StripeData::new_fetch().await {
        Ok(ok) => Some(ok),
        Err(err) => {
            log::error!("Couldn't fetch new StripeData!!!: {:#?}", err);
            None
        }
    };

    appstate.stripe_data = match new_stripedata.clone() {
        Some(data) => {
            info!("v----Synced-StripeData----v");
            info!("Synchronized AppState with Stripe API");
            info!("Total Products: {:#?}", data.products.len());
            info!("Total Customers: {:#?}", data.customers.len());
            info!("Shiping Rate ID: {:#?}", data.default_shipping_rate_id);
            tracing::info!(
                "Total of currently Open \"Checkout Sessions\": {:}",
                data.checkout_sessions
                    .clone()
                    .into_iter()
                    .filter(|c| match &c.status {
                        Some(s) => match s {
                            crate::stripe_retypes::DbCheckoutSessionStatus::Complete => false,
                            crate::stripe_retypes::DbCheckoutSessionStatus::Expired => false,
                            crate::stripe_retypes::DbCheckoutSessionStatus::Open => true,
                        },
                        None => false,
                    })
                    .collect::<Vec<crate::stripe_retypes::DbCheckoutSession>>()
                    .len()
            );
            Some(data)
        }
        None => {
            log::error!("Couldn't update StripeData");
            return Err(leptos::ServerFnError::ServerError(
                "Couldn't update StripeData".into(),
            ));
        }
    };

    Ok(serde_json::json!({
        "code": match appstate.stripe_data.clone() {
            Some(_) => http::StatusCode::NO_CONTENT.to_string(),
            None => http::StatusCode::INTERNAL_SERVER_ERROR.to_string(),        },
        "count": {
            "products": match appstate.stripe_data.clone() {
                Some(data) => data.products.len(),
                None => 0,
            },
            "customers": match appstate.stripe_data.clone() {
                Some(data) => data.customers.len(),
                None => 0
            },
        },
    }))
}
