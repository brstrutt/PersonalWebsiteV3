use leptos::prelude::*;
use leptos_router::{components::Route, path, MatchNestedRoutes};

#[component(transparent)]
pub fn Routes() -> impl MatchNestedRoutes + Clone {
    view! { <Route path=path!("/maps") view=Page /> }.into_inner()
}

#[component]
fn Page() -> impl IntoView {
    view! { <h1>"Maps!"</h1> }
}
