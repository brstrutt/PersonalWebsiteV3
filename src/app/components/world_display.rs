use std::{cell::RefCell, rc::Rc};

use leptos::{html::Canvas, prelude::*};
use leptos_router::hooks::use_navigate;
use wasmfenbein3d::{
    render::screen_buffer_column_first::ScreenBufferColumnFirst,
    state::{textures::TextureLibrary, world, State},
};

use crate::app::utils::LoggerWithDuration;

pub mod controls;
mod textures;

#[component]
pub fn world_display() -> impl IntoView {
    let node_ref = NodeRef::<Canvas>::new();

    let navigate = use_navigate();
    Effect::new(move |_| {
        if let Some(element) = node_ref.get() {
            let mut logger = LoggerWithDuration::new();
            logger.log_time("Initialising wasmfenbein3d!");
            // Setup the canvas image size to be half resolution when compared to canvas element size
            let width: u32 = element.offset_width() as u32;
            let height: u32 = element.offset_height() as u32;
            element.set_width(width / 2);
            element.set_height(height / 2);
            logger.log_time("Canvas element resized...");

            // Setup the screen buffer we render to
            let screen_width = element.height() as usize;
            let screen_height = element.width() as usize;
            let screen_buffer = ScreenBufferColumnFirst::setup(screen_width, screen_height);
            logger.log_time("Screen buffer created...");

            let textures = TextureLibrary::load(&textures::TILING_TEXTURES, &textures::TEXTURES);
            logger.log_time("Textures loaded...");

            // Setup the wasmfenbein3d state object
            let state = Rc::new(RefCell::new(State::setup(
                screen_buffer,
                world::World::load(&textures, include_str!("world_display/world/data.json")),
            )));
            logger.log_time("Game state initialised...");

            // Setup the render loop
            wasmfenbein3d::wasmrender::setup(state.clone(), element.clone());
            logger.log_time("Render loop setup...");

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
            logger.log_time("Wasm controls setup...");
            logger.log_time("Wasmfenbein3d initialised!");
        }
    });

    view! { <canvas node_ref=node_ref class="world_display" /> }
}
