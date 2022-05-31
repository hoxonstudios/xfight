use crate::systems::drawing::Sprite;

pub const FLOOR_TEXTURE_PATH: &'static str = "assets/background.png";

pub const FLOOR_SPRITE: Sprite = Sprite {
    center: (600, 500),
    area: (0, 0, 1187, 600),
};
