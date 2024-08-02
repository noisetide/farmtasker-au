#![allow(unused)]
use crate::*;
use http::StatusCode;
use leptos::error::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use stripe::{Metadata, *};

#[cfg(feature = "ssr")]
use axum::{
    extract::{FromRequest, State},
    response::*,
    Extension, Json,
};

pub async fn stripe_sync(
    Extension(shared_state): Extension<Arc<Mutex<AppState>>>,
    client: stripe::Client,
) -> Result<Json<serde_json::Value>> {
    todo!();
    // Clear DB
    // shared_state.persist.clear();
    // let client = Client::new(&shared_state.stripe_key);

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
            log::info!("Customers#: {:?}", x);
        }
        _ => {}
    }

    let mut product_list_params = ListProducts::new();
    product_list_params.active = Some(true);
    product_list_params.expand = &["data.default_price"];

    let mut list_of_products_from_stripe_api =
        match Product::list(&client, &product_list_params).await {
            Ok(list) => list,
            Err(err) => {
                log::error!("{:#?}", err);
                return Err(ErrorResponse::from(Json::from(err.to_string())));
            }
        };

    let listed_price = list_of_products_from_stripe_api.data[0]
        .default_price
        .clone()
        .unwrap()
        .into_object()
        .unwrap();

    log::info!("First api default price: {:?}", &listed_price);

    let saver: DbPrice = DbPrice::from(listed_price);

    type Saver = DbPrice;

    // shared_state.persist.save::<Saver>("price", saver).unwrap();

    // let loaded_price = shared_state.persist.load::<Saver>("price").unwrap();

    // log::info!("First load default price: {:#?}", loaded_price);

    for i in list_of_products_from_stripe_api.data.iter_mut() {
        i.default_price = None;
    }

    let data = StripeData::new(
        list_of_products_from_stripe_api.clone(),
        list_of_customers_from_stripe_api.clone(),
    );

    // match shared_state.persist.save::<StripeData>("data", data) {
    //     Ok(_) => {
    //         let list_data = shared_state.persist.list().expect("Could not load data!");

    //         log::info!("Saved Data: {:#?}", list_data);

    //         // print out 10 products from stripe api
    //         // for (n, x) in list_of_products_from_stripe_api.data.iter().enumerate() {
    //         //     if n >= 10 {
    //         //         break;
    //         //     }
    //         //     log::info!(
    //         //         "{n}: {:#?}, [{:?}]",
    //         //         x.name.clone().unwrap_or_default(),
    //         //         x.default_price.clone().unwrap_or_default().into_object()
    //         //     );
    //         // }

    //         Ok(Json::from(
    //             serde_json::json!({"code": StatusCode::RESET_CONTENT.as_str() }),
    //         ))
    //     }
    //     Err(err) => {
    //         log::error!("{err:#?}");
    //         Err(ErrorResponse::from(err.to_string()))
    //     }
    // }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct StripeData {
    products: Vec<DbProduct>,
    customers: Vec<DbCustomer>,
}

