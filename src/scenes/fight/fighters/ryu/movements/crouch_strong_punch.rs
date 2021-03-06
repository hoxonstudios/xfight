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

pub const RYU_CROUCH_STRONG_PUNCH: Movement = Movement {
    sprites: &[
        MovementSprite {
            sprite: Sprite {
                center: (148, 419),
                area: (129, 415, 171, 469),
            },
            velocity_change: Some(MovementVelocityChange::Horizontal(0.0)),
            damage_point: None,
            shield: None,
            spell: None,
            frames: 3,
        },
        MovementSprite {
            sprite: Sprite {
                center: (202, 419),
                area: (183, 415, 223, 469),
            },
            velocity_change: None,
            damage_point: None,
            shield: None,
            spell: None,
            frames: 3,
        },
        MovementSprite {
            sprite: Sprite {
                center: (254, 419),
                area: (235, 414, 290, 469),
            },
            damage_point: Some(Damage {
                area: DamageArea::RelativePoint(33, 9),
                power: 6,
                tag: STATE_STUN,
            }),
            velocity_change: None,
            shield: None,
            spell: None,
            frames: 3,
        },
        MovementSprite {
            sprite: Sprite {
                center: (202, 419),
                area: (183, 415, 223, 469),
            },
            damage_point: None,
            velocity_change: None,
            shield: None,
            spell: None,
            frames: 3,
        },
        MovementSprite {
            sprite: Sprite {
                center: (148, 419),
                area: (129, 415, 171, 469),
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
