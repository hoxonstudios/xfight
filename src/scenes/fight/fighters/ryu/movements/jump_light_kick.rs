use crate::{
    scenes::fight::fighters::{STATE_GROUNDED, STATE_STUN},
    systems::{
        damage::DamagePoint,
        drawing::Sprite,
        movement::{Movement, MovementSprite, MovementTransition, MovementTransitionCondition},
        tag::StateTag,
    },
};

use super::{RYU_JUMP_STUN_INDEX, RYU_STAND_INDEX};

pub const RYU_JUMP_LIGHT_KICK: Movement = Movement {
    sprites: &[
        MovementSprite {
            sprite: Sprite {
                center: (481, 562),
                area: (463, 547, 502, 612),
            },
            damage_point: None,
            velocity_change: None,
            shield: None,
            frames: 3,
        },
        MovementSprite {
            sprite: Sprite {
                center: (529, 556),
                area: (506, 550, 570, 606),
            },
            damage_point: Some(DamagePoint {
                point: (38, 44),
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
