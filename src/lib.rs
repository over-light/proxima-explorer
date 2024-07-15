use cfg_if::cfg_if;

pub mod app;
pub mod components;
pub mod error_template;
#[cfg(feature = "ssr")]
pub mod fileserv;
pub mod layouts;
pub mod pages;
pub mod utils;
pub mod contexts;

cfg_if! {
    if #[cfg(feature = "hydrate")] {
        use leptos::*;
        use wasm_bindgen::prelude::wasm_bindgen;
        use crate::app::*;

        #[wasm_bindgen]
        pub fn hydrate() {
            console_error_panic_hook::set_once();
            tracing_wasm::set_as_global_default_with_config(
                tracing_wasm::WASMLayerConfigBuilder
                    ::default()
                    .set_max_level(tracing::Level::DEBUG)
                    .build()
            );

            leptos::mount_to_body(App);
        }
    }
}
