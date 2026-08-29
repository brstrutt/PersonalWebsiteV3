use wasmfenbein3d::core::{render::rgb_palette::RgbPalette, world::wall::Wall};
mod entrance;
mod gallery;
mod library;

pub mod entity_ids {
    pub const DUMMY_ID: &str = "no_on_click_behaviour";
    pub use super::gallery::entity_ids::*;
    pub use super::library::entity_ids::*;
}

pub fn load_walls(palette: &mut RgbPalette) -> Vec<Wall> {
    let mut result = Vec::<Wall>::new();
    result.append(&mut entrance::get_walls(palette));
    result.append(&mut gallery::get_walls(palette));
    result.append(&mut library::get_walls(palette));
    result
}
