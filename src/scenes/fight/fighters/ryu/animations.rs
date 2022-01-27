use crate::components::animation::{AnimationSprite, AnimationTransition};

use super::sprites::{
    RYU_STAND_1, RYU_STAND_2, RYU_STAND_3, RYU_STAND_4, RYU_WALKING_1, RYU_WALKING_2,
    RYU_WALKING_3, RYU_WALKING_4, RYU_WALKING_5, RYU_WALKING_6,
};

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

const RYU_TRANSITION_MOVING_FORWARD: AnimationTransition = AnimationTransition {
    sprites: &[
        AnimationSprite {
            texture: RYU_WALKING_1,
            duration: 3,
        },
        AnimationSprite {
            texture: RYU_WALKING_2,
            duration: 3,
        },
        AnimationSprite {
            texture: RYU_WALKING_3,
            duration: 3,
        },
        AnimationSprite {
            texture: RYU_WALKING_4,
            duration: 3,
        },
        AnimationSprite {
            texture: RYU_WALKING_5,
            duration: 3,
        },
        AnimationSprite {
            texture: RYU_WALKING_6,
            duration: 3,
        },
    ],
};

const RYU_TRANSITION_MOVING_BACKWARD: AnimationTransition = AnimationTransition {
    sprites: &[
        AnimationSprite {
            texture: RYU_WALKING_6,
            duration: 3,
        },
        AnimationSprite {
            texture: RYU_WALKING_5,
            duration: 3,
        },
        AnimationSprite {
            texture: RYU_WALKING_4,
            duration: 3,
        },
        AnimationSprite {
            texture: RYU_WALKING_3,
            duration: 3,
        },
        AnimationSprite {
            texture: RYU_WALKING_2,
            duration: 3,
        },
        AnimationSprite {
            texture: RYU_WALKING_1,
            duration: 3,
        },
    ],
};

pub const RYU_TRANSITIONS: [AnimationTransition; 3] = [
    RYU_TRANSITION_STAND,
    RYU_TRANSITION_MOVING_FORWARD,
    RYU_TRANSITION_MOVING_BACKWARD,
];
