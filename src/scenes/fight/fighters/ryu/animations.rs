use crate::components::animation::{AnimationSprite, AnimationTransition};

use super::sprites::{RYU_STAND_1, RYU_STAND_2, RYU_STAND_3, RYU_STAND_4};

const RYU_TRANSITION_STAND: AnimationTransition = AnimationTransition {
    sprites: &[
        AnimationSprite {
            texture: RYU_STAND_1,
            duration: 3,
        },
        AnimationSprite {
            texture: RYU_STAND_2,
            duration: 3,
        },
        AnimationSprite {
            texture: RYU_STAND_3,
            duration: 3,
        },
        AnimationSprite {
            texture: RYU_STAND_4,
            duration: 3,
        },
    ],
};

pub const RYU_TRANSITION_STAND_INDEX: usize = 0;
pub const RYU_TRANSITIONS: [AnimationTransition; 1] = [RYU_TRANSITION_STAND];
