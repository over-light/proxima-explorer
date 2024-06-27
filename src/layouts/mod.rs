use footer::Footer;
use header::Header;
use leptonic::prelude::*;
use leptos::*;

pub mod footer;
pub mod header;

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    view! {
        <Box class="flex flex-col h-full">
            <Header/>
            {children()}
            <Footer/>
        </Box>
    }
}
