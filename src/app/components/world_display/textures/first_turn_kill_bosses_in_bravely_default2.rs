use std::rc::Rc;
use wasmfenbein3d::core::render::{rgb_palette::RgbPalette, texture::Texture};

#[allow(unused)]
pub fn load_texture(palette: &mut RgbPalette) -> Rc<Texture> {
    Rc::new(Texture::new_from_bmp_data(
        include_bytes!("./first_turn_kill_bosses_in_bravely_default2.bmp"),
        palette,
    ))
}
