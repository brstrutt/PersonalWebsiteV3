use wasmfenbein3d::{include_texture, state::textures::RawTexture};

pub const ENVIRONMENT_BIG_FLOOR: RawTexture = include_texture!("environment/", "big_floor");
pub const ENVIRONMENT_FLOOR: RawTexture = include_texture!("environment/", "floor");
pub const ENVIRONMENT_WALL_STONE: RawTexture = include_texture!("environment/", "wall_stone");
pub const ENVIRONMENT_WALL_WOOD: RawTexture = include_texture!("environment/", "wall_wood");

pub const ENVIRONMENT_FLOOR_WOOD: RawTexture = include_texture!("environment/", "floor_wood");
pub const ENVIRONMENT_CEILING_WHITE: RawTexture = include_texture!("environment/", "ceiling_white");
pub const ENVIRONMENT_WALL_DEFAULT: RawTexture = include_texture!("environment/", "wall_default");
pub const ENVIRONMENT_WALL_BOOKSHELVES: RawTexture =
    include_texture!("environment/", "wall_bookshelves");
pub const ENVIRONMENT_WALL_BIG_DOORS: RawTexture =
    include_texture!("environment/", "wall_big_doors");
pub const ENVIRONMENT_WALL_COAT_HOOKS: RawTexture =
    include_texture!("environment/", "wall_coat_hooks");
pub const ENVIRONMENT_SPLASH_SCREEN: RawTexture = include_texture!("environment/", "splashscreen");
