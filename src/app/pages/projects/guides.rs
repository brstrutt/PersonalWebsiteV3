use leptos::prelude::*;
use leptos_router::{
    components::{Outlet, ParentRoute},
    path, MatchNestedRoutes,
};

mod control_dark_souls_with_a_piano;
mod embed_presentation_in_hugo;
mod ridge_racer_ds_unlock_burning_nightmare;

#[component(transparent)]
pub fn Routes() -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("/guides") view=Outlet>
            <control_dark_souls_with_a_piano::Routes />
            <embed_presentation_in_hugo::Routes />
            <ridge_racer_ds_unlock_burning_nightmare::Routes />
        </ParentRoute>
    }
    .into_inner()
}
