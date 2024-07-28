#![allow(unused)]
#[cfg(feature = "ssr")]
use http::StatusCode;
use log::*;
use std::collections::HashMap;
use std::sync::Arc;
use stripe::*;

use farmtasker_au::*;

use axum::{
    extract::{FromRequest, State},
    response::*,
    Extension, Json,
};

pub async fn stripe_sync(
    // Extension(shared_state): Extension<Arc<AppState>>,
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
                error!("{:#?}", err);
                return Err(ErrorResponse::from(Json::from(err.to_string())));
            }
        };

    match list_of_customers_from_stripe_api.data.len() {
        0 => {
            info!("No Customers");
        }
        x if x > 0 => {
            info!("Customers#: {:?}", x);
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
                error!("{:#?}", err);
                return Err(ErrorResponse::from(Json::from(err.to_string())));
            }
        };

    let listed_price = list_of_products_from_stripe_api.data[0]
        .default_price
        .clone()
        .unwrap()
        .into_object()
        .unwrap();

    info!("First api default price: {:?}", &listed_price);

    let saver: DbPrice = DbPrice::from(listed_price);

    type Saver = DbPrice;

    // shared_state.persist.save::<Saver>("price", saver).unwrap();

    // let loaded_price = shared_state.persist.load::<Saver>("price").unwrap();

    // info!("First load default price: {:#?}", loaded_price);

    for i in list_of_products_from_stripe_api.data.iter_mut() {
        i.default_price = None;
    }

    let data = GlobalData::new(
        list_of_products_from_stripe_api.clone(),
        list_of_customers_from_stripe_api.clone(),
    );

    // match shared_state.persist.save::<GlobalData>("data", data) {
    //     Ok(_) => {
    //         let list_data = shared_state.persist.list().expect("Could not load data!");

    //         info!("Saved Data: {:#?}", list_data);

    //         // print out 10 products from stripe api
    //         // for (n, x) in list_of_products_from_stripe_api.data.iter().enumerate() {
    //         //     if n >= 10 {
    //         //         break;
    //         //     }
    //         //     info!(
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
    //         error!("{err:#?}");
    //         Err(ErrorResponse::from(err.to_string()))
    //     }
    // }
}
