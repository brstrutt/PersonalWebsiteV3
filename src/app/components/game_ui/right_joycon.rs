use std::{cell::RefCell, rc::Rc};

use super::AButton;
use leptos::prelude::*;
use wasmfenbein3d::state::State;

#[component]
pub fn right_joycon(
    game_state: ReadSignal<Option<Rc<RefCell<State>>>, LocalStorage>,
) -> impl IntoView {
    view! {
        <div class="right-joycon">
            <AButton game_state=game_state />
        </div>
    }
}
