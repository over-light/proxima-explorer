use leptos::*;
// use std::error::Error as StdError;

#[component]
pub fn Error(#[prop(optional)] errors: dyn leptos::SignalGet) -> impl IntoView {
    view! {
        <h1>"Uh oh! Something went wrong!"</h1>
        <p>"Errors: "</p>
        <ul>
            {move || {
                if let Some(errors) = &errors {
                    errors.get()
                        .iter()
                        .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                        .collect_view()
                } else {
                    view! { <li>"No errors provided."</li> }
                }
            }}
        </ul>
    }
}
