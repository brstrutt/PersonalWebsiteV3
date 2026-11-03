use crate::app::components::DPad;
use leptos::prelude::*;

#[component]
pub fn left_joycon() -> impl IntoView {
    view! {
        <div class="left-joycon">
            <DPad />
        </div>
    }
}
