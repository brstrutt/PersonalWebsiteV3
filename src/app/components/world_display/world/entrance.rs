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
            Point2D::new(5.0, 6.0),
            Point2D::new(1.0, 2.0),
            Point2D::new(-1.0, 2.0),
            Point2D::new(-3.0, 4.0),
        ],
        &wood_wall_texture,
    ));
    result.append(&mut walls_from_point_path(
        &vec![
            Point2D::new(-4.0, 3.0),
            Point2D::new(-2.0, 1.0),
            Point2D::new(-2.0, 0.0),
            Point2D::new(-1.0, -1.0),
            Point2D::new(1.0, -1.0),
            Point2D::new(2.0, 0.0),
            Point2D::new(2.0, 1.0),
            Point2D::new(6.25, 5.25),
        ],
        &wood_wall_texture,
    ));
    return result;
}
