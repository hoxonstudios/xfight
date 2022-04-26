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

use super::RYU_STAND_INDEX;

pub const RYU_BLOCK: Movement = Movement {
    sprites: &[MovementSprite {
        sprite: Sprite {
            center: (1229, 44),
            area: (1210, 12, 1249, 94),
        },
        shield: Some(Shield {
            x0: -19,
            y0: -32,
            x1: 20,
            y1: 0,
            reduction: 0.25,
        }),
        velocity_change: Some(MovementVelocityChange::Horizontal(0.0)),
        damage_point: None,
        spell: None,
        frames: 0,
    }],
    destroy_script: None,
    transitions: &[MovementTransition {
        conditions: &[MovementTransitionCondition::ActionNotActivated(
            ACTION_BLOCK,
        )],
        movement: RYU_STAND_INDEX,
        wait: true,
    }],
};
