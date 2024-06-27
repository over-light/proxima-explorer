use crate::utils::constants::BOTTOM_BAR_HEIGHT;
use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <AppBar height=BOTTOM_BAR_HEIGHT class="z-5 p-2">
            <div class="flex items-center">
                <H3 class="mx-2">"Proxima Explorer @2024"</H3>
            </div>
            <Stack orientation=StackOrientation::Horizontal spacing=Size::Em(1.0) class="mr-4">
                <LinkExt href="#" target=LinkExtTarget::Parent>
                    <Icon icon=icondata::BsDiscord class="text-2xl"/>
                </LinkExt>
                <LinkExt href="#" target=LinkExtTarget::Parent>
                    <Icon icon=icondata::BsTelegram class="text-2xl"/>
                </LinkExt>
                <LinkExt href="#" target=LinkExtTarget::Parent>
                    <Icon icon=icondata::BsGithub class="text-2xl"/>
                </LinkExt>
                <LinkExt href="#" target=LinkExtTarget::Parent>
                    <Icon icon=icondata::BsTwitter class="text-2xl"/>
                </LinkExt>
            </Stack>
        </AppBar>
    }
}
