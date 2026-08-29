use const_format::concatcp;
use leptos::prelude::*;
use leptos_router::{components::Route, path, MatchNestedRoutes, StaticSegment};

use crate::app::components::MarkdownContent;

pub const URL_FRAGMENT: (StaticSegment<&str>,) = path!("/control-dark-souls-with-piano");
#[allow(dead_code)]
pub const PAGE_PATH: &str = concatcp!(super::PAGE_PATH, "/", URL_FRAGMENT.0 .0);

#[component(transparent)]
pub fn Routes() -> impl MatchNestedRoutes + Clone {
    view! { <Route path=URL_FRAGMENT view=Page /> }.into_inner()
}

#[component]
fn Page() -> impl IntoView {
    view! { <MarkdownContent markdown_text=include_str!("control_dark_souls_with_a_piano.md") /> }
}
