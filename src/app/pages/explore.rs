use leptos::prelude::*;
use leptos_router::{
    MatchNestedRoutes, components::Route, path
};

use crate::app::components::{
    world_display::controls::input_action_keys::*,
    TouchscreenButton,
};

#[component(transparent)]
pub fn Routes() -> impl MatchNestedRoutes + Clone {
    view! { <Route path=path!("/explore") view=Page /> }
    .into_inner()
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
