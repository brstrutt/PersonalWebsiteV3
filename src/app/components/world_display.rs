use std::{cell::RefCell, rc::Rc};

use leptos::{html::Canvas, prelude::*};
use leptos_router::hooks::use_navigate;
use wasmfenbein3d::{
    render::screen_buffer_column_first::ScreenBufferColumnFirst,
    state::{textures::TextureLibrary, world, State},
};

pub mod controls;
mod textures;

#[component]
pub fn world_display() -> impl IntoView {
    let node_ref = NodeRef::<Canvas>::new();

    let navigate = use_navigate();
    Effect::new(move |_| {
        if let Some(element) = node_ref.get() {
            log::info!("Initialising wasmfenbein3d!");
            // Setup the canvas image size to be half resolution when compared to canvas element size
            let width: u32 = element.offset_width() as u32;
            let height: u32 = element.offset_height() as u32;
            element.set_width(width / 2);
            element.set_height(height / 2);
            log::info!("Canvas element resized...");

            // Setup the screen buffer we render to
            let screen_width = element.height() as usize;
            let screen_height = element.width() as usize;
            let screen_buffer = ScreenBufferColumnFirst::setup(screen_width, screen_height);
            log::info!("Screen buffer created...");

            let textures = TextureLibrary::load(&textures::TILING_TEXTURES, &textures::TEXTURES);
            log::info!("Textures loaded...");

            // Setup the wasmfenbein3d state object
            let state = Rc::new(RefCell::new(State::setup(
                screen_buffer,
                world::World::load(&textures, include_str!("world_display/world/data.json")),
            )));
            log::info!("Game state initialised...");

            // Setup the render loop
            wasmfenbein3d::wasmrender::setup(state.clone(), element.clone());
            log::info!("Render loop setup...");

            // Setup controls
            wasmfenbein3d::wasmcontrols::setup(
                state.clone(),
                element.clone(),
                |id: &str| {
                    controls::on_click(id, navigate.clone());
                },
                || {
                    controls::on_mouse_capture(navigate.clone());
                },
            );
            log::info!("Wasm controls setup...");
            log::info!("Wasmfenbein3d initialised!");
        }
    });

    view! { <canvas node_ref=node_ref class="world_display" /> }
}
