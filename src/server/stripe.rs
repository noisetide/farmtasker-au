use crate::{AppState, StripeData};
use leptos::*;
use leptos::ServerFnError;
use log::*;

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

#[server (
    name = StripeSync,
    endpoint = "sync", // WORKING BUT TODO IMPLEMENT AUTHENTIFICATION
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
