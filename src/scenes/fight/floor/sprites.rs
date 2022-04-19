use crate::systems::drawing::Sprite;

pub const FLOOR_TEXTURE_PATH: &'static str = "assets/floor.png";

pub const FLOOR_SPRITE: Sprite = Sprite {
    center: (400, 100),
    area: (0, 0, 800, 200),
};
