use xfight_ecs::components::{Animation, AnimationSprite};

use super::sprites::{RYU_STAND_1, RYU_STAND_2};

pub const RYU_STAND: Animation = Animation {
    sprites: &[
        AnimationSprite {
            texture: &RYU_STAND_1,
            duration_frames: 5,
        },
        AnimationSprite {
            texture: &RYU_STAND_2,
            duration_frames: 5,
        },
    ],
};
