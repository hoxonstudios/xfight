use crate::{
    scenes::fight::{
        fighters::{
            ACTION_CROUCH, ACTION_CROUCH_BLOCK, ACTION_CROUCH_LIGHT_KICK,
            ACTION_CROUCH_LIGHT_PUNCH, ACTION_CROUCH_STRONG_KICK, ACTION_CROUCH_STRONG_PUNCH,
        },
        states::{STATE_DEAD, STATE_STUN},
    },
    systems::{
        drawing::Sprite,
        movement::{
            Movement, MovementSprite, MovementTransition, MovementTransitionCondition,
            MovementVelocityChange,
        },
    },
};

use super::{
    RYU_CROUCH_BLOCK_INDEX, RYU_CROUCH_LIGHT_KICK_INDEX, RYU_CROUCH_LIGHT_PUNCH_INDEX,
    RYU_CROUCH_STRONG_KICK_INDEX, RYU_CROUCH_STRONG_PUNCH_INDEX, RYU_CROUCH_STUN_INDEX,
    RYU_DEFEATED_INDEX, RYU_STAND_INDEX,
};

pub const RYU_CROUCH: Movement = Movement {
    sprites: &[MovementSprite {
        sprite: Sprite {
            center: (1181, 44),
            area: (1162, 40, 1199, 94),
        },
        velocity_change: Some(MovementVelocityChange::Horizontal(0.0)),
        damage_point: None,
        shield: None,
        spell: None,
        frames: 0,
    }],
    destroy_script: None,
    transitions: &[
        MovementTransition {
            conditions: &[MovementTransitionCondition::StateActive(STATE_DEAD)],
            movement: RYU_DEFEATED_INDEX,
            wait: false,
        },
        MovementTransition {
            conditions: &[MovementTransitionCondition::StateActive(STATE_STUN)],
            movement: RYU_CROUCH_STUN_INDEX,
            wait: false,
        },
        MovementTransition {
            conditions: &[MovementTransitionCondition::ActionActivated(
                ACTION_CROUCH_BLOCK,
            )],
            movement: RYU_CROUCH_BLOCK_INDEX,
            wait: true,
        },
        MovementTransition {
            conditions: &[MovementTransitionCondition::ActionActivated(
                ACTION_CROUCH_LIGHT_PUNCH,
            )],
            movement: RYU_CROUCH_LIGHT_PUNCH_INDEX,
            wait: true,
        },
        MovementTransition {
            conditions: &[MovementTransitionCondition::ActionActivated(
                ACTION_CROUCH_LIGHT_KICK,
            )],
            movement: RYU_CROUCH_LIGHT_KICK_INDEX,
            wait: true,
        },
        MovementTransition {
            conditions: &[MovementTransitionCondition::ActionActivated(
                ACTION_CROUCH_STRONG_PUNCH,
            )],
            movement: RYU_CROUCH_STRONG_PUNCH_INDEX,
            wait: true,
        },
        MovementTransition {
            conditions: &[MovementTransitionCondition::ActionActivated(
                ACTION_CROUCH_STRONG_KICK,
            )],
            movement: RYU_CROUCH_STRONG_KICK_INDEX,
            wait: true,
        },
        MovementTransition {
            conditions: &[MovementTransitionCondition::ActionNotActivated(
                ACTION_CROUCH,
            )],
            movement: RYU_STAND_INDEX,
            wait: true,
        },
    ],
};
