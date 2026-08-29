use super::super::textures;
use wasmfenbein3d::core::{
    primitives::{line2d::Line2D, point2d::Point2D},
    render::rgb_palette::RgbPalette,
    world::{painting::Painting, wall::Wall, walls::walls_from_point_path},
};

pub mod entity_ids {
    pub const RIDGE_RACER_BURNING_NIGHTMARE_ID: &str = "ridge_racer_burning_nightmare";
}
use super::entity_ids::*;

pub fn get_walls(palette: &mut RgbPalette) -> Vec<Wall> {
    let wood_wall_texture = textures::wall_wood::load_texture(palette);
    let ridge_racer_ds_unlock_burning_nightmare =
        textures::ridge_racer_ds_unlock_burning_nightmare::load_texture(palette);

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
        &vec![Point2D::new(0.25, 9.75), Point2D::new(-3.5, 12.5)],
        &wood_wall_texture,
    ));

    result.push(Wall::new(
        Line2D {
            start: Point2D::new(-3.5, 12.5),
            end: Point2D::new(-9.0, 8.0),
        },
        &wood_wall_texture,
        vec![Painting::new_to_scale(
            RIDGE_RACER_BURNING_NIGHTMARE_ID,
            ridge_racer_ds_unlock_burning_nightmare,
            Point2D::new(0.85, 0.1),
        )],
    ));

    result.append(&mut walls_from_point_path(
        &vec![Point2D::new(-9.0, 8.0), Point2D::new(-4.0, 3.0)],
        &wood_wall_texture,
    ));
    return result;
}
