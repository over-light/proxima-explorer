use leptos::*;

#[component]
pub fn Error(cx: Scope, errors: Vec<String>) -> impl IntoView {
    let errors = cx.use_signal(Vec::new);
    view! { cx,
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
