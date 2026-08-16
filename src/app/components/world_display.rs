use std::{cell::RefCell, rc::Rc};

use leptos::{
    ev::{
        self,
        Event,
        KeyboardEvent,
        MouseEvent
    },
    html::Canvas,
    leptos_dom,
    prelude::*,
    web_sys::{
        EventTarget, HtmlCanvasElement,
    },
};
use wasmfenbein3d::core::{
    render::{
        rgb_palette::RgbPalette,
        screen_buffer_row_first::ScreenBufferRowFirst,
    }, state::GameState,
};

use crate::app::utils::add_event_listener_with_callback;

mod textures;
mod world;

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

            // Setup the render loop
            leptos_dom::helpers::request_animation_frame(move || {
                render_to_screen_buffer(&screen_buffer, &state);
                if let Ok(context_result) = element.get_context("2d") &&
                let Some(canvas_context) = context_result {
                    let buffer = screen_buffer.borrow();
                    canvas_context
                    .dyn_into::<CanvasRenderingContext2d>()
                    .expect("Failed to get 2D context even MORE")
                    .put_image_data(&buffer.to_imagedata(), 0.0, 0.0)
                    .expect("Failed to copy Screen Buffer to canvas.");
                }
            });
        }
    });

    view! { <canvas node_ref=node_ref class="world_display" /> }
}

fn setup_controls(canvas_element: HtmlCanvasElement, state: Rc<RefCell<GameState>>) {
    let cloned_state = state.clone();
    add_event_listener_with_callback(EventTarget::from(document()), "pointerlockchange", move |_: Event| {
        let mut state = cloned_state.borrow_mut();
        state.input.pointer_locked = document().pointer_lock_element().is_some();
        if !state.input.pointer_locked {
            state.input.sprint = false;
            state.input.move_left = false;
            state.input.move_right = false;
            state.input.move_forward = false;
            state.input.move_backward = false;
        }
    });
    let cloned_state = state.clone();
    leptos_dom::helpers::window_event_listener(ev::mousemove, move |e: MouseEvent| {
        let mut state = cloned_state.borrow_mut();

        if state.input.pointer_locked {
            state.input.camera_rotation += e.movement_x();
        }
    });
    let cloned_canvas = canvas_element.clone();
    leptos_dom::helpers::window_event_listener(ev::click, move |_| {
        cloned_canvas.request_pointer_lock();
    });


    let cloned_state = state.clone();
    add_event_listener_with_callback(EventTarget::from(document()), "keydown", move |e: KeyboardEvent| {
        let mut state = cloned_state.borrow_mut();
        if state.input.pointer_locked {
            state.input.sprint = e.shift_key();
            match e.key().as_str() {
                "a" | "A" => state.input.move_left = true,
                "d" | "D" => state.input.move_right = true,
                "w" | "W" => state.input.move_forward = true,
                "s" | "S" => state.input.move_backward = true,
                &_ => return,
            }
        }
    });

    let cloned_state = state.clone();
    add_event_listener_with_callback(EventTarget::from(document()), "keyup", move |e: KeyboardEvent| {
        let mut state = cloned_state.borrow_mut();
        if state.input.pointer_locked {
            state.input.sprint = e.shift_key();
            match e.key().as_str() {
                "a" | "A" => state.input.move_left = false,
                "d" | "D" => state.input.move_right = false,
                "w" | "W" => state.input.move_forward = false,
                "s" | "S" => state.input.move_backward = false,
                &_ => return,
            }
        }
    });
}

