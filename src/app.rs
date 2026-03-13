use core::str;

use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};
mod components;
mod pages;
use crate::app::components::{Footer, NavigationBar};

const URL_SUFFIX: &str = "/PersonalWebsiteV3";

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router base=URL_SUFFIX>
            <NavigationBar />
            <main>
                <Routes fallback=|| "404 Not found!">
                    <Route path=path!("/") view=pages::Home />
                    <Route path=path!("/art") view=pages::Art />
                    <Route path=path!("/projects") view=pages::Projects />
                    <Route path=path!("/other-page") view=pages::Other />
                </Routes>
            </main>
            <Footer />
        </Router>
    }
}
