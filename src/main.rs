#![allow(unused)]
#[cfg(not(feature = "ssr"))]
fn main() {}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::routing::post;
    use axum::*;
    use core::panic;
    use farmtasker_au::app::*;
    use farmtasker_au::fileserv::file_and_error_handler;
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use std::borrow::BorrowMut;
    use std::io::{BufRead, BufReader};
    use std::sync::{Arc, Mutex};
    use stripe::*;
    use tracing::*;
    use tracing_subscriber;

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
        stripe_data: farmtasker_au::sync::StripeData::new_fetch()
            .await
            .expect("Could not fetch data from stripe api"),
    };

    appstate.id = 5;
    let products = &appstate.stripe_data.products;
    for i in products {
        tracing::info!(
            "Product: {:#?} - {:#?}$ AUD",
            i.name,
            i.default_price
                .clone()
                .unwrap()
                .as_object()
                .unwrap()
                .unit_amount
                .unwrap() as f64
                / 100.0
        );
    }
    tracing::info!("Total Products: {:}", products.len());

    // build our application with a route
    let app = Router::new()
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            {
                let appstate = appstate.clone();
                move || provide_context(appstate.clone())
            },
            App,
        )
        // .leptos_routes(&leptos_options, routes, App)
        .fallback(file_and_error_handler)
        .with_state(leptos_options)
        .route("/api/sync", post(farmtasker_au::sync::stripe_sync))
        .layer(Extension(appstate));

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!("listening on http://{}\n", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
