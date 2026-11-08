use leptos::{html::Canvas, prelude::*};
use web_sys::MouseEvent;

use crate::app::{
    components::game_ui::WorldDisplayState,
    utils::{render_texture_on_canvas, request_pointer_lock},
};

pub mod controls;
pub mod textures;
use textures::{ENVIRONMENT_LOADING_SCREEN, ENVIRONMENT_SPLASH_SCREEN};

#[component]
pub fn world_display(
    node_ref: NodeRef<Canvas>,
    state: ReadSignal<WorldDisplayState>,
    set_state: WriteSignal<WorldDisplayState>,
    textures_to_load: usize,
    textures_loaded: ReadSignal<usize>,
) -> impl IntoView {
    Effect::new(move |_| {
        if let Some(element) = node_ref.get() {
            // Setup the canvas image size to be half resolution when compared to canvas element size
            let width: u32 = element.offset_width() as u32;
            let height: u32 = element.offset_height() as u32;
            element.set_width(width / 2);
            element.set_height(height / 2);

            // Render the correct image for the current state
            match state.get() {
                WorldDisplayState::SplashScreen => {
                    render_texture_on_canvas(element, ENVIRONMENT_SPLASH_SCREEN.bytes)
                }
                WorldDisplayState::Loading => {
                    render_texture_on_canvas(element, ENVIRONMENT_LOADING_SCREEN.bytes)
                }
                _ => {}
            }
        }
    });

    view! {
        <canvas
            node_ref=node_ref
            class="world_display"
            on:click=move |click_event: MouseEvent| {
                if state.get() == WorldDisplayState::SplashScreen {
                    request_pointer_lock(click_event.into());
                    set_state.set(WorldDisplayState::Loading)
                }
            }
        />
        {move || {
            if state.get() == WorldDisplayState::Loading {
                Some(
                    view! {
                        <progress
                            class="loading_world"
                            max=textures_to_load
                            value=textures_loaded
                        />
                    },
                )
            } else {
                None
            }
        }}
    }
}
