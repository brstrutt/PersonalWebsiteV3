use const_format::concatcp;
use leptos::prelude::*;
use leptos_router::{
    components::{Outlet, ParentRoute},
    path, MatchNestedRoutes, StaticSegment,
};

pub mod control_dark_souls_with_a_piano;
pub mod embed_presentation_in_hugo;
pub mod ridge_racer_ds_unlock_burning_nightmare;

pub const URL_FRAGMENT: (StaticSegment<&str>,) = path!("/guides");
pub const PAGE_PATH: &str = concatcp!(super::PAGE_PATH, "/", URL_FRAGMENT.0 .0);

#[component(transparent)]
pub fn Routes() -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=URL_FRAGMENT view=Outlet>
            <control_dark_souls_with_a_piano::Routes />
            <embed_presentation_in_hugo::Routes />
            <ridge_racer_ds_unlock_burning_nightmare::Routes />
        </ParentRoute>
    }
    .into_inner()
}
