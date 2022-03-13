use crate::systems::physics::TextureSprite;

pub const FLOOR_TEXTURE_PATH: &'static str = "assets/floor.png";

pub const FLOOR_SPRITE: TextureSprite = TextureSprite {
    center: (400, 100),
    area: (0, 0, 800, 200),
};
