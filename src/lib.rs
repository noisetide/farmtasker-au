#![allow(unused)]
pub mod app;
pub mod error_template;
#[cfg(feature = "ssr")]
pub mod fileserv;
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
      name = Stater,
)]
pub async fn stater() -> Result<StripeData, leptos::ServerFnError> {
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
                stripe_api_key: None,
                stripe_data: None,
            },
        })
        .await?;

    // log::info!("Server data: {:#?}", data.stripe_data.clone());
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

use app::PagerPropsBuilder_Error_Missing_required_field_page;
use leptos::{create_effect, Serializable, ServerFnError};
use leptos_router::FromFormData;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppState {
    pub stripe_api_key: Option<String>,
    pub stripe_data: Option<StripeData>,
}

use std::collections::HashMap;
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ShoppingCart(HashMap<String, u8>);

impl ShoppingCart {
    pub fn add_single_product(&mut self, product_id: &String) {
        // If the product is already in the cart, increase its quantity by 1
        if let Some(quantity) = self.0.get_mut(product_id) {
            *quantity += 1;
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
    pub fn delete_product(&mut self, product_id: String) {
        self.0.remove(&product_id);
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
}

#[leptos::server(name = NewCheckoutSession)]
pub async fn new_checkout_session(
    shopping_cart: HashMap<String, u8>,
    checkout_sessionid: String,
) -> Result<String, ServerFnError> {
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

    let stripe_data: StripeData = stater().await?;

    let base_url = match std::env::var("DEVPORT") {
        Ok(port) => "http://localhost:4444",
        Err(err) => "https://farmtasker.au",
    };

    if stripe_data
        .checkout_sessions
        .iter()
        .any(|session| session.id == checkout_sessionid.clone())
    {
        info!(
            "Checkout Session with id '{:#?}' already exists!",
            checkout_sessionid.to_string().clone()
        );
        return Ok(checkout_sessionid.to_string());
    } else {
        let checkout_session = {
            let cancel_url = format!("{:#}/cancel", base_url);
            let success_url = format!("{:#}/success", base_url);

            let mut params = stripe::CreateCheckoutSession::new();
            params.cancel_url = Some(&cancel_url); // TODO!
            params.success_url = Some(&success_url); // TODO!
            params.customer = None;
            params.customer_creation = Some(stripe::CheckoutSessionCustomerCreation::IfRequired);
            params.shipping_address_collection =
                Some(stripe::CreateCheckoutSessionShippingAddressCollection {
                    allowed_countries: vec![
                        stripe::CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Au,
                    ],
                });
            params.custom_text = Some(stripe::CreateCheckoutSessionCustomText {
                after_submit: None,
                shipping_address: Some(stripe::CreateCheckoutSessionCustomTextShippingAddress {
                    message: "We make deliveries only within Derwent Valley in Tasmania"
                        .to_string(),
                }),
                submit: None,
                terms_of_service_acceptance: None,
            });
            params.phone_number_collection =
                Some(stripe::CreateCheckoutSessionPhoneNumberCollection { enabled: true });
            params.ui_mode = Some(stripe::CheckoutSessionUiMode::Hosted);
            params.mode = Some(stripe::CheckoutSessionMode::Payment);

            let mut vec_of_create_checkout_session_line_items =
                Vec::<stripe::CreateCheckoutSessionLineItems>::new();

            // params.shipping_options = Some(stripe::CreateCheckoutSessionShippingOptions)

            params.billing_address_collection =
                Some(stripe::CheckoutSessionBillingAddressCollection::Required);
            params.currency = Some(stripe::Currency::AUD);

            for (product_id, quantity) in &shopping_cart.0 {
                if let Some(product) = stripe_data.products.iter().find(|p| p.id == *product_id) {
                    let line_item = CreateCheckoutSessionLineItems {
                        quantity: Some((*quantity).into()),
                        price: Some(product.default_price.clone().expect("NO PRICE!").id),
                        ..Default::default()
                    };
                    vec_of_create_checkout_session_line_items.push(line_item);
                }
            }

            // params.line_items = Some(vec![stripe::CreateCheckoutSessionLineItems {
            //     quantity: Some(3),
            //     price: Some(price.id.to_string()),
            //     ..Default::default()
            // }]);
            params.line_items = Some(vec_of_create_checkout_session_line_items);
            params.expand = &["line_items", "line_items.data.price.product"];

            stripe::CheckoutSession::create(&client, params).await?
        };
        info!(
            "Created checkout session: {:#?}, for {:#?} $AUD. (Created: {:#?} / Expires at: {:#?} )",
            &checkout_session.id,
            checkout_session.amount_total.unwrap().clone() as f64 / 100.0,
            &checkout_session.created,
            &checkout_session.expires_at
        );

        Ok(checkout_session.id.to_string())
    }
}

use log::*;
use stripe_retypes::DbProduct;
#[leptos::server(
    // name = FetchStripeData,
    // endpoint = "fetch_stripe_data",
)]
pub async fn fetch_stripe_data() -> Result<StripeData, leptos::ServerFnError> {
    use stripe::*;
    let client = Client::new(match std::env::var("STRIPE_KEY") {
        Ok(ok) => ok,
        Err(err) => {
            log::error!("{:#?}", err);
            return Err(ServerFnError::ServerError(err.to_string()));
        }
    });

    let mut product_list_params = ListProducts::new();
    product_list_params.active = Some(true);
    product_list_params.expand = &["data.default_price"];
    let list_of_products_from_stripe_api = match Product::list(&client, &product_list_params).await
    {
        Ok(list) => list,
        Err(err) => {
            log::error!("{:#?}", err);
            return Err(ServerFnError::ServerError(err.to_string()));
        }
    };

    let customer_list_params = ListCustomers::new();
    let list_of_customers_from_stripe_api =
        match Customer::list(&client, &customer_list_params).await {
            Ok(list) => list,
            Err(err) => {
                log::error!("{:#?}", err);
                return Err(ServerFnError::ServerError(err.to_string()));
            }
        };
    let checkout_session_list_params = ListCheckoutSessions::new();
    let list_of_checkout_sessions_from_stripe_api =
        match CheckoutSession::list(&client, &checkout_session_list_params).await {
            Ok(list) => list,
            Err(err) => {
                log::error!("{:#?}", err);
                return Err(ServerFnError::ServerError(err.to_string()));
            }
        };

    info!("New fetch api call to Stripe...");
    Ok(StripeData::new(
        list_of_products_from_stripe_api,
        list_of_customers_from_stripe_api,
        list_of_checkout_sessions_from_stripe_api,
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
        ) -> Self {
            StripeData {
                products: products.data.into_iter().map(|x| x.into()).collect(),
                customers: customers.data.into_iter().map(|x| x.into()).collect(),
                checkout_sessions: checkout_sessions
                    .data
                    .into_iter()
                    .map(|x| x.into())
                    .collect(),
            }
        }
        pub async fn new_fetch() -> Result<Self, ServerFnError> {
            fetch_stripe_data().await
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
                    Some(x) => Some(x.into_object().unwrap().into()),
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
                    Some(x) => Some(x.into_object().unwrap().into()),
                    _ => None,
                },
                description: value.description,
                images: value.images,
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
    name = Redirect,
)]
pub async fn redirect(url: String) -> Result<(), leptos::ServerFnError> {
    leptos_axum::redirect(&url);
    Ok(())
}

use leptos::*;
#[server (
    name = StripeSync,
    // endpoint = "sync", // WORKING BUT TODO IMPLEMENT AUTHENTIFICATION
)]
pub async fn stripe_sync() -> Result<serde_json::Value, leptos::ServerFnError> {
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
                stripe_api_key: None,
                stripe_data: None,
            },
        })
        .await?;

    info!("Starting sync of local StripeData with Stripe API...");

    let new_stripedata: Option<StripeData> = match StripeData::new_fetch().await {
        Ok(ok) => Some(ok),
        Err(err) => {
            log::error!("Couldn't fetch new StripeData!!!: {:#?}", err);
            None
        }
    };

    appstate.stripe_data = match new_stripedata.clone() {
        Some(data) => {
            info!("v----Synced-StripeData---v");
            info!("Synchronized AppState with Stripe API");
            info!("Total Products: {:#?}", data.products.len());
            info!("Total Customers: {:#?}", data.customers.len());
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
