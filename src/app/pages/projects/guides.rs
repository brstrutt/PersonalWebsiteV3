use leptos::prelude::*;
use leptos_router::{MatchNestedRoutes, components::Route, path};

#[component(transparent)]
pub fn Routes() -> impl MatchNestedRoutes + Clone {
    view! { <Route path=path!("/guides") view=Page /> }
    .into_inner()
}

#[component]
fn Page() -> impl IntoView {
    view! { <h1>"Guides!"</h1> }
}