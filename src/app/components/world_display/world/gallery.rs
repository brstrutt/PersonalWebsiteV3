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
            Point2D::new(4.0, 7.0),
            Point2D::new(3.0, 7.0),
            Point2D::new(1.0, 9.0),
            Point2D::new(1.0, 9.25),
            Point2D::new(0.25, 9.25),
        ],
        &wood_wall_texture,
    ));
    result.append(&mut walls_from_point_path(
        &vec![
            Point2D::new(0.25, 9.75),
            Point2D::new(1.0, 9.75),
            Point2D::new(1.0, 10.0),
            Point2D::new(5.0, 14.0),
            Point2D::new(9.0, 14.0),
            Point2D::new(19.0, 4.0),
            Point2D::new(16.0, 1.0),
            Point2D::new(9.0, 8.0),
            Point2D::new(6.75, 5.75),
            Point2D::new(7.75, 4.75),
            Point2D::new(8.0, 5.0),
            Point2D::new(9.0, 5.0),
            Point2D::new(10.0, 4.0),
            Point2D::new(10.0, 3.0),
            Point2D::new(9.0, 2.0),
            Point2D::new(8.0, 2.0),
            Point2D::new(7.0, 3.0),
            Point2D::new(7.0, 4.0),
            Point2D::new(7.25, 4.25),
            Point2D::new(6.25, 5.25),
        ],
        &wood_wall_texture,
    ));
    result.append(&mut walls_from_point_path(
        &vec![
            Point2D::new(6.0, 7.0),
            Point2D::new(5.0, 8.0),
            Point2D::new(7.0, 10.0),
            Point2D::new(8.0, 9.0),
            Point2D::new(6.0, 7.0),
        ],
        &wood_wall_texture,
    ));
    result.append(&mut walls_from_point_path(
        &vec![
            Point2D::new(10.5, 8.5),
            Point2D::new(10.25, 8.25),
            Point2D::new(8.25, 10.25),
            Point2D::new(8.5, 10.5),
            Point2D::new(10.5, 8.5),
        ],
        &wood_wall_texture,
    ));
    return result;
}
