use std::{cell::RefCell, rc::Rc};

use leptos::prelude::*;
use wasmfenbein3d::state::State;

#[component]
pub fn camera_slider(
    game_state: ReadSignal<Option<Rc<RefCell<State>>>, LocalStorage>,
) -> impl IntoView {
    view! { <div class="camera_slider" /> }
}
