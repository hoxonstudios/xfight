use crate::{
    scenes::fight::fighters::STATE_GROUNDED,
    systems::{
        drawing::Sprite,
        movement::{
            Movement, MovementSprite, MovementTransition, MovementTransitionCondition,
            MovementVelocityChange,
        },
    },
};

use super::RYU_STAND_INDEX;

pub const RYU_JUMP_STUN: Movement = Movement {
    sprites: &[MovementSprite {
        sprite: Sprite {
            center: (533, 779),
            area: (506, 763, 555, 829),
        },
        velocity_change: Some(MovementVelocityChange::HorizontalToAim(-4.0)),
        damage_point: None,
        shield: None,
        frames: 15,
    }],
    transitions: &[MovementTransition {
        conditions: &[MovementTransitionCondition::StateActive(STATE_GROUNDED)],
        movement: RYU_STAND_INDEX,
        wait: false,
    }],
};
