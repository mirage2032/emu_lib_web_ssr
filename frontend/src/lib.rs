use app::*;
use leptos::*;
use leptos::config::LeptosOptions;
use leptos::prelude::*;
use leptos_meta::MetaTags;
use wasm_bindgen::prelude::wasm_bindgen;
#[wasm_bindgen]
pub fn hydrate() {
    // initializes logging using the `log` crate
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    hydrate_islands();
}
