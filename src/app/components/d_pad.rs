use crate::app::components::TouchscreenButton;
use leptos::prelude::*;
use wasmfenbein3d::wasmcontrols::data_action;

#[component]
pub fn d_pad() -> impl IntoView {
    view! {
        <div class="d-pad">
            <div class="row">
                <TouchscreenButton class="up" data_action=data_action::MOVE_FORWARD />
            </div>
            <div class="row">
                <TouchscreenButton class="left" data_action=data_action::MOVE_LEFT />
                <div class="center" />
                <TouchscreenButton class="right" data_action=data_action::MOVE_RIGHT />
            </div>
            <div class="row">
                <TouchscreenButton class="down" data_action=data_action::MOVE_BACK />
            </div>
        </div>
    }
}
