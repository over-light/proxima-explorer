use crate::components::counter_btn::Button;
use crate::components::error_component::Error;
use leptos::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=move |cx, errors| {
            let error_messages = errors.iter().map(|e| e.to_string()).collect::<Vec<_>>();
            println!("{?}", error_components);
            view! {
                <Error errors=error_messages/>
            }
        }>

            <div class="container">

                <picture>
                    <source
                        srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_pref_dark_RGB.svg"
                        media="(prefers-color-scheme: dark)"
                    />
                    <img
                        src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg"
                        alt="Leptos Logo"
                        height="200"
                        width="400"
                    />
                </picture>

                <h1>"Welcome to Leptos"</h1>

                <div class="buttons">
                    <Button/>
                    <Button increment=5/>
                </div>

            </div>
        </ErrorBoundary>
    }
}
