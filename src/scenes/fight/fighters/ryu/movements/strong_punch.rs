use crate::{
    scenes::fight::fighters::STATE_STUN,
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

pub const RYU_STRONG_PUNCH: Movement = Movement {
    sprites: &[
        MovementSprite {
            sprite: Sprite {
                center: (185, 159),
                area: (169, 130, 208, 209),
            },
            velocity_change: Some(MovementVelocityChange::Horizontal(0.0)),
            damage_point: None,
            shield: None,
            frames: 3,
        },
        MovementSprite {
            sprite: Sprite {
                center: (244, 158),
                area: (220, 125, 261, 208),
            },
            velocity_change: None,
            damage_point: None,
            shield: None,
            frames: 3,
        },
        MovementSprite {
            sprite: Sprite {
                center: (300, 158),
                area: (279, 126, 343, 208),
            },
            damage_point: Some(DamagePoint {
                point: (39, -13),
                power: 5,
                tag: STATE_STUN,
            }),
            velocity_change: None,
            shield: None,
            frames: 3,
        },
        MovementSprite {
            sprite: Sprite {
                center: (379, 158),
                area: (355, 126, 401, 208),
            },
            damage_point: None,
            velocity_change: None,
            shield: None,
            frames: 3,
        },
        MovementSprite {
            sprite: Sprite {
                center: (426, 159),
                area: (410, 130, 449, 209),
            },
            damage_point: None,
            velocity_change: None,
            shield: None,
            frames: 3,
        },
    ],
    next: Some(RYU_STAND_INDEX),
    transitions: &[MovementTransition {
        conditions: &[MovementTransitionCondition::StateActive(STATE_STUN)],
        movement: RYU_STUN_INDEX,
    }],
};
