use const_format::concatcp;
use leptos::prelude::*;
use leptos_router::{components::Route, path, MatchNestedRoutes, StaticSegment};

use crate::app::utils::internal_path;

pub const URL_FRAGMENT: (StaticSegment<&str>,) = path!("/vermintide");
pub const PAGE_PATH: &str = concatcp!(super::PAGE_PATH, "/", URL_FRAGMENT.0 .0);

#[component(transparent)]
pub fn Routes() -> impl MatchNestedRoutes + Clone {
    view! { <Route path=URL_FRAGMENT view=Page /> }.into_inner()
}

#[component]
fn Page() -> impl IntoView {
    view! {
        <h1>"Vermintide"</h1>
        <p>"I've played perhaps a bit too much Warhammer Vermintide 2."</p>
        <p>"Here is some pixel art inspired by the game."</p>
        <img
            src=internal_path("/assets/images/pages/art/pixel_art/vermintide/the_squad.png")
            title="The Squad"
            alt="Pixel art of the five main characters from Warhammer Vermintide 2"
            style:border="1px solid black"
        />
        <br />
        <img
            src=internal_path("/assets/images/pages/art/pixel_art/vermintide/maps.png")
            title="Maps"
            alt="Pixel art maps of various levels from Warhammer Vermintide 2"
        />
    }
}
