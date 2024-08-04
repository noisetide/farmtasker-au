#![allow(unused)]
use axum::*;
use core::panic;
use farmtasker_au::app::*;
use farmtasker_au::fileserv::file_and_error_handler;
use farmtasker_au::sync::*;
use farmtasker_au::*;
use leptos::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use std::borrow::BorrowMut;
use std::io::{BufRead, BufReader};
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
    let stripe_client = stripe::Client::new(key.clone());

    let mut appstate = Arc::new(Mutex::new(AppState {
        id: 0,
        stripe_api_key: key.to_string(),
        stripe_data: crate::sync::StripeData::new_fetch(&stripe_client)
            .await
            .expect("Could not fetch data from stripe api"),
    }));

    appstate.lock().unwrap().id = 5;
    let products = &appstate
        .lock()
        .expect("Could not get appstate")
        .stripe_data
        .products;
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
        .leptos_routes(&leptos_options, routes, App)
        .fallback(file_and_error_handler)
        .with_state(leptos_options)
        .layer(Extension(appstate.clone()));

    // Spawn a task to read commands from standard input
    use inquire::{
        validator::{StringValidator, Validation},
        InquireError, Select, Text,
    };
    let server_task = tokio::task::spawn(async move {
        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
        tracing::info!("listening on http://{}\n", &addr);
        axum::serve(listener, app.into_make_service())
            .await
            .unwrap();
    });

    use tokio::io::{self, AsyncBufReadExt, BufReader};

    let input_task = tokio::task::spawn(async {
        let stdin = io::stdin();
        let reader = BufReader::new(stdin);
        let mut lines = reader.lines();

        loop {
            // Use async function to read the next line
            if let Ok(Some(line)) = lines.next_line().await {
                println!("You typed: {}", line);

                // Handle the input here
                // handle_input(line).await;
            }

            // Introduce a delay of 1 second
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    });

    let _ = tokio::join!(server_task, input_task);
}
