use leptos::prelude::*;
use leptos_router::{components::Route, path, MatchNestedRoutes};

#[component(transparent)]
pub fn Routes() -> impl MatchNestedRoutes + Clone {
    view! { <Route path=path!("/ridge-racer-ds-unlock-burning-nightmare") view=Page /> }
        .into_inner()
}

#[component]
fn Page() -> impl IntoView {
    view! { <div inner_html=markdown::to_html(include_str!("ridge_racer_ds_unlock_burning_nightmare.md")) /> }
}
