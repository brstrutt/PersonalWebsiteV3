use const_format::concatcp;
use leptos::prelude::*;
use leptos_router::{components::Route, path, MatchNestedRoutes, StaticSegment};

use crate::app::components::{world_display::controls::input_action_keys::*, TouchscreenButton};

pub const URL_FRAGMENT: (StaticSegment<&str>,) = path!("/explore");
pub const PAGE_PATH: &str = concatcp!(super::PAGE_PATH, "/", URL_FRAGMENT.0 .0);

#[component(transparent)]
pub fn Routes() -> impl MatchNestedRoutes + Clone {
    view! { <Route path=URL_FRAGMENT view=Page /> }.into_inner()
}

#[component]
fn Page() -> impl IntoView {
    view! {
        <main>
            <div class="screen_controls">
                <TouchscreenButton data_action=DATA_ACTION_MOVE_LEFT>"◄"</TouchscreenButton>
                <div class="vertical_movement_buttons">
                    <TouchscreenButton data_action=DATA_ACTION_MOVE_FORWARD>
                        "▲"
                    </TouchscreenButton>
                    <TouchscreenButton data_action=DATA_ACTION_MOVE_BACKWARD>
                        "▼"
                    </TouchscreenButton>
                </div>
                <TouchscreenButton data_action=DATA_ACTION_MOVE_RIGHT>"►"</TouchscreenButton>
            </div>
        </main>
    }
}
