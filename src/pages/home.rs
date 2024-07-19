use crate::contexts::color_context::ColorContext;
use leptonic::prelude::*;
use leptos::*;
use leptos_use::ColorMode;

#[component]
pub fn Home() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    
    let mode = use_context::<Signal<ColorMode>>().unwrap();

    view! {
        <Box class="flex flex-1 flex-col h-full justify-center items-center">
            <>
                <H1>{mode()}</H1>
                <H2 class="text-3xl">""</H2>
                <span>"Count: " {move || count.get()}</span>
                <Button on_click=move |_| set_count.update(|c| *c += 1) class="mt-4">
                    "Increase"
                </Button>
            </>
        </Box>
    }
}
