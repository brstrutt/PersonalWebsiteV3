use leptos::prelude::*;
use leptos_router::{
    components::{Outlet, ParentRoute, Route, A},
    path, MatchNestedRoutes,
};

use crate::app::utils::internal_path;

mod control_dark_souls_with_a_piano;
mod embed_presentation_in_hugo;
mod ridge_racer_ds_unlock_burning_nightmare;

#[component(transparent)]
pub fn Routes() -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("/guides") view=Outlet>
            <Route path=path!("/") view=Page />
            <control_dark_souls_with_a_piano::Routes />
            <embed_presentation_in_hugo::Routes />
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
        <A href=internal_path(
            "/projects/guides/embed-presentation-in-hugo",
        )>"How to embed a presentation in a Hugo website"</A>
        <br />
        <A href=internal_path(
            "/projects/guides/control-dark-souls-with-piano",
        )>"How to control Dark Souls (and other videogames) with a piano"</A>
    }
}
