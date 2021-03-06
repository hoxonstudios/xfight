use crate::{
    scenes::fight::states::{STATE_DEAD, STATE_STUN},
    systems::{
        damage::{Damage, DamageArea},
        drawing::Sprite,
        movement::{
            Movement, MovementSprite, MovementTransition, MovementTransitionCondition,
            MovementVelocityChange,
        },
    },
};

use super::{RYU_CROUCH_INDEX, RYU_CROUCH_STUN_INDEX, RYU_DEFEATED_INDEX};

pub const RYU_CROUCH_LIGHT_PUNCH: Movement = Movement {
    sprites: &[
        MovementSprite {
            sprite: Sprite {
                center: (30, 419),
                area: (11, 415, 53, 469),
            },
            velocity_change: Some(MovementVelocityChange::Horizontal(0.0)),
            damage_point: None,
            shield: None,
            spell: None,
            frames: 3,
        },
        MovementSprite {
            sprite: Sprite {
                center: (82, 420),
                area: (63, 415, 120, 470),
            },
            damage_point: Some(Damage {
                area: DamageArea::RelativePoint(34, 11),
                power: 3,
                tag: STATE_STUN,
            }),
            velocity_change: None,
            shield: None,
            spell: None,
            frames: 3,
        },
        MovementSprite {
            sprite: Sprite {
                center: (30, 419),
                area: (11, 415, 53, 469),
            },
            damage_point: None,
            velocity_change: None,
            shield: None,
            spell: None,
            frames: 3,
        },
    ],
    destroy_script: None,
    transitions: &[
        MovementTransition {
            conditions: &[MovementTransitionCondition::StateActive(STATE_DEAD)],
            movement: RYU_DEFEATED_INDEX,
            wait: false,
        },
        MovementTransition {
            conditions: &[MovementTransitionCondition::StateActive(STATE_STUN)],
            movement: RYU_CROUCH_STUN_INDEX,
            wait: false,
        },
        MovementTransition {
            conditions: &[],
            movement: RYU_CROUCH_INDEX,
            wait: true,
        },
    ],
};
