use leptos::prelude::*;
use leptos_router::{
    components::{Outlet, ParentRoute, Route},
    path, MatchNestedRoutes,
};

pub mod guides;
pub mod websites;

#[component(transparent)]
pub fn Routes() -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("/projects") view=Wrapper>
            <Route path=path!("/") view=Page />
            <guides::Routes />
            <websites::Routes />
        </ParentRoute>
    }
    .into_inner()
}

#[component]
fn Wrapper() -> impl IntoView {
    view! {
        <main>
            <Outlet />
        </main>
    }
}

#[component]
fn Page() -> impl IntoView {
    view! { <h1>"Projects!"</h1> }
}
