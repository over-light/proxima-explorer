use crate::components::nav_link::NavLink;
use crate::utils::constants::{APP_BAR_HEIGHT, BOTTOM_BAR_HEIGHT};
use leptonic::prelude::*;
use leptos::*;
use leptos_use::use_media_query;

#[component]
pub fn Header() -> impl IntoView {
    let (drawer_show, set_drawer_show) = create_signal(false);
    let is_small = use_media_query("(min-width: 800px)");

    view! {
        <>
            <AppBar height=APP_BAR_HEIGHT class="z-10">
                <Link href="/" class="flex items-center p-3">
                    <img src="/favicon.ico" class="w-9 h-9 rounded-full" alt="favicon"/>
                    <H3 class="mx-2 font-bold text-xl">"Proxima"</H3>
                </Link>
                <Stack orientation=StackOrientation::Horizontal spacing=Size::Em(1.0) class="mr-4">
                    {move || match is_small.get() {
                        false => {
                            view! {
                                <Button
                                    variant=ButtonVariant::Flat
                                    size=ButtonSize::Big
                                    on_click=move |_| {
                                        set_drawer_show.update(|shown| *shown = !*shown)
                                    }
                                >
                                    <Icon icon=icondata::BsList class="text-xl"/>
                                </Button>
                            }
                                .into_view()
                        }
                        true => {
                            view! {
                                <Stack
                                    orientation=StackOrientation::Horizontal
                                    spacing=Size::Em(1.0)
                                    class="mr-4"
                                >
                                    <NavLink link="/" icon=icondata::BsHouse text="Home"/>
                                    <NavLink
                                        link="/blockchain"
                                        icon=icondata::BsBoxes
                                        text="Blockchain"
                                    />
                                    <NavLink
                                        link="/tokens"
                                        icon=icondata::BiCoinStackSolid
                                        text="Tokens"
                                    />
                                    <NavLink link="/other" icon=icondata::BsCardList text="Other"/>
                                </Stack>
                            }
                                .into_view()
                        }
                    }}

                    <ThemeToggle off=LeptonicTheme::Light on=LeptonicTheme::Dark variant=ToggleVariant::Stationary/>
                </Stack>
            </AppBar>
            <Drawer
                side=DrawerSide::Left
                shown=drawer_show
                class="absolute flex flex-col z-10 w-[200px] overflow-y-auto px-3 py-10"
                style=format!("top: {APP_BAR_HEIGHT}; bottom: {BOTTOM_BAR_HEIGHT}")
            >
                <Stack spacing=Size::Em(0.5)>
                    <NavLink link="/" icon=icondata::BsHouse text="Home" attr:class="w-full"/>
                    <NavLink
                        link="/blockchain"
                        icon=icondata::BsBoxes
                        text="Blockchain"
                        attr:class="w-full"
                    />
                    <NavLink
                        link="/tokens"
                        icon=icondata::BiCoinStackSolid
                        text="Tokens"
                        attr:class="w-full"
                    />
                    <NavLink
                        link="/other"
                        icon=icondata::BsCardList
                        text="Other"
                        attr:class="w-full"
                    />
                </Stack>
            </Drawer>
        </>
    }
}
