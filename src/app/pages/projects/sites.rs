use const_format::concatcp;
use leptos::prelude::*;
use leptos_router::{
    components::{Outlet, ParentRoute},
    path, MatchNestedRoutes, StaticSegment,
};

pub mod is_worcester_underwater;
pub mod moon;
pub mod pixel_perfect_website;
pub mod roll_of_the_die;

pub const URL_FRAGMENT: (StaticSegment<&str>,) = path!("/sites");
pub const PAGE_PATH: &str = concatcp!(super::PAGE_PATH, "/", URL_FRAGMENT.0 .0);

#[component(transparent)]
pub fn Routes() -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=URL_FRAGMENT view=Outlet>
            <is_worcester_underwater::Routes />
            <moon::Routes />
            <pixel_perfect_website::Routes />
            <roll_of_the_die::Routes />
        </ParentRoute>
    }
    .into_inner()
}
