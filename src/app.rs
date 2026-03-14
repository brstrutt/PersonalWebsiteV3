use core::str;
use leptos::prelude::*;
use leptos_router::{
    components::{Outlet, ParentRoute, Route, Router, Routes},
    path,
};
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
                <Route path=path!("/") view=pages::Home />
                <ParentRoute path=path!("/art") view=pages::art::Wrapper>
                    <Route path=path!("/") view=pages::Art />
                    <Route path=path!("/maps") view=pages::art::Maps />
                    <Route path=path!("/warhammer") view=pages::art::Warhammer />
                    <ParentRoute path=path!("/pixel-art") view=Outlet>
                        <Route path=path!("/") view=pages::art::PixelArt />
                        <Route
                            path=path!("/nokia-art-jam-3")
                            view=pages::art::pixel_art::NokiaArtJam
                        />
                        <Route
                            path=path!("/vermintide")
                            view=pages::art::pixel_art::Vermintide
                        />
                    </ParentRoute>
                </ParentRoute>
                <ParentRoute path=path!("/projects") view=pages::projects::Wrapper>
                    <Route path=path!("/") view=pages::Projects />
                    <Route path=path!("/guides") view=pages::projects::Guides />
                    <Route path=path!("/websites") view=pages::projects::Websites />
                </ParentRoute>
                <Route path=path!("/other-page") view=pages::Other />
            </Routes>
            <Footer />
        </Router>
    }
}
