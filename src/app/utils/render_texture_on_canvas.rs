use image::{imageops::FilterType, ImageReader};
use std::io::Cursor;
use web_sys::wasm_bindgen::{Clamped, JsCast};

pub fn render_texture_on_canvas(canvas: web_sys::HtmlCanvasElement, texture_bytes: &[u8]) {
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
