use leptos::prelude::*;
use leptos_router::{
    MatchNestedRoutes, components::Route, path
};

#[component(transparent)]
pub fn Routes() -> impl MatchNestedRoutes + Clone {
    view! { <Route path=path!("/") view=Page /> }
    .into_inner()
}

#[component]
fn Page() -> impl IntoView {
    view! {
        <main>
            <h1>"HOME!"</h1>
        </main>
    }
}
