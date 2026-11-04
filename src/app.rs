use crate::app::components::{Footer, LeftJoycon, Overlay, RightJoycon, WorldDisplay};
use core::str;
use leptos::prelude::*;
use leptos_router::components::{Router, Routes};

mod components;
mod pages;
mod utils;

const URL_SUFFIX: &str = "/PersonalWebsiteV3";

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router base=URL_SUFFIX>
            <div class="game_ui">
                <div class="game_screen">
                    <WorldDisplay />
                </div>
                <div class="joycons">
                    <LeftJoycon />
                    <div class="spacer" />
                    <RightJoycon />
                </div>
            </div>
            <Overlay>
                <Routes fallback=|| "404 Not found!" transition=true>
                    <pages::Routes />
                </Routes>
                <Footer />
            </Overlay>
        </Router>
    }
}
