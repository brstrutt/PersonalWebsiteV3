use super::super::textures;
use wasmfenbein3d::core::{
    primitives::{line2d::Line2D, point2d::Point2D},
    render::rgb_palette::RgbPalette,
    world::{painting::Painting, wall::Wall, walls::walls_from_point_path},
};

pub mod entity_ids {
    pub const VERMINTIDE_TAPESTRY_ID: &str = "vermintide_tapestry";
}
use super::entity_ids::*;

pub fn get_walls(palette: &mut RgbPalette) -> Vec<Wall> {
    let wood_wall_texture = textures::environment::wall_wood::load_texture(palette);

    let vermintide_tapestry =
        textures::paintings::pixel_art::vermintide_tapestry::load_texture(palette);
    let blood_in_the_darkness =
        textures::paintings::maps::blood_in_the_darkness::load_texture(palette);
    let burplespue_halescourge =
        textures::paintings::maps::burplespue_halescourge::load_texture(palette);
    let castle_drachenfels = textures::paintings::maps::castle_drachenfels::load_texture(palette);
    let into_the_nest = textures::paintings::maps::into_the_nest::load_texture(palette);
    let righteous_stand = textures::paintings::maps::righteous_stand::load_texture(palette);
    let taals_horn_keep = textures::paintings::maps::taals_horn_keep::load_texture(palette);

    let ubersreik_five = textures::paintings::pixel_art::ubersreik_five::load_texture(palette);
    let postman = textures::paintings::pixel_art::postman::load_texture(palette);
    let zant = textures::paintings::pixel_art::zant::load_texture(palette);
    let pokemon = textures::paintings::pixel_art::pokemon::load_texture(palette);
    let bradley = textures::paintings::pixel_art::bradley::load_texture(palette);

    let daily_fall = textures::paintings::pixel_art::daily_fall::load_texture(palette);

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
        ],
        &wood_wall_texture,
    ));

    result.push(Wall::new(
        Line2D {
            start: Point2D::new(9.0, 14.0),
            end: Point2D::new(5.0, 14.0),
        },
        &wood_wall_texture,
        vec![
            Painting::new_to_scale(
                VERMINTIDE_TAPESTRY_ID,
                vermintide_tapestry,
                Point2D::new(0.85, 0.1),
            ),
            Painting::new(
                DUMMY_ID,
                blood_in_the_darkness,
                Point2D::new(0.50, 0.5),
                Point2D::new(0.80, 0.8),
            ),
            Painting::new(
                DUMMY_ID,
                burplespue_halescourge,
                Point2D::new(0.50, 1.3),
                Point2D::new(0.80, 1.6),
            ),
            Painting::new(
                DUMMY_ID,
                castle_drachenfels,
                Point2D::new(0.50, 0.9),
                Point2D::new(0.80, 1.2),
            ),
            Painting::new(
                DUMMY_ID,
                into_the_nest,
                Point2D::new(3.40, 0.5),
                Point2D::new(3.70, 0.8),
            ),
            Painting::new(
                DUMMY_ID,
                righteous_stand,
                Point2D::new(3.40, 1.3),
                Point2D::new(3.70, 1.6),
            ),
            Painting::new(
                DUMMY_ID,
                taals_horn_keep,
                Point2D::new(3.40, 0.9),
                Point2D::new(3.70, 1.2),
            ),
        ],
    ));

    result.push(Wall::new(
        Line2D {
            start: Point2D::new(19.0, 4.0),
            end: Point2D::new(9.0, 14.0),
        },
        &wood_wall_texture,
        vec![
            Painting::new_to_scale(DUMMY_ID, zant, Point2D::new(0.1, 0.1)),
            Painting::new_to_scale(DUMMY_ID, bradley, Point2D::new(3.1, 0.1)),
            Painting::new_to_scale(DUMMY_ID, postman, Point2D::new(5.1, 0.1)),
            Painting::new_to_scale(DUMMY_ID, pokemon, Point2D::new(9.1, 0.8)),
            Painting::new_to_scale(DUMMY_ID, ubersreik_five, Point2D::new(11.1, 0.1)),
        ],
    ));

    result.append(&mut walls_from_point_path(
        &vec![
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
        &vec![Point2D::new(10.5, 8.5), Point2D::new(10.25, 8.25)],
        &wood_wall_texture,
    ));

    result.push(Wall::new(
        Line2D {
            start: Point2D::new(10.25, 8.25),
            end: Point2D::new(8.25, 10.25),
        },
        &wood_wall_texture,
        vec![Painting::new_to_scale(
            DUMMY_ID,
            daily_fall,
            Point2D::new(0.1, 0.1),
        )],
    ));

    result.append(&mut walls_from_point_path(
        &vec![
            Point2D::new(8.25, 10.25),
            Point2D::new(8.5, 10.5),
            Point2D::new(10.5, 8.5),
        ],
        &wood_wall_texture,
    ));
    return result;
}
