use super::super::textures;
use wasmfenbein3d::core::{
    primitives::{line2d::Line2D, point2d::Point2D},
    render::rgb_palette::RgbPalette,
    world::{painting::Painting, wall::Wall, walls::walls_from_point_path},
};

pub mod entity_ids {
    pub const RIDGE_RACER_BURNING_NIGHTMARE_ID: &str = "ridge_racer_burning_nightmare";
    pub const FIRST_TURN_KILL_BOSSES_IN_BRAVELY_DEFAULT2_ID: &str =
        "first_turn_kill_bosses_in_bravely_default2";
    pub const PAYDAY2_SAFEHOUSE_NIGHTMARE_ID: &str = "payday2_safehouse_nightmare";
    pub const CONTROL_DARK_SOULS_WITH_A_PIANO_ID: &str = "control_dark_souls_with_a_piano";
    pub const EMBED_PRESENTATION_IN_HUGO_ID: &str = "embed_presentation_in_hugo";
}
use super::entity_ids::*;

pub fn get_walls(palette: &mut RgbPalette) -> Vec<Wall> {
    let wood_wall_texture = textures::wall_wood::load_texture(palette);
    let ridge_racer_ds_unlock_burning_nightmare =
        textures::ridge_racer_ds_unlock_burning_nightmare::load_texture(palette);
    let first_turn_kill_bosses_in_bravely_default2 =
        textures::first_turn_kill_bosses_in_bravely_default2::load_texture(palette);
    let payday2_safehouse_nightmare = textures::payday2_safehouse_nightmare::load_texture(palette);
    let control_dark_souls_with_a_piano =
        textures::control_dark_souls_with_a_piano::load_texture(palette);
    let embed_presentation_in_hugo = textures::embed_presentation_in_hugo::load_texture(palette);

    let mut result = Vec::<Wall>::new();
    result.append(&mut walls_from_point_path(
        &vec![
            Point2D::new(-3.0, 4.0),
            Point2D::new(-3.0, 6.0),
            Point2D::new(0.25, 9.25),
        ],
        &wood_wall_texture,
    ));

    result.push(Wall::new(
        Line2D {
            start: Point2D::new(0.25, 9.75),
            end: Point2D::new(-3.5, 12.5),
        },
        &wood_wall_texture,
        vec![
            Painting::new(
                CONTROL_DARK_SOULS_WITH_A_PIANO_ID,
                control_dark_souls_with_a_piano,
                Point2D::new(0.1, 0.2),
                Point2D::new(1.9, 1.8),
            ),
            Painting::new(
                EMBED_PRESENTATION_IN_HUGO_ID,
                embed_presentation_in_hugo,
                Point2D::new(2.1, 0.2),
                Point2D::new(3.9, 1.8),
            ),
        ],
    ));

    result.push(Wall::new(
        Line2D {
            start: Point2D::new(-3.5, 12.5),
            end: Point2D::new(-9.0, 8.0),
        },
        &wood_wall_texture,
        vec![
            Painting::new(
                RIDGE_RACER_BURNING_NIGHTMARE_ID,
                ridge_racer_ds_unlock_burning_nightmare,
                Point2D::new(0.1, 0.2),
                Point2D::new(1.9, 1.8),
            ),
            Painting::new(
                FIRST_TURN_KILL_BOSSES_IN_BRAVELY_DEFAULT2_ID,
                first_turn_kill_bosses_in_bravely_default2,
                Point2D::new(2.1, 0.2),
                Point2D::new(3.9, 1.8),
            ),
            Painting::new(
                PAYDAY2_SAFEHOUSE_NIGHTMARE_ID,
                payday2_safehouse_nightmare,
                Point2D::new(4.1, 0.2),
                Point2D::new(5.9, 1.8),
            ),
        ],
    ));

    result.append(&mut walls_from_point_path(
        &vec![Point2D::new(-9.0, 8.0), Point2D::new(-4.0, 3.0)],
        &wood_wall_texture,
    ));
    return result;
}
