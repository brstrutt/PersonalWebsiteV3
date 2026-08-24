use std::{cell::RefCell, rc::Rc};

use leptos::{
    html::{Button, Canvas}, prelude::*,
};
use leptos_router::hooks::use_navigate;
use wasmfenbein3d::core::{
    render::{
        rgb_palette::RgbPalette,
        screen_buffer_row_first::ScreenBufferRowFirst,
    }, state::GameState,
};

use controls::setup_controls;
use render::render_loop;
use physics::character_motion_loop;
use super::touchscreen_button::TouchscreenButton;


mod controls;
mod textures;
mod world;
mod render;
mod physics;

#[component]
pub fn world_display() -> impl IntoView {
    let node_ref = NodeRef::<Canvas>::new();
    let touchscreen_control_node_refs = TouchscreenMovementControlButtonRefs{
        left: NodeRef::<Button>::new(),
        right: NodeRef::<Button>::new(),
        up: NodeRef::<Button>::new(),
        down: NodeRef::<Button>::new(),
    };

    let navigate = use_navigate();
    Effect::new(move |_| {
        if let Some(element) = node_ref.get() &&
        let Some(left_button_ref) = touchscreen_control_node_refs.left.get() &&
        let Some(right_button_ref) = touchscreen_control_node_refs.right.get() &&
        let Some(up_button_ref) = touchscreen_control_node_refs.up.get() &&
        let Some(down_button_ref) = touchscreen_control_node_refs.down.get() {
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
            setup_controls(element.clone(), state.clone(), left_button_ref, right_button_ref, up_button_ref, down_button_ref, navigate.clone());
            character_motion_loop(state.clone());

            // Setup the render loop
            render_loop(screen_buffer, state, element);
        }
    });

    view! {
        <canvas node_ref=node_ref class="world_display" />
        <TouchscreenMovementControls touchscreen_control_node_refs=touchscreen_control_node_refs />
    }
}

struct TouchscreenMovementControlButtonRefs {
    pub left: NodeRef::<Button>,
    pub right: NodeRef::<Button>,
    pub up: NodeRef::<Button>,
    pub down: NodeRef::<Button>,
}

#[component]
fn touchscreen_movement_controls(
    /// Mutable reference to allow reference to controls to bubble up
    #[prop(into)]
    touchscreen_control_node_refs: TouchscreenMovementControlButtonRefs,
) -> impl IntoView {
    view! {
        <div class="screen_controls">
            <TouchscreenButton node_ref=touchscreen_control_node_refs.left>"◄"</TouchscreenButton>
            <div class="vertical_movement_buttons">
                <TouchscreenButton node_ref=touchscreen_control_node_refs
                    .up>"▲"</TouchscreenButton>
                <TouchscreenButton node_ref=touchscreen_control_node_refs
                    .down>"▼"</TouchscreenButton>
            </div>
            <TouchscreenButton node_ref=touchscreen_control_node_refs
                .right>"►"</TouchscreenButton>
        </div>
    }
}
