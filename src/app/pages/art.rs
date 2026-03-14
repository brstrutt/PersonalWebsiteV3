use leptos::prelude::*;
use leptos_router::{MatchNestedRoutes, components::{Outlet, ParentRoute, Route}, path};
use crate::app::components::LeftNavigationBar;

pub mod maps;
pub mod pixel_art;
pub mod warhammer;

#[component(transparent)]
pub fn Routes() -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("/art") view=Wrapper>
            <Route path=path!("/") view=Page />
            <maps::Routes />
            <warhammer::Routes />
            <pixel_art::Routes />
        </ParentRoute>
    }
    .into_inner()
}

#[component]
fn Wrapper() -> impl IntoView {
    view! {
        <LeftNavigationBar />
        <main>
            <Outlet />
        </main>
    }
}

#[component]
fn Page() -> impl IntoView {
    view! { <h1>"Art!"</h1> }
}
