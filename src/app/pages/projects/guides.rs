use leptos::prelude::*;
use leptos_router::{
    components::{Outlet, ParentRoute, Route, A},
    path, MatchNestedRoutes,
};

use crate::app::utils::internal_path;

mod ridge_racer_ds_unlock_burning_nightmare;

#[component(transparent)]
pub fn Routes() -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("/guides") view=Outlet>
            <Route path=path!("/") view=Page />
            <ridge_racer_ds_unlock_burning_nightmare::Routes />
        </ParentRoute>
    }
    .into_inner()
}

#[component]
fn Page() -> impl IntoView {
    view! {
        <h1>"Guides!"</h1>
        <A href=internal_path(
            "/projects/guides/ridge-racer-ds-unlock-burning-nightmare",
        )>"How to unlock Burning Nightmare in Ridge Racer DS"</A>
        <br />
    }
}
