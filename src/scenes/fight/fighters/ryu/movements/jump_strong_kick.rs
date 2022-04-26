use crate::{
    scenes::fight::states::{STATE_GROUNDED, STATE_STUN},
    systems::{
        damage::{Damage, DamageArea},
        drawing::Sprite,
        movement::{Movement, MovementSprite, MovementTransition, MovementTransitionCondition},
    },
};

use super::{RYU_JUMP_STUN_INDEX, RYU_STAND_INDEX};

pub const RYU_JUMP_STRONG_KICK: Movement = Movement {
    sprites: &[MovementSprite {
        sprite: Sprite {
            center: (164, 562),
            area: (144, 529, 191, 612),
        },
        damage_point: Some(Damage {
            area: DamageArea::RelativePoint(25, -22),
            power: 5,
            tag: STATE_STUN,
        }),
        velocity_change: None,
        shield: None,
        spell: None,
        frames: 10,
    }],
    destroy_script: None,
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
