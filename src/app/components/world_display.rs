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
    wasm_bindgen::{JsCast},
    web_sys::{
        CanvasRenderingContext2d, EventTarget, HtmlCanvasElement,
    },
};
use wasmfenbein3d::core::{
    motion,
    render::{
        render_to_screen_buffer,
        rgb_palette::RgbPalette,
        screen_buffer::ScreenBuffer,
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
            character_motion_loop(state.clone());

            // Setup the render loop
            render_loop(screen_buffer, state, element);
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
    add_event_listener_with_callback(EventTarget::from(cloned_canvas.clone()), "click", move |_: Event| {
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

fn render_loop<T: ScreenBuffer + 'static>(
    screen_buffer: Rc<RefCell<T>>,
    state: Rc<RefCell<GameState>>,
    element: HtmlCanvasElement,
) {
    let render_start_time = leptos_dom::helpers::window().performance().unwrap().now();
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
    let render_end_time = leptos_dom::helpers::window().performance().unwrap().now();

    {
        let mut state = state.borrow_mut();
        state.last_time_to_render_one_frame_ms = render_end_time - render_start_time;
    }

    leptos_dom::helpers::request_animation_frame(|| render_loop(screen_buffer, state, element));
}


fn character_motion_loop(state: Rc<RefCell<GameState>>) {
    {
        let mut state = state.borrow_mut();

        // Use time delta to control for framerate variations
        let current_time = leptos_dom::helpers::window().performance().unwrap().now();
        let time_since_last_frame_ms = current_time - state.last_frame_time_ms;

        let time_since_last_frame_s = time_since_last_frame_ms / 1000.0;

        // Calculate movement speed
        let velocity_per_s = if state.input.sprint { 12.0 } else { 4.0 };
        let velocity = velocity_per_s * time_since_last_frame_s;

        // Calculate the direction the player is facing
        let camera_rotation = state.world.camera.ray.get_angle();
        let motion = state
            .input
            .get_cameraspace_movement_direction()
            .rotate(camera_rotation)
            * velocity;

        // Move the character
        state.world.camera.ray.origin =
            motion::move_object(state.world.camera.ray.origin, &motion, &state.world);


        // Rotate the camera
        const ROTATION_SPEED: f64 = 0.001;

        let camera_rotation = state.input.camera_rotation;
        state.input.camera_rotation = 0;

        if camera_rotation != 0 {
            state.world.camera = state
                .world
                .camera
                .rotate(camera_rotation as f64 * ROTATION_SPEED);
        }
        state.world.camera.refresh_screen_rays();


        // Track the last time we ran a physics tick
        state.last_frame_time_ms = current_time;

        // Track how much time is passing between each tick (for FPS calculation/display)
        state.last_time_between_frames_ms = time_since_last_frame_ms;
    }

    leptos_dom::helpers::request_animation_frame(|| character_motion_loop(state));
}
