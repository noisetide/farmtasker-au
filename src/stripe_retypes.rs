use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DbProduct {
    pub id: String,
    pub active: bool,
    // Measured in seconds since the Unix epoch.
    pub created: Option<i64>,
    pub default_price: Option<DbPrice>,
    pub description: Option<String>,
    // A list of up to 8 URLs of images for this product, meant to be displayable to the customer
    pub images: Option<Vec<String>>,
    pub metadata: Option<HashMap<String, String>>,
    pub name: String,
    // pub package_dimensions: Option<PackageDimensions>,
    // TLDR AMOUNT OF PRODUCT INCLUDEDI
    // A label that represents units of this product.
    // When set, this will be included in customers' receipts, invoices, Checkout, and the customer portal.
    pub unit_label: Option<String>,
    pub updated: Option<i64>,
    // url of this product
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DbPriceBillingScheme {
    PerUnit,
    Tiered,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DbCustomUnitAmount {
    pub maximum: Option<i64>,
    pub minimum: Option<i64>,
    pub preset: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DbRecurring {
    pub aggregate_usage: Option<DbRecurringAggregateUsage>,
    pub interval: DbRecurringInterval,
    pub interval_count: u64,
    pub trial_period_days: Option<u32>,
    pub usage_type: DbRecurringUsageType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DbRecurringAggregateUsage {
    LastDuringPeriod,
    LastEver,
    Max,
    Sum,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DbRecurringInterval {
    Day,
    Month,
    Week,
    Year,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DbRecurringUsageType {
    Licensed,
    Metered,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DbPriceType {
    OneTime,
    Recurring,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DbPrice {
    pub id: String,
    pub active: bool,
    pub billing_scheme: Option<DbPriceBillingScheme>,
    pub created: Option<i64>,
    // pub currency: Option<Currency>, // PLEASE default to AUD
    // pub currency_options: Option<HashMap<Currency, CurrencyOption>>,
    pub custom_unit_amount: Option<DbCustomUnitAmount>,
    pub livemode: bool,
    pub lookup_key: Option<String>,
    pub metadata: Option<HashMap<String, String>>,
    pub nickname: Option<String>,
    pub product: Option<String>, // instead of just Option<DbProduct>
    pub recurring: Option<DbRecurring>,
    // pub tiers: Option<Vec<PriceTier>>,
    // pub tiers_mode: Option<PriceTiersMode>,
    // pub transform_quantity: Option<TransformQuantity>,
    pub type_: Option<DbPriceType>,
    pub unit_amount: Option<i64>,
    pub unit_amount_decimal: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DbAddress {
    pub city: Option<String>,
    pub country: Option<String>,
    pub line1: Option<String>,
    pub line2: Option<String>,
    pub postal_code: Option<String>,
    pub state: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DbCustomer {
    pub id: String,
    pub address: Option<DbAddress>,
    pub balance: Option<i64>,
    // pub cash_balance: Option<CashBalance>,
    pub created: Option<i64>,
    // pub currency: Option<Currency>, // PLEASE default to AUD
    // pub default_source: Option<PaymentSource>,
    // pub delinquent: Option<bool>,
    pub description: Option<String>,
    // pub discount: Option<Discount>,
    pub email: Option<String>,
    // pub invoice_credit_balance: Option<u64>,
    // pub invoice_prefix: Option<String>,
    // pub invoice_settings: Option<InvoiceSettingCustomerSetting>,
    // pub next_invoice_sequence: Option<i64>,
    pub livemode: bool,
    pub metadata: Option<HashMap<String, String>>,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub shipping: Option<DbShipping>,
    // pub sources: Vec<PaymentSource>,
    // pub subscriptions: Option<List<Subscription>>,
    // pub tax: Option<CustomerTax>,
    // pub tax_exempt: Option<CustomerTaxExempt>,
    // pub tax_ids: Option<List<TaxId>>,
    // pub test_clock: Option<TestHelpersTestClock>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DbShipping {
    pub address: Option<DbAddress>,
    pub carrier: Option<String>,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub tracking_number: Option<String>,
}
