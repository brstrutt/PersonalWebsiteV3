use leptos::prelude::*;

pub mod nokia_art_jam;
pub mod other;
pub mod vermintide;

use leptos_router::{
    components::{Outlet, ParentRoute},
    path, MatchNestedRoutes,
};

#[component(transparent)]
pub fn Routes() -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("/pixel-art") view=Outlet>
            <nokia_art_jam::Routes />
            <vermintide::Routes />
            <other::Routes />
        </ParentRoute>
    }
    .into_inner()
}
