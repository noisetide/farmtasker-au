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
    log::info!("Hello!");
    leptos::mount_to_body(App);
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppState {
    pub id: u64,
    pub stripe_api_key: String,
    pub stripe_data: StripeData,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StripeData {
    pub products: Vec<stripe_retypes::DbProduct>,
    pub customers: Vec<stripe_retypes::DbCustomer>,
}

#[cfg(feature = "ssr")]
pub mod sync {
    #![allow(unused)]

    use crate::AppState;
    use axum::{
        response::{ErrorResponse, IntoResponse},
        Extension, Json,
    };
    use http::StatusCode;
    use log::*;
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    use stripe::{
        Address, CustomUnitAmount, Metadata, PriceBillingScheme, PriceId, RecurringAggregateUsage,
        RecurringInterval, RecurringUsageType, *,
    };
    use stripe_retypes::{
        DbAddress, DbCustomUnitAmount, DbPriceBillingScheme, DbPriceType, DbRecurring,
        DbRecurringAggregateUsage, DbRecurringInterval, DbRecurringUsageType, DbShipping,
    };

    use super::*;
    use crate::stripe_retypes::*;

    impl StripeData {
        pub fn new(products: List<Product>, customers: List<Customer>) -> Self {
            StripeData {
                products: products.data.into_iter().map(|x| x.into()).collect(),
                customers: customers.data.into_iter().map(|x| x.into()).collect(),
            }
        }
        pub async fn new_fetch() -> Result<Self, ErrorResponse> {
            let client = Client::new(match std::env::var("REMOVED") {
                Ok(ok) => ok,
                Err(err) => err.to_string(),
            });

            let mut product_list_params = ListProducts::new();
            product_list_params.active = Some(true);
            product_list_params.expand = &["data.default_price"];

            let list_of_products_from_stripe_api =
                match Product::list(&client, &product_list_params).await {
                    Ok(list) => list,
                    Err(err) => {
                        log::error!("{:#?}", err);
                        return Err(ErrorResponse::from(Json::from(err.to_string())));
                        // return Err(err);
                    }
                };

            let customer_list_params = ListCustomers::new();
            let list_of_customers_from_stripe_api =
                match Customer::list(&client, &customer_list_params).await {
                    Ok(list) => list,
                    Err(err) => {
                        log::error!("{:#?}", err);
                        return Err(ErrorResponse::from(Json::from(err.to_string())));
                        // return Err(err);
                    }
                };

            info!("New fetch api call to Stripe...");
            Ok(StripeData::new(
                list_of_products_from_stripe_api,
                list_of_customers_from_stripe_api,
            ))
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

    pub async fn stripe_sync(
        mut appstate: Extension<AppState>,
    ) -> Result<impl IntoResponse, ErrorResponse> {
        info!("Starting sync of local StripeData with Stripe API...");
        let key = appstate.id.clone();
        trace!("Here {:#?}", key);

        let client = Client::new(std::env::var("REMOVED").unwrap().as_str());

        let customer_list_params = ListCustomers::new();
        let list_of_customers_from_stripe_api =
            match Customer::list(&client, &customer_list_params).await {
                Ok(list) => list,
                Err(err) => {
                    log::error!("{:#?}", err);
                    return Err(ErrorResponse::from(Json::from(err.to_string())));
                }
            };

        match list_of_customers_from_stripe_api.data.len() {
            0 => {
                log::info!("No Customers");
            }
            x if x > 0 => {
                log::info!("Amount of Customers: {:?}", x);
            }
            _ => {}
        };

        let mut product_list_params = ListProducts::new();
        product_list_params.active = Some(true);
        product_list_params.expand = &["data.default_price"];

        let list_of_products_from_stripe_api =
            match Product::list(&client, &product_list_params).await {
                Ok(list) => list,
                Err(err) => {
                    log::error!("{:#?}", err);
                    return Err(ErrorResponse::from(Json::from(err.to_string())));
                }
            };

        match list_of_products_from_stripe_api.data.len() {
            0 => {
                log::info!("No Products");
            }
            x if x > 0 => {
                log::info!("Amount of Products#: {:?}", x);
            }
            _ => {}
        };

        let data = StripeData::new(
            list_of_products_from_stripe_api.clone(),
            list_of_customers_from_stripe_api.clone(),
        );

        info!("Updating local AppState with synced data from Stripe API");
        appstate.stripe_data = data.clone();
        let value = serde_json::json!({
            "code": StatusCode::NO_CONTENT.to_string(),
            "count": {
                "products": data.clone().products.len(),
                "customers": data.clone().customers.len()
            }
        });
        Ok(Json::from(value))
    }
}
