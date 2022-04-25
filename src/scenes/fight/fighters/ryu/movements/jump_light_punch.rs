use crate::{
    scenes::fight::fighters::{STATE_GROUNDED, STATE_STUN},
    systems::{
        damage::DamagePoint,
        drawing::Sprite,
        movement::{Movement, MovementSprite, MovementTransition, MovementTransitionCondition},
    },
};

use super::{RYU_JUMP_STUN_INDEX, RYU_STAND_INDEX};

pub const RYU_JUMP_LIGHT_PUNCH: Movement = Movement {
    sprites: &[MovementSprite {
        sprite: Sprite {
            center: (434, 555),
            area: (416, 550, 457, 605),
        },
        damage_point: Some(DamagePoint {
            point: (21, 41),
            power: 5,
            tag: STATE_STUN,
        }),
        velocity_change: None,
        shield: None,
        frames: 10,
    }],
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
