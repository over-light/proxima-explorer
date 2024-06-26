use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn Welcome() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <Box class="flex flex-1 flex-col items-center p-4">
            <H2 class="text-3xl">"Proxima Explorer"</H2>
            <span class="mt-12">"Count: " {move || count.get()}</span>
            <Button
                on_click=move |_| set_count.update(|c| *c += 1)
                class="mt-4 p-2 bg-blue-500 text-white rounded"
            >
                "Increase"
            </Button>
        </Box>
    }
}
