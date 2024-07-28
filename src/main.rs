#![allow(unused)]
use axum::Router;
use farmtasker_au::app::*;
use farmtasker_au::fileserv::file_and_error_handler;
use farmtasker_au::*;
use leptos::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use std::sync::Arc;
use stripe::*;

pub mod sync;
use sync::*;

use tracing::*;
use tracing_subscriber;
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    info!("This Runs...");

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    // let appdata = GlobalData::default();
    // let appdata = Arc::new(appdata);

    let key = std::env::var("REMOVED").unwrap();

    let _stripe_client = Client::new(key);

    // build our application with a route
    let app = Router::new()
        .leptos_routes(&leptos_options, routes, App)
        .fallback(file_and_error_handler)
        .with_state(leptos_options);
    // .with_state(&appdata.clone());

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    logging::log!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
