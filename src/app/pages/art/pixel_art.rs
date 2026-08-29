use const_format::concatcp;
use leptos::prelude::*;

pub mod nokia_art_jam;
pub mod other;
pub mod vermintide;

use leptos_router::{
    components::{Outlet, ParentRoute},
    path, MatchNestedRoutes, StaticSegment,
};

pub const URL_FRAGMENT: (StaticSegment<&str>,) = path!("/pixel-art");
pub const PAGE_PATH: &str = concatcp!(super::PAGE_PATH, "/", URL_FRAGMENT.0 .0);

#[component(transparent)]
pub fn Routes() -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=URL_FRAGMENT view=Outlet>
            <nokia_art_jam::Routes />
            <vermintide::Routes />
            <other::Routes />
        </ParentRoute>
    }
    .into_inner()
}