impl StripeData {
    pub fn new(products: List<Product>, customers: List<Customer>) -> Self {
        StripeData {
            products: products.data.into_iter().map(|x| x.into()).collect(),
            customers: customers.data.into_iter().map(|x| x.into()).collect(),
        }
    }
    pub fn update_products(self, products: List<Product>) -> Self {
        StripeData {
            products: products.data.into_iter().map(|x| x.into()).collect(),
            customers: self.customers,
        }
    }
    pub fn update_customers(self, customers: List<Customer>) -> Self {
        StripeData {
            products: self.products,
            customers: customers.data.into_iter().map(|x| x.into()).collect(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DbProduct {
    pub id: ProductId,
    pub active: bool,
    // Measured in seconds since the Unix epoch.
    pub created: Option<Timestamp>,
    pub default_price: Option<Expandable<Price>>,
    pub description: Option<String>,
    // A list of up to 8 URLs of images for this product, meant to be displayable to the customer
    pub images: Option<Vec<String>>,
    pub metadata: Option<Metadata>,
    pub name: String,
    // pub package_dimensions: Option<PackageDimensions>,
    // TLDR AMOUNT OF PRODUCT INCLUDEDI
    // A label that represents units of this product.
    // When set, this will be included in customers' receipts, invoices, Checkout, and the customer portal.
    pub unit_label: Option<String>,
    pub updated: Option<Timestamp>,
    // url of this product
    pub url: Option<String>,
}

impl From<Product> for DbProduct {
    fn from(value: Product) -> Self {
        DbProduct {
            id: value.id,
            active: value.active.unwrap_or(false),
            created: value.created,
            default_price: match value.default_price {
                Some(x) => Some(x),
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DbPrice {
    pub id: PriceId,
    pub active: bool,
    pub billing_scheme: Option<PriceBillingScheme>,
    pub created: Option<Timestamp>,
    pub currency: Option<Currency>,
    pub currency_options: Option<HashMap<Currency, CurrencyOption>>,
    pub custom_unit_amount: Option<CustomUnitAmount>,
    pub livemode: bool,
    pub lookup_key: Option<String>,
    pub metadata: Option<Metadata>,
    pub nickname: Option<String>,
    pub product: Option<DbProduct>,
    pub recurring: Option<Recurring>,
    pub tiers: Option<Vec<PriceTier>>,
    pub tiers_mode: Option<PriceTiersMode>,
    pub transform_quantity: Option<TransformQuantity>,
    pub type_: Option<PriceType>,
    pub unit_amount: Option<i64>,
    pub unit_amount_decimal: Option<String>,
}

impl From<Price> for DbPrice {
    fn from(value: Price) -> Self {
        DbPrice {
            id: value.id,
            active: value.active.unwrap_or(false),
            billing_scheme: value.billing_scheme,
            created: value.created,
            currency: value.currency,
            currency_options: value.currency_options,
            custom_unit_amount: value.custom_unit_amount,
            livemode: value.livemode.unwrap_or(false),
            lookup_key: value.lookup_key,
            metadata: value.metadata,
            nickname: value.nickname,
            product: value
                .product
                .unwrap_or_default()
                .into_object()
                .map(|x| DbProduct::from(x)),

            recurring: value.recurring,
            tiers: value.tiers,
            tiers_mode: value.tiers_mode,
            transform_quantity: value.transform_quantity,
            type_: value.type_,
            unit_amount: value.unit_amount,
            unit_amount_decimal: value.unit_amount_decimal,
        }
    }
}

impl Object for DbPrice {
    type Id = PriceId;

    fn id(&self) -> Self::Id {
        self.id.clone().into()
    }

    fn object(&self) -> &'static str {
        "dbprice"
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DbCustomer {
    pub id: CustomerId,
    pub address: Option<Address>,
    pub balance: Option<i64>,
    pub cash_balance: Option<CashBalance>,
    pub created: Option<Timestamp>,
    pub currency: Option<Currency>,
    pub default_source: Option<PaymentSource>,
    pub delinquent: Option<bool>,
    pub description: Option<String>,
    pub discount: Option<Discount>,
    pub email: Option<String>,
    // pub invoice_credit_balance: Option<u64>,
    // pub invoice_prefix: Option<String>,
    // pub invoice_settings: Option<InvoiceSettingCustomerSetting>,
    // pub next_invoice_sequence: Option<i64>,
    pub livemode: bool,
    pub metadata: Option<Metadata>,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub shipping: Option<Shipping>,
    pub sources: List<PaymentSource>,
    // pub subscriptions: Option<List<Subscription>>,
    // pub tax: Option<CustomerTax>,
    // pub tax_exempt: Option<CustomerTaxExempt>,
    // pub tax_ids: Option<List<TaxId>>,
    // pub test_clock: Option<TestHelpersTestClock>,
}

impl From<Customer> for DbCustomer {
    fn from(value: Customer) -> Self {
        DbCustomer {
            id: value.id,
            address: value.address,
            balance: value.balance,
            cash_balance: value.cash_balance,
            created: value.created,
            currency: value.currency,
            default_source: value.default_source.unwrap_or_default().into_object(),
            delinquent: value.delinquent,
            description: value.description,
            discount: value.discount,
            email: value.email,
            livemode: value.livemode.unwrap_or(false),
            metadata: value.metadata,
            name: value.name,
            phone: value.phone,
            shipping: value.shipping,
            sources: value.sources,
        }
    }
}
