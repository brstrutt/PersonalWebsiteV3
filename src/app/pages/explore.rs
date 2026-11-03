use const_format::concatcp;
use leptos::prelude::*;
use leptos_router::{components::Route, path, MatchNestedRoutes, StaticSegment};

pub const URL_FRAGMENT: (StaticSegment<&str>,) = path!("/explore");
pub const PAGE_PATH: &str = concatcp!(super::PAGE_PATH, "/", URL_FRAGMENT.0 .0);

#[component(transparent)]
pub fn Routes() -> impl MatchNestedRoutes + Clone {
    view! { <Route path=URL_FRAGMENT view=Page /> }.into_inner()
}

#[component]
fn Page() -> impl IntoView {
    view! { <main></main> }
}
