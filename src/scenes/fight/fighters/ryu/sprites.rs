use xfight_ecs::components::{ShapeTexture, TextureSprite};

const RYU_TEXTURE_INDEX: usize = 0;
pub const RYU_STAND_1: ShapeTexture = ShapeTexture::Texture {
    texture_index: RYU_TEXTURE_INDEX,
    sprite: TextureSprite {
        position: (0, 0),
        size: (50, 90),
    },
};
pub const RYU_STAND_2: ShapeTexture = ShapeTexture::Texture {
    texture_index: RYU_TEXTURE_INDEX,
    sprite: TextureSprite {
        position: (50, 0),
        size: (50, 90),
    },
};
