use std::rc::Rc;
use wasmfenbein3d::core::render::{rgb_palette::RgbPalette, texture::Texture};

#[allow(unused)]
pub fn load_texture(palette: &mut RgbPalette) -> Rc<Texture> {
    Rc::new(Texture::new_from_bmp_data(
        include_bytes!("./ridge_racer_ds_unlock_burning_nightmare.bmp"),
        palette,
    ))
}
