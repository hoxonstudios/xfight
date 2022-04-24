use crate::{
    scenes::fight::fighters::STATE_STUN,
    systems::{
        damage::DamagePoint,
        drawing::Sprite,
        movement::{Movement, MovementSprite, MovementVelocityChange},
    },
};

use super::RYU_CROUCH_INDEX;

pub const RYU_CROUCH_LIGHT_KICK: Movement = Movement {
    sprites: &[
        MovementSprite {
            sprite: Sprite {
                center: (682, 421),
                area: (663, 414, 706, 471),
            },
            velocity_change: Some(MovementVelocityChange::Horizontal(0.0)),
            damage_point: None,
            shield: None,
            frames: 3,
        },
        MovementSprite {
            sprite: Sprite {
                center: (735, 422),
                area: (716, 413, 782, 472),
            },
            damage_point: Some(DamagePoint {
                point: (40, 42),
                power: 5,
                tag: STATE_STUN,
            }),
            velocity_change: None,
            shield: None,
            frames: 3,
        },
        MovementSprite {
            sprite: Sprite {
                center: (682, 421),
                area: (663, 414, 706, 471),
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
