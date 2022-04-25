use crate::{
    scenes::fight::fighters::ACTION_BLOCK,
    systems::{
        drawing::Sprite,
        health::Shield,
        movement::{
            Movement, MovementSprite, MovementTransition, MovementTransitionCondition,
            MovementVelocityChange,
        },
    },
};

use super::RYU_CROUCH_INDEX;

pub const RYU_CROUCH_BLOCK: Movement = Movement {
    sprites: &[MovementSprite {
        sprite: Sprite {
            center: (1278, 45),
            area: (1259, 34, 1296, 95),
        },
        shield: Some(Shield {
            x0: -19,
            y0: -11,
            x1: 18,
            y1: 50,
            reduction: 0.75,
        }),
        velocity_change: Some(MovementVelocityChange::Horizontal(0.0)),
        damage_point: None,
        frames: 0,
    }],
    transitions: &[MovementTransition {
        conditions: &[MovementTransitionCondition::ActionNotActivated(
            ACTION_BLOCK,
        )],
        movement: RYU_CROUCH_INDEX,
        wait: true,
    }],
};