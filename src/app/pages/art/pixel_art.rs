use leptos::prelude::*;

pub mod nokia_art_jam;
pub mod other;
pub mod vermintide;

use leptos_router::{MatchNestedRoutes, components::{A, Outlet, ParentRoute, Route}, path};
use crate::app::utils::internal_path;

#[component(transparent)]
pub fn Routes() -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("/pixel-art") view=Outlet>
            <Route path=path!("/") view=Page />
            <nokia_art_jam::Routes />
            <vermintide::Routes />
            <other::Routes />
        </ParentRoute>
    }
    .into_inner()
}

#[component]
fn Page() -> impl IntoView {
    view! {
        <h1>"Pixel Art!"</h1>
        <A href=internal_path("/art/pixel-art/nokia-art-jam-3")>"Nokia art jam 3"</A>
        <br />
        <A href=internal_path("/art/pixel-art/vermintide")>"Warhammer Vermintide 2"</A>
        <br />
        <A href=internal_path("/art/pixel-art/other")>"Other"</A>
    }
}
