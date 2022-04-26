use crate::{
    scenes::fight::states::STATE_STUN,
    systems::{
        damage::{Damage, DamageArea},
        drawing::Sprite,
        movement::{
            Movement, MovementSprite, MovementTransition, MovementTransitionCondition,
            MovementVelocityChange,
        },
    },
};

use super::{RYU_STAND_INDEX, RYU_STUN_INDEX};

pub const RYU_STRONG_KICK: Movement = Movement {
    sprites: &[
        MovementSprite {
            sprite: Sprite {
                center: (23, 288),
                area: (1, 257, 42, 338),
            },
            velocity_change: Some(MovementVelocityChange::Horizontal(0.0)),
            damage_point: None,
            shield: None,
            spell: None,
            frames: 5,
        },
        MovementSprite {
            sprite: Sprite {
                center: (84, 288),
                area: (57, 255, 126, 338),
            },
            damage_point: Some(Damage {
                area: DamageArea::RelativePoint(36, -27),
                power: 5,
                tag: STATE_STUN,
            }),
            velocity_change: None,
            shield: None,
            spell: None,
            frames: 5,
        },
        MovementSprite {
            sprite: Sprite {
                center: (152, 288),
                area: (130, 257, 171, 338),
            },
            velocity_change: None,
            damage_point: None,
            shield: None,
            spell: None,
            frames: 5,
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
