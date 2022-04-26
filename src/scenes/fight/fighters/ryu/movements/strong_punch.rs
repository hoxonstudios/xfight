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
            spell: None,
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
            spell: None,
            frames: 3,
        },
        MovementSprite {
            sprite: Sprite {
                center: (300, 158),
                area: (279, 126, 343, 208),
            },
            damage_point: Some(Damage {
                area: DamageArea::RelativePoint(39, -13),
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
                center: (379, 158),
                area: (355, 126, 401, 208),
            },
            damage_point: None,
            velocity_change: None,
            shield: None,
            spell: None,
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
