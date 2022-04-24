use crate::{
    scenes::fight::fighters::STATE_STUN,
    systems::{
        damage::DamagePoint,
        drawing::Sprite,
        movement::{Movement, MovementSprite, MovementVelocityChange},
    },
};

use super::RYU_CROUCH_INDEX;

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
            frames: 3,
        },
        MovementSprite {
            sprite: Sprite {
                center: (82, 420),
                area: (63, 415, 120, 470),
            },
            damage_point: Some(DamagePoint {
                point: (34, 11),
                power: 5,
                tag: STATE_STUN,
            }),
            velocity_change: None,
            shield: None,
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
            frames: 3,
        },
    ],
    next: Some(RYU_CROUCH_INDEX),
    transitions: &[],
};
