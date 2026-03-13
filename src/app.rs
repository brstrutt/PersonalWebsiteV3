use leptos::prelude::*;
use leptos_router::{components::{Route, Router, Routes}, path};
mod components;
mod pages;
use crate::app::components::NavigationBar;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <NavigationBar />
            <main>
                <Routes fallback=|| "404 Not found!">
                    <Route path=path!("/") view=pages::Home />
                    <Route path=path!("/other-page") view=pages::Other />
                </Routes>
            </main>
        </Router>
    }
}
