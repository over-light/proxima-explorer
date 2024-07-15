use leptonic::prelude::*;
use leptos::*;
use leptos_use::{use_color_mode, UseColorModeReturn};
use serde_json::to_string;

#[component]
pub fn ColorContext() -> impl IntoView {
    let UseColorModeReturn { mode, .. } = use_color_mode();
    view! { <span>{move || mode.get().to_string()}</span> }
}
