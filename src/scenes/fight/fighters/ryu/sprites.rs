use crate::components::shape::ShapeTexture;

pub const RYU_SPRITE_PATH: &'static str = "assets/ryu.png";
const RYU_TEXTURE_INDEX: usize = 0;

// STAND
pub const RYU_STAND_1: ShapeTexture = ShapeTexture {
    texture_index: RYU_TEXTURE_INDEX,
    position: (0, 0),
    size: (50, 90),
};
pub const RYU_STAND_2: ShapeTexture = ShapeTexture {
    texture_index: RYU_TEXTURE_INDEX,
    position: (50, 0),
    size: (50, 90),
};
pub const RYU_STAND_3: ShapeTexture = ShapeTexture {
    texture_index: RYU_TEXTURE_INDEX,
    position: (100, 0),
    size: (50, 90),
};
pub const RYU_STAND_4: ShapeTexture = ShapeTexture {
    texture_index: RYU_TEXTURE_INDEX,
    position: (150, 0),
    size: (50, 90),
};

// WALKING
pub const RYU_WALKING_1: ShapeTexture = ShapeTexture {
    texture_index: RYU_TEXTURE_INDEX,
    position: (200, 0),
    size: (50, 90),
};
pub const RYU_WALKING_2: ShapeTexture = ShapeTexture {
    texture_index: RYU_TEXTURE_INDEX,
    position: (250, 0),
    size: (50, 90),
};
pub const RYU_WALKING_3: ShapeTexture = ShapeTexture {
    texture_index: RYU_TEXTURE_INDEX,
    position: (300, 0),
    size: (50, 90),
};
pub const RYU_WALKING_4: ShapeTexture = ShapeTexture {
    texture_index: RYU_TEXTURE_INDEX,
    position: (350, 0),
    size: (50, 90),
};
pub const RYU_WALKING_5: ShapeTexture = ShapeTexture {
    texture_index: RYU_TEXTURE_INDEX,
    position: (400, 0),
    size: (50, 90),
};
pub const RYU_WALKING_6: ShapeTexture = ShapeTexture {
    texture_index: RYU_TEXTURE_INDEX,
    position: (450, 0),
    size: (49, 90),
};
