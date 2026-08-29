use super::super::textures;
use wasmfenbein3d::core::{
    primitives::point2d::Point2D,
    render::rgb_palette::RgbPalette,
    world::{wall::Wall, walls::walls_from_point_path},
};

pub fn get_walls(palette: &mut RgbPalette) -> Vec<Wall> {
    let wood_wall_texture = textures::wall_wood::load_texture(palette);

    let mut result = Vec::<Wall>::new();
    result.append(&mut walls_from_point_path(
        &vec![
            Point2D::new(-3.0, 4.0),
            Point2D::new(-3.0, 6.0),
            Point2D::new(0.25, 9.25),
        ],
        &wood_wall_texture,
    ));
    result.append(&mut walls_from_point_path(
        &vec![
            Point2D::new(0.25, 9.75),
            Point2D::new(-3.5, 12.5),
            Point2D::new(-9.0, 8.0),
            Point2D::new(-4.0, 3.0),
        ],
        &wood_wall_texture,
    ));
    return result;
}
