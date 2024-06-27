use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn NavLink(
    #[prop(into)] link: String,
    #[prop(into)] icon: MaybeSignal<icondata::Icon>,
    #[prop(into)] text: String,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {
        <a {..attrs} href=link>
            <div class="flex gap-2">
                <Icon icon=icon class="text-2xl"/>
                <div>{text}</div>
            </div>
        </a>
    }
}
