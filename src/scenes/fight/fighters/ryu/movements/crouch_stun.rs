use crate::{
    scenes::fight::fighters::ACTION_CROUCH,
    systems::{
        drawing::Sprite,
        movement::{
            Movement, MovementSprite, MovementTransition, MovementTransitionCondition,
            MovementVelocityChange,
        },
    },
};

use super::{RYU_CROUCH_INDEX, RYU_STAND_INDEX};

pub const RYU_CROUCH_STUN: Movement = Movement {
    sprites: &[MovementSprite {
        sprite: Sprite {
            center: (470, 783),
            area: (446, 770, 487, 833),
        },
        velocity_change: Some(MovementVelocityChange::HorizontalToAim(-4.0)),
        damage_point: None,
        shield: None,
        spell: None,
        frames: 10,
    }],
    destroy_script: None,
    transitions: &[
        MovementTransition {
            conditions: &[MovementTransitionCondition::ActionActivated(ACTION_CROUCH)],
            movement: RYU_CROUCH_INDEX,
            wait: true,
        },
        MovementTransition {
            conditions: &[],
            movement: RYU_STAND_INDEX,
            wait: true,
        },
    ],
};
