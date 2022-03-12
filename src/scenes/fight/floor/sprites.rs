use crate::systems::drawing::TextureSprite;

pub const FLOOR_TEXTURE_PATH: &'static str = "assets/floor.png";

pub const FLOOR_SPRITE: TextureSprite = TextureSprite {
    position: (0, 0),
    size: (800, 200),
};
