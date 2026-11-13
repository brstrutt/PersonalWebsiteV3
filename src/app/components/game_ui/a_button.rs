use std::{cell::RefCell, rc::Rc};

use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use wasmfenbein3d::state::State;

use crate::app::components::game_ui::world_display::controls;

#[component]
pub fn a_button(game_state: ReadSignal<Option<Rc<RefCell<State>>>, LocalStorage>) -> impl IntoView {
    let navigate = use_navigate();
    // on:click callback copies some code from the "on click" passthrough function from wasmfenbein3d::wasmcontrols::click_passthrough::setup
    // TODO: Update wasmfenbein3d itself to expose simple functions for triggering on_click and movement via the state object
    view! {
        <button
            class="a-button"
            on:click=move |_| {
                if let Some(game_state) = game_state.get() {
                    let state = game_state.borrow();
                    let item_ids = state.input.get_items_under_cursor(&state);
                    for id in item_ids {
                        controls::on_click(&id, navigate.clone());
                    }
                }
            }
        >
            <div class="label">A</div>
        </button>
    }
}
