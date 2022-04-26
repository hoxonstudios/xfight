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

pub const RYU_LIGHT_PUNCH: Movement = Movement {
    sprites: &[
        MovementSprite {
            sprite: Sprite {
                center: (18, 159),
                area: (2, 130, 41, 209),
            },
            velocity_change: Some(MovementVelocityChange::Horizontal(0.0)),
            damage_point: None,
            shield: None,
            spell: None,
            frames: 4,
        },
        MovementSprite {
            sprite: Sprite {
                center: (67, 159),
                area: (51, 130, 106, 209),
            },
            damage_point: Some(Damage {
                area: DamageArea::RelativePoint(35, -11),
                power: 5,
                tag: STATE_STUN,
            }),
            velocity_change: None,
            shield: None,
            spell: None,
            frames: 4,
        },
        MovementSprite {
            sprite: Sprite {
                center: (132, 159),
                area: (116, 130, 155, 209),
            },
            velocity_change: None,
            damage_point: None,
            shield: None,
            spell: None,
            frames: 4,
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
