pub mod art;
pub mod home;
pub mod other;
pub mod projects;

use leptos::prelude::*;
use leptos_router::MatchNestedRoutes;

#[component(transparent)]
pub fn Routes() -> impl MatchNestedRoutes + Clone {
    view! {
        <home::Routes />
        <art::Routes />
        <projects::Routes />
        <other::Routes />
    }
    .into_inner()
}
