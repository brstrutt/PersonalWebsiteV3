use super::DPad;
use leptos::prelude::*;

#[component]
pub fn right_joycon() -> impl IntoView {
    view! {
        <div class="right-joycon">
            <DPad />
        </div>
    }
}
