use crate::app::components::PageWrapper;
use leptos::prelude::*;
use leptos_router::{components::ParentRoute, path, MatchNestedRoutes};

pub mod pixel_art;

#[component(transparent)]
pub fn Routes() -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("/art") view=PageWrapper>
            <pixel_art::Routes />
        </ParentRoute>
    }
    .into_inner()
}
