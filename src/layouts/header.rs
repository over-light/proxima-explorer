use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <AppBar class="z-10 text-white" style="background: var(--brand-color);">
            <H3 class="ml-4">"Proxima"</H3>
            <Stack orientation=StackOrientation::Horizontal spacing=Size::Em(1.0) class="mr-4">
                <Button on_click=move |_| {} class="text-white">
                    <Icon icon=icondata::BsGithub/>
                </Button>
                <Button on_click=move |_| {} class="text-white">
                    <Icon icon=icondata::BsPower/>
                </Button>
            </Stack>
        </AppBar>
    }
}
