#![allow(unused)]
#[cfg(not(feature = "ssr"))]
fn main() {}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    pub use axum::routing::post;
    pub use axum::*;
    pub use core::panic;
    pub use farmtasker_au::app::*;
    pub use farmtasker_au::fileserv::file_and_error_handler;
    use farmtasker_au::StripeData;
    pub use leptos::*;
    pub use leptos_axum::{generate_route_list, LeptosRoutes};
    pub use std::borrow::BorrowMut;
    pub use std::io::{BufRead, BufReader};
    pub use std::sync::{Arc, Mutex};
    pub use stripe::*;
    pub use tracing::*;
    pub use tracing_subscriber;

    tracing_subscriber::fmt::init();

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let key = std::env::var("REMOVED").expect("couldn't get env var REMOVED");
    let stripe_client = stripe::Client::new(key.clone());

    let mut appstate = farmtasker_au::AppState {
        id: 0,
        stripe_api_key: key.to_string(),
        stripe_data: farmtasker_au::StripeData::new_fetch()
            .await
            .expect("Could not fetch data from stripe api"),
    };

    appstate.id = 5;
    let products = &appstate.stripe_data.products;
    for i in products {
        tracing::info!(
            "Product: {:#?} - {:#?}$ AUD",
            i.name,
            i.default_price.clone().unwrap().unit_amount.unwrap() as f64 / 100.0
        );
    }
    tracing::info!("Total Products: {:}", products.len());

    let stripedata = StripeData::new_fetch().await.unwrap();

    // build our application with a route
    let app = Router::new()
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            {
                let appstate = appstate.clone();
                move || provide_context(stripedata.clone())
            },
            App,
        )
        // .leptos_routes(&leptos_options, routes, App)
        .fallback(file_and_error_handler)
        .with_state(leptos_options)
        .layer(Extension(appstate))
        .route("/api/sync", post(farmtasker_au::sync::stripe_sync));

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!("listening on http://{}\n", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
