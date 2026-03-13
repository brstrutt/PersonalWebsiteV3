use leptos::prelude::*;
use leptos_router::{components::{Route, Router, Routes}, path};
mod components;
mod pages;
use crate::app::components::{NavigationBar, Footer};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <NavigationBar />
            <main>
                <Routes fallback=|| "404 Not found!">
                    <Route path=path!("/") view=pages::Home />
                    <Route path=path!("/art") view=pages::Art />
                    <Route path=path!("/projects") view=pages::Projects />
                    <Route path=path!("/other-page") view=pages::Other />
                </Routes>
            </main>
            <Footer/>
        </Router>
    }
}
