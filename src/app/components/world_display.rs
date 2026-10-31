use std::{cell::RefCell, io::Cursor, rc::Rc};

use image::{imageops::FilterType, ImageReader};
use leptos::{html::Canvas, prelude::*};
use leptos_router::hooks::use_navigate;
use wasmfenbein3d::{
    render::screen_buffer_column_first::ScreenBufferColumnFirst,
    state::{textures::TextureLibrary, world, State},
};
use web_sys::{MouseEvent, wasm_bindgen::{Clamped, JsCast}};

use crate::app::{
    components::world_display::textures::{ENVIRONMENT_LOADING_SCREEN, ENVIRONMENT_SPLASH_SCREEN}, utils::{LoggerWithDuration, request_pointer_lock},
};

pub mod controls;
mod textures;

#[derive(Copy, Clone, PartialEq)]
enum CanvasState {
    SplashScreen,
    Loading,
    Wasmfenbein3dLoaded,
}

#[component]
pub fn world_display() -> impl IntoView {
    let node_ref = NodeRef::<Canvas>::new();
    let (canvas_state, set_canvas_state) = signal(CanvasState::SplashScreen);

    Effect::new(move |_| {
        if let Some(element) = node_ref.get() {
            match canvas_state.get() {
                CanvasState::SplashScreen => render_splash_screen(element),
                CanvasState::Loading => render_loading_screen(element),
                _ => {}
            }
        }
    });

    let texture_count = textures::TILING_TEXTURES.len() + textures::TEXTURES.len();
    let (loaded_textures, set_loaded_textures) = signal(0);
    let textures = LocalResource::<TextureLibrary>::new(move || textures::load_async_tracking_progress(set_loaded_textures));

    let navigate = use_navigate();
    Effect::new(move |_| {
        if let Some(element) = node_ref.get() {
            textures.with(|loaded_textures| {
                if let Some(textures) = loaded_textures && canvas_state.get() == CanvasState::Loading{
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

                    // Setup the wasmfenbein3d state object
                    let state = Rc::new(RefCell::new(State::setup(
                        screen_buffer,
                        world::World::load(
                            &textures,
                            include_str!("world_display/world/data.json"),
                        ),
                    )));
                    state.borrow_mut().input.pointer_locked = window()
                        .document()
                        .expect("no global `document` exists")
                        .pointer_lock_element()
                        .is_some();
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

                    set_canvas_state.set(CanvasState::Wasmfenbein3dLoaded);
                }
            });
        }
    });

    view! {
        <canvas
            node_ref=node_ref
            class="world_display"
            on:click=move |click_event: MouseEvent| {
                if canvas_state.get() == CanvasState::SplashScreen {
                    request_pointer_lock(click_event.into());
                    set_canvas_state.set(CanvasState::Loading)
                }
            }
        />
        {move || {
            if canvas_state.get() == CanvasState::Loading {
                Some(view! { <progress class="overlay" max=texture_count value=loaded_textures /> })
            } else {
                None
            }
        }}
    }
}

fn render_loading_screen(canvas: web_sys::HtmlCanvasElement) {
    render_texture_on_canvas(canvas, ENVIRONMENT_LOADING_SCREEN.bytes);
}

fn render_splash_screen(canvas: web_sys::HtmlCanvasElement) {
    render_texture_on_canvas(canvas, ENVIRONMENT_SPLASH_SCREEN.bytes);
}

fn render_texture_on_canvas(canvas: web_sys::HtmlCanvasElement, texture_bytes: &[u8]) {
    let splash_screen_texture = ImageReader::new(Cursor::new(texture_bytes))
        .with_guessed_format()
        .expect("Failed to guess image format of splash screen raw data.")
        .decode()
        .expect("Failed to decode splash screen.")
        // Resize the image to fill the canvas. The image will be rendered at it's native size, so we adjust the size here.
        .resize_exact(canvas.height(), canvas.width(), FilterType::Nearest)
        // Rotate and flip the image to account for the canvas being rotated due to the use of the column first screen buffer
        .rotate90()
        .fliph()
        .to_rgba8();

    let splash_screen_image_data = web_sys::ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(splash_screen_texture.as_raw()),
        splash_screen_texture.width() as u32,
        splash_screen_texture.height() as u32,
    )
    .expect("couldnt convert splash_screen_texture to ImageData");

    let canvas_2d_context = canvas
        .get_context("2d")
        .expect("Rendering splash screen: failed to get canvas 2d context")
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .expect("Rendering splash screen: failed to convert 2d context to correct type");

    // Turn off image smoothing to try and keep the pixels looking crisp
    canvas_2d_context.set_image_smoothing_enabled(false);
    canvas_2d_context
        .put_image_data(&splash_screen_image_data, 0.0, 0.0)
        .expect("Rendering splash screen: failed to render splash screen");
}