use std::{cell::RefCell, rc::Rc};

use leptos::{
    html::Canvas,
    prelude::*,
};
use wasmfenbein3d::core::{
    render::{
        rgb_palette::RgbPalette,
        screen_buffer_row_first::ScreenBufferRowFirst,
    }, state::GameState,
};

use controls::setup_controls;
use render::render_loop;
use physics::character_motion_loop;


mod controls;
mod textures;
mod world;
mod render;
mod physics;

#[component]
pub fn world_display() -> impl IntoView {
    let node_ref = NodeRef::<Canvas>::new();
    Effect::new(move |_| {
        if let Some(element) = node_ref.get() {
            // Setup the canvas image size to be half resolution when compared to canvas element size
            let width: u32 = element.offset_width() as u32;
            let height: u32 = element.offset_height() as u32;
            element.set_width(width/2);
            element.set_height(height/2);

            // Setup the screen buffer we render to
            let screen_width = element.width() as usize;
            let screen_height = element.height() as usize;
            let screen_buffer = Rc::new(RefCell::new(ScreenBufferRowFirst::setup(
                screen_width,
                screen_height,
            )));

            // Load the textures world
            let mut palette = RgbPalette::new();
            let walls = world::load_walls(&mut palette);
            let floor_texture = textures::big_floor::load_texture(&mut palette);
            let ceiling_texture = textures::floor::load_texture(&mut palette);

            // Setup the initial game state
            let state = Rc::new(RefCell::new(GameState::setup(
                screen_width,
                screen_height,
                walls,
                &mut palette,
                floor_texture.clone(),
                ceiling_texture,
            )));

            // Allow controlling the player character
            setup_controls(element.clone(), state.clone());
            character_motion_loop(state.clone());

            // Setup the render loop
            render_loop(screen_buffer, state, element);
        }
    });

    view! { <canvas node_ref=node_ref class="world_display" /> }
}
