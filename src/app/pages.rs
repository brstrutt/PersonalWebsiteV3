pub mod art;
pub mod explore;
pub mod home;
pub mod projects;

use leptos::prelude::*;
use leptos_router::MatchNestedRoutes;

pub const PAGE_PATH: &str = "";

#[component(transparent)]
pub fn Routes() -> impl MatchNestedRoutes + Clone {
    view! {
        <explore::Routes />
        <home::Routes />
        <art::Routes />
        <projects::Routes />
    }
    .into_inner()
}
