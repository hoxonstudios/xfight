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

use super::{RYU_CROUCH_INDEX, RYU_CROUCH_STUN_INDEX};

pub const RYU_CROUCH_STRONG_KICK: Movement = Movement {
    sprites: &[
        MovementSprite {
            sprite: Sprite {
                center: (867, 421),
                area: (849, 414, 892, 471),
            },
            velocity_change: Some(MovementVelocityChange::Horizontal(0.0)),
            damage_point: None,
            shield: None,
            spell: None,
            frames: 4,
        },
        MovementSprite {
            sprite: Sprite {
                center: (929, 419),
                area: (902, 422, 986, 469),
            },
            velocity_change: None,
            shield: None,
            damage_point: Some(DamagePoint {
                point: (51, 45),
                power: 5,
                tag: STATE_STUN,
            }),
            spell: None,
            frames: 4,
        },
        MovementSprite {
            sprite: Sprite {
                center: (867, 421),
                area: (849, 414, 892, 471),
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
