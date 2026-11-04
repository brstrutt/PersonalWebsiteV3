use crate::app::components::{LeftJoycon, RightJoycon, WorldDisplay};
use leptos::prelude::*;

#[component]
pub fn game_ui() -> impl IntoView {
    view! {
        <div class="game_ui">
            <div class="game_screen">
                <WorldDisplay />
            </div>
            <div class="joycons">
                <LeftJoycon />
                <div class="spacer" />
                <RightJoycon />
            </div>
        </div>
    }
}
