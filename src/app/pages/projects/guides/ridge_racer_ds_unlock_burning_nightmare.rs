use leptos::prelude::*;
use leptos_router::{MatchNestedRoutes, components::Route, path};

#[component(transparent)]
pub fn Routes() -> impl MatchNestedRoutes + Clone {
    view! { <Route path=path!("/ridge-racer-ds-unlock-burning-nightmare") view=Page /> }
    .into_inner()
}

#[component]
fn Page() -> impl IntoView {
    view! { <h1>"How to unlock burning nightmare in ridge racer DS!"</h1> }
}
