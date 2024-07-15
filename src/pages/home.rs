use leptonic::prelude::*;
use leptos::*;
use crate::contexts::color_context::ColorContext;

#[component]
pub fn Home() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <Box class="flex flex-1 flex-col h-full justify-center items-center">
            <>
                <H2 class="text-3xl">""</H2>
                <ColorContext/>
                <span>"Count: " {move || count.get()}</span>
                <Button on_click=move |_| set_count.update(|c| *c += 1) class="mt-4">
                    "Increase"
                </Button>
            </>
        </Box>
    }
}
