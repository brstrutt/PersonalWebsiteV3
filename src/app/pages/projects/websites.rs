use leptos::prelude::*;
use leptos_router::{components::Route, path, MatchNestedRoutes};

#[component(transparent)]
pub fn Routes() -> impl MatchNestedRoutes + Clone {
    view! { <Route path=path!("/websites") view=Page /> }.into_inner()
}

#[component]
fn Page() -> impl IntoView {
    view! { <h1>"Websites!"</h1> }
}
