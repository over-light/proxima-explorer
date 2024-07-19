use leptonic::prelude::*;
use leptos::*;
use leptos_use::{use_color_mode, UseColorModeReturn};

#[component]
pub fn ColorContext(children: Children) -> impl IntoView {
    let UseColorModeReturn { mode, .. } = use_color_mode();

    provide_context(mode);

    view! { <>{children()}</> }
}
