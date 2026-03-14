use core::str;
use leptos::prelude::*;
use leptos_router::components::{Router, Routes};
use crate::app::components::{Footer, Header};

mod utils;
mod components;
mod pages;

const URL_SUFFIX: &str = "/PersonalWebsiteV3";

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router base=URL_SUFFIX>
            <Header />
            <Routes fallback=|| "404 Not found!">
                <pages::Routes />
            </Routes>
            <Footer />
        </Router>
    }
}
