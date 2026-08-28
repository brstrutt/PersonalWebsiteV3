use crate::app::components::PageWrapper;
use leptos::prelude::*;
use leptos_router::{components::ParentRoute, path, MatchNestedRoutes};

pub mod guides;

#[component(transparent)]
pub fn Routes() -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("/projects") view=PageWrapper>
            <guides::Routes />
        </ParentRoute>
    }
    .into_inner()
}
