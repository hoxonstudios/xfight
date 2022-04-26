use crate::{
    scenes::fight::{
        fighters::{
            ACTION_BLOCK, ACTION_CROUCH, ACTION_JUMP, ACTION_LEFT, ACTION_LIGHT_KICK,
            ACTION_LIGHT_PUNCH, ACTION_RIGHT, ACTION_SPECIAL, ACTION_STRONG_KICK,
            ACTION_STRONG_PUNCH,
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
    RYU_BLOCK_INDEX, RYU_CROUCH_INDEX, RYU_DEFEATED_INDEX, RYU_JUMP_INDEX, RYU_LIGHT_KICK_INDEX,
    RYU_LIGHT_PUNCH_INDEX, RYU_SPECIAL_ADOKEN_INDEX, RYU_STAND_INDEX, RYU_STRONG_KICK_INDEX,
    RYU_STRONG_PUNCH_INDEX, RYU_STUN_INDEX, RYU_WALK_LEFT_INDEX, RYU_WALK_RIGHT_INDEX,
};

pub const RYU_STAND: Movement = Movement {
    sprites: &[
        MovementSprite {
            sprite: Sprite {
                center: (19, 42),
                area: (4, 14, 40, 92),
            },
            velocity_change: Some(MovementVelocityChange::Horizontal(0.0)),
            damage_point: None,
            shield: None,
            spell: None,
            frames: 3,
        },
        MovementSprite {
            sprite: Sprite {
                center: (68, 42),
                area: (52, 15, 89, 92),
            },
            velocity_change: None,
            damage_point: None,
            shield: None,
            spell: None,
            frames: 3,
        },
        MovementSprite {
            sprite: Sprite {
                center: (118, 43),
                area: (103, 14, 138, 93),
            },
            velocity_change: None,
            damage_point: None,
            shield: None,
            spell: None,
            frames: 3,
        },
        MovementSprite {
            sprite: Sprite {
                center: (167, 42),
                area: (152, 13, 187, 92),
            },
            velocity_change: None,
            damage_point: None,
            shield: None,
            spell: None,
            frames: 3,
        },
    ],
    destroy_script: None,
    transitions: &[
        MovementTransition {
            conditions: &[MovementTransitionCondition::StateActive(STATE_DEAD)],
            movement: RYU_DEFEATED_INDEX,
            wait: false,
        },
        MovementTransition {
            conditions: &[MovementTransitionCondition::StateActive(STATE_STUN)],
            movement: RYU_STUN_INDEX,
            wait: false,
        },
        MovementTransition {
            conditions: &[MovementTransitionCondition::ActionActivated(ACTION_CROUCH)],
            movement: RYU_CROUCH_INDEX,
            wait: false,
        },
        MovementTransition {
            conditions: &[MovementTransitionCondition::ActionActivated(ACTION_BLOCK)],
            movement: RYU_BLOCK_INDEX,
            wait: false,
        },
        MovementTransition {
            conditions: &[MovementTransitionCondition::ActionActivated(ACTION_JUMP)],
            movement: RYU_JUMP_INDEX,
            wait: false,
        },
        MovementTransition {
            conditions: &[MovementTransitionCondition::ActionActivated(
                ACTION_LIGHT_PUNCH,
            )],
            movement: RYU_LIGHT_PUNCH_INDEX,
            wait: false,
        },
        MovementTransition {
            conditions: &[MovementTransitionCondition::ActionActivated(
                ACTION_LIGHT_KICK,
            )],
            movement: RYU_LIGHT_KICK_INDEX,
            wait: false,
        },
        MovementTransition {
            conditions: &[MovementTransitionCondition::ActionActivated(
                ACTION_STRONG_PUNCH,
            )],
            movement: RYU_STRONG_PUNCH_INDEX,
            wait: false,
        },
        MovementTransition {
            conditions: &[MovementTransitionCondition::ActionActivated(
                ACTION_STRONG_KICK,
            )],
            movement: RYU_STRONG_KICK_INDEX,
            wait: false,
        },
        MovementTransition {
            conditions: &[MovementTransitionCondition::ActionActivated(ACTION_SPECIAL)],
            movement: RYU_SPECIAL_ADOKEN_INDEX,
            wait: false,
        },
        MovementTransition {
            conditions: &[
                MovementTransitionCondition::ActionActivated(ACTION_LEFT),
                MovementTransitionCondition::ActionNotActivated(ACTION_RIGHT),
            ],
            movement: RYU_WALK_LEFT_INDEX,
            wait: false,
        },
        MovementTransition {
            conditions: &[
                MovementTransitionCondition::ActionActivated(ACTION_RIGHT),
                MovementTransitionCondition::ActionNotActivated(ACTION_LEFT),
            ],
            movement: RYU_WALK_RIGHT_INDEX,
            wait: false,
        },
        MovementTransition {
            conditions: &[],
            movement: RYU_STAND_INDEX,
            wait: true,
        },
    ],
};
