use crate::components::shape::ShapeTexture;

pub const FLOOR_SPRITE_PATH: &'static str = "assets/floor.png";
const FLOOR_SPRITE_INDEX: usize = 1;

pub const FLOOR_SPRITE: ShapeTexture = ShapeTexture {
    texture_index: FLOOR_SPRITE_INDEX,
    position: (0, 0),
    size: (800, 200),
};
