use crate::app::components::PageWrapper;
use const_format::concatcp;
use leptos::prelude::*;
use leptos_router::{components::ParentRoute, path, MatchNestedRoutes, StaticSegment};

pub mod bolognese;
pub mod brownies;
pub mod burger;
pub mod cheese_sauce;
pub mod chicken_kiev;
pub mod chicken_pie;
pub mod hot_chocolate;
pub mod hunters_chicken;
pub mod marble_cake;
pub mod meatballs;
pub mod puff_pastry;

pub const URL_FRAGMENT: (StaticSegment<&str>,) = path!("/recipes");
pub const PAGE_PATH: &str = concatcp!(super::PAGE_PATH, "/", URL_FRAGMENT.0 .0);

#[component(transparent)]
pub fn Routes() -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=URL_FRAGMENT view=PageWrapper>
            <bolognese::Routes />
            <brownies::Routes />
            <burger::Routes />
            <cheese_sauce::Routes />
            <chicken_kiev::Routes />
            <chicken_pie::Routes />
            <hot_chocolate::Routes />
            <hunters_chicken::Routes />
            <marble_cake::Routes />
            <meatballs::Routes />
            <puff_pastry::Routes />
        </ParentRoute>
    }
    .into_inner()
}
