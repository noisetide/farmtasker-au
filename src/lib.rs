pub mod app;
pub mod error_template;
#[cfg(feature = "ssr")]
pub mod fileserv;
#[cfg(feature = "ssr")]
pub mod sync;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    log::info!("Hello!");
    leptos::mount_to_body(App);
}

#[cfg(feature = "ssr")]
#[derive(Default, serde::Serialize, serde::Deserialize, Clone, Copy, Debug)]
pub struct AppState {
    pub id: u64,
}
