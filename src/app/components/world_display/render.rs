use std::{cell::RefCell, rc::Rc};
use leptos::leptos_dom;
use leptos::web_sys::{HtmlCanvasElement, CanvasRenderingContext2d};
use leptos::wasm_bindgen::{JsCast};
use wasmfenbein3d::core::{render::{render_to_screen_buffer, screen_buffer::ScreenBuffer}, state::GameState};

pub fn render_loop<T: ScreenBuffer + 'static>(
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