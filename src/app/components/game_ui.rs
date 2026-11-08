use leptos::prelude::*;

pub mod world_display;
pub use world_display::WorldDisplay;
pub mod touchscreen_button;
use touchscreen_button::TouchscreenButton;
pub mod d_pad;
use d_pad::DPad;
pub mod left_joycon;
pub use left_joycon::LeftJoycon;
pub mod right_joycon;
pub use right_joycon::RightJoycon;
pub mod a_button;
use a_button::AButton;

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
