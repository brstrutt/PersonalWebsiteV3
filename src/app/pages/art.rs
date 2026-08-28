use leptos::prelude::*;
use leptos_router::{
    components::{Outlet, ParentRoute},
    path, MatchNestedRoutes,
};

pub mod pixel_art;

#[component(transparent)]
pub fn Routes() -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("/art") view=Wrapper>
            <pixel_art::Routes />
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
