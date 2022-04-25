use crate::{
    scenes::fight::fighters::{STATE_GROUNDED, STATE_STUN},
    systems::{
        damage::DamagePoint,
        drawing::Sprite,
        movement::{Movement, MovementSprite, MovementTransition, MovementTransitionCondition},
    },
};

use super::{RYU_JUMP_STUN_INDEX, RYU_STAND_INDEX};

pub const RYU_JUMP_STRONG_PUNCH: Movement = Movement {
    sprites: &[
        MovementSprite {
            sprite: Sprite {
                center: (24, 562),
                area: (2, 543, 44, 612),
            },
            velocity_change: None,
            damage_point: None,
            shield: None,
            frames: 3,
        },
        MovementSprite {
            sprite: Sprite {
                center: (67, 555),
                area: (47, 548, 100, 605),
            },
            damage_point: Some(DamagePoint {
                point: (28, 27),
                power: 5,
                tag: STATE_STUN,
            }),
            velocity_change: None,
            shield: None,
            frames: 10,
        },
    ],
    transitions: &[
        MovementTransition {
            conditions: &[MovementTransitionCondition::StateActive(STATE_STUN)],
            movement: RYU_JUMP_STUN_INDEX,
            wait: false,
        },
        MovementTransition {
            conditions: &[MovementTransitionCondition::StateActive(STATE_GROUNDED)],
            movement: RYU_STAND_INDEX,
            wait: false,
        },
    ],
};
