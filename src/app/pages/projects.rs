use crate::app::components::PageWrapper;
use const_format::concatcp;
use leptos::prelude::*;
use leptos_router::{components::ParentRoute, path, MatchNestedRoutes, StaticSegment};

pub mod guides;

pub const URL_FRAGMENT: (StaticSegment<&str>,) = path!("/projects");
pub const PAGE_PATH: &str = concatcp!(super::PAGE_PATH, "/", URL_FRAGMENT.0 .0);

#[component(transparent)]
pub fn Routes() -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=URL_FRAGMENT view=PageWrapper>
            <guides::Routes />
        </ParentRoute>
    }
    .into_inner()
}
