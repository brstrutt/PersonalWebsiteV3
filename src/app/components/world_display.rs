use std::{cell::RefCell, rc::Rc};

use leptos::{
    html::Canvas,
    leptos_dom,
    prelude::*,
    web_sys::CanvasRenderingContext2d,
    wasm_bindgen::JsCast,
};
use wasmfenbein3d::core::{
    render::{
        render_to_screen_buffer,
        rgb_palette::RgbPalette,
        screen_buffer_row_first::ScreenBufferRowFirst,
        screen_buffer::ScreenBuffer,
    }, state::GameState,
};

mod textures;
mod world;

#[component]
pub fn world_display() -> impl IntoView {
    let node_ref = NodeRef::<Canvas>::new();
    Effect::new(move |_| {
        if let Some(element) = node_ref.get() {
            let width: u32 = element.offset_width() as u32;
            let height: u32 = element.offset_height() as u32;
            element.set_width(width/2);
            element.set_height(height/2);

            let screen_width = element.width() as usize;
            let screen_height = element.height() as usize;

            let screen_buffer = Rc::new(RefCell::new(ScreenBufferRowFirst::setup(
                screen_width,
                screen_height,
            )));

            let mut palette = RgbPalette::new();
            let walls = world::load_walls(&mut palette);
            let floor_texture = textures::big_floor::load_texture(&mut palette);
            let ceiling_texture = textures::floor::load_texture(&mut palette);

            let state = Rc::new(RefCell::new(GameState::setup(
                screen_width,
                screen_height,
                walls,
                &mut palette,
                floor_texture.clone(),
                ceiling_texture,
            )));

            leptos_dom::helpers::request_animation_frame(move || {
                render_to_screen_buffer(&screen_buffer, &state);
                if let Some(canvas) = node_ref.get() &&
                let Ok(context_result) = canvas.get_context("2d") &&
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
