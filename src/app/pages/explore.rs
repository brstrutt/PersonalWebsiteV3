use const_format::concatcp;
use leptos::prelude::*;
use leptos_router::{components::Route, path, MatchNestedRoutes, StaticSegment};
use wasmfenbein3d::wasmcontrols::data_action;

use crate::app::components::TouchscreenButton;

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
                <TouchscreenButton data_action=data_action::MOVE_LEFT>"◄"</TouchscreenButton>
                <div class="vertical_movement_buttons">
                    <TouchscreenButton data_action=data_action::MOVE_FORWARD>
                        "▲"
                    </TouchscreenButton>
                    <TouchscreenButton data_action=data_action::MOVE_BACK>"▼"</TouchscreenButton>
                </div>
                <TouchscreenButton data_action=data_action::MOVE_RIGHT>"►"</TouchscreenButton>
            </div>
        </main>
    }
}
