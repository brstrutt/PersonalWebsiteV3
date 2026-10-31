use wasmfenbein3d::{include_texture, state::textures::RawTexture};

pub const ENVIRONMENT_BIG_FLOOR: RawTexture = include_texture!("environment/", "big_floor", ".bmp");
pub const ENVIRONMENT_FLOOR: RawTexture = include_texture!("environment/", "floor", ".bmp");
pub const ENVIRONMENT_WALL_STONE: RawTexture =
    include_texture!("environment/", "wall_stone", ".bmp");
pub const ENVIRONMENT_WALL_WOOD: RawTexture = include_texture!("environment/", "wall_wood", ".bmp");

pub const ENVIRONMENT_FLOOR_WOOD: RawTexture =
    include_texture!("environment/", "floor_wood", ".bmp");
pub const ENVIRONMENT_CEILING_WHITE: RawTexture =
    include_texture!("environment/", "ceiling_white", ".bmp");
pub const ENVIRONMENT_WALL_DEFAULT: RawTexture =
    include_texture!("environment/", "wall_default", ".bmp");
pub const ENVIRONMENT_WALL_BOOKSHELVES: RawTexture =
    include_texture!("environment/", "wall_bookshelves", ".bmp");
pub const ENVIRONMENT_WALL_BIG_DOORS: RawTexture =
    include_texture!("environment/", "wall_big_doors", ".bmp");
pub const ENVIRONMENT_WALL_COAT_HOOKS: RawTexture =
    include_texture!("environment/", "wall_coat_hooks", ".bmp");
pub const ENVIRONMENT_SPLASH_SCREEN: RawTexture =
    include_texture!("environment/", "splashscreen", ".bmp");
pub const ENVIRONMENT_LOADING_SCREEN: RawTexture =
    include_texture!("environment/", "loading", ".bmp");
