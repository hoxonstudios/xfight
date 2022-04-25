use crate::{
    scenes::fight::states::STATE_STUN,
    systems::{
        damage::DamagePoint,
        drawing::Sprite,
        movement::{
            Movement, MovementSprite, MovementTransition, MovementTransitionCondition,
            MovementVelocityChange,
        },
    },
};

use super::{RYU_STAND_INDEX, RYU_STUN_INDEX};

pub const RYU_LIGHT_KICK: Movement = Movement {
    sprites: &[
        MovementSprite {
            sprite: Sprite {
                center: (523, 290),
                area: (511, 258, 542, 340),
            },
            velocity_change: Some(MovementVelocityChange::Horizontal(0.0)),
            damage_point: None,
            shield: None,
            spell: None,
            frames: 3,
        },
        MovementSprite {
            sprite: Sprite {
                center: (585, 290),
                area: (569, 260, 619, 340),
            },
            damage_point: Some(DamagePoint {
                point: (27, 23),
                power: 5,
                tag: STATE_STUN,
            }),
            velocity_change: None,
            shield: None,
            spell: None,
            frames: 3,
        },
        MovementSprite {
            sprite: Sprite {
                center: (652, 290),
                area: (641, 258, 672, 340),
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
            conditions: &[MovementTransitionCondition::StateActive(STATE_STUN)],
            movement: RYU_STUN_INDEX,
            wait: false,
        },
        MovementTransition {
            conditions: &[],
            movement: RYU_STAND_INDEX,
            wait: true,
        },
    ],
};
