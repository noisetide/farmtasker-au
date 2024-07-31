#![allow(unused)]
use axum::Router;
use axum::*;
use farmtasker_au::app::*;
use farmtasker_au::fileserv::file_and_error_handler;
use farmtasker_au::*;
use leptos::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use stripe::*;

pub mod sync;
use sync::*;

#[derive(Default, Serialize, Deserialize, Clone, Copy, Debug)]
pub struct AppState {
    id: i32,
}

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

    let mut appstate = Arc::new(AppState { id: 54 });
    info!("This Runs... {:#?}", &appstate);

    let key = std::env::var("REMOVED").unwrap();

    let stripe_client = Client::new(key);

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
