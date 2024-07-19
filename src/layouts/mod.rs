use footer::Footer;
use header::Header;
use leptonic::prelude::*;
use leptos::*;
use crate::contexts::color_context::*;

pub mod footer;
pub mod header;

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    view! {
        <ColorContext>
        <Box class="flex flex-col h-full">
            <Header/>
            {children()}
            <Footer/>
        </Box>
        </ColorContext>
    }
}
