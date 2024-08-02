#![allow(unused)]
use axum::*;
use farmtasker_au::app::*;
use farmtasker_au::fileserv::file_and_error_handler;
use farmtasker_au::sync::*;
use farmtasker_au::*;
use leptos::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use std::sync::{Arc, Mutex};
use stripe::*;
use tracing::*;
use tracing_subscriber;

#[tokio::main]
async fn main() {
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
    let stripe_client = stripe::Client::new(key);

    let mut appstate = Arc::new(Mutex::new(AppState {
        id: 0,
        stripe_data: crate::sync::StripeData::new_fetch(&stripe_client)
            .await
            .expect("Could not fetch data from stripe api"),
    }));

    appstate.lock().unwrap().id = 5;

    info!("This Runs... {:?}", &appstate);

    sync::stripe_sync(Extension(appstate.clone()), stripe_client);

    // build our application with a route
    let app = Router::new()
        .leptos_routes(&leptos_options, routes, App)
        .fallback(file_and_error_handler)
        .with_state(leptos_options)
        .layer(Extension(appstate.clone()));

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    logging::log!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
