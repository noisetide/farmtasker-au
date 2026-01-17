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
                stripe_id: p.id,
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
