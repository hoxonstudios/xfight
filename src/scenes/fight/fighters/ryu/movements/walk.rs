use crate::{
    scenes::fight::{
        fighters::{
            ACTION_BLOCK, ACTION_CROUCH, ACTION_JUMP_LEFT, ACTION_JUMP_RIGHT, ACTION_LEFT,
            ACTION_LIGHT_KICK, ACTION_LIGHT_PUNCH, ACTION_RIGHT, ACTION_SPECIAL,
            ACTION_STRONG_KICK, ACTION_STRONG_PUNCH,
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
    RYU_BLOCK_INDEX, RYU_CROUCH_INDEX, RYU_DEFEATED_INDEX, RYU_JUMP_LEFT_INDEX,
    RYU_JUMP_RIGHT_INDEX, RYU_LIGHT_KICK_INDEX, RYU_LIGHT_PUNCH_INDEX, RYU_SPECIAL_ADOKEN_INDEX,
    RYU_STAND_INDEX, RYU_STRONG_KICK_INDEX, RYU_STRONG_PUNCH_INDEX, RYU_STUN_INDEX,
    RYU_WALK_LEFT_INDEX, RYU_WALK_RIGHT_INDEX, RYU_WALK_VELOCITY,
};

pub const RYU_WALK_LEFT: Movement = Movement {
    sprites: &[
        MovementSprite {
            sprite: Sprite {
                center: (221, 43),
                area: (207, 20, 237, 93),
            },
            velocity_change: Some(MovementVelocityChange::Horizontal(-RYU_WALK_VELOCITY)),
            damage_point: None,
            shield: None,
            spell: None,
            frames: 2,
        },
        MovementSprite {
            sprite: Sprite {
                center: (268, 44),
                area: (249, 15, 288, 94),
            },
            velocity_change: None,
            damage_point: None,
            shield: None,
            spell: None,
            frames: 2,
        },
        MovementSprite {
            sprite: Sprite {
                center: (317, 43),
                area: (302, 14, 341, 93),
            },
            velocity_change: None,
            damage_point: None,
            shield: None,
            spell: None,
            frames: 2,
        },
        MovementSprite {
            sprite: Sprite {
                center: (367, 43),
                area: (353, 15, 387, 93),
            },
            velocity_change: None,
            damage_point: None,
            shield: None,
            spell: None,
            frames: 2,
        },
        MovementSprite {
            sprite: Sprite {
                center: (416, 44),
                area: (402, 15, 435, 94),
            },
            velocity_change: None,
            damage_point: None,
            shield: None,
            spell: None,
            frames: 2,
        },
        MovementSprite {
            sprite: Sprite {
                center: (468, 43),
                area: (454, 20, 485, 93),
            },
            velocity_change: None,
            damage_point: None,
            shield: None,
            spell: None,
            frames: 2,
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
            conditions: &[MovementTransitionCondition::ActionActivated(
                ACTION_JUMP_LEFT,
            )],
            movement: RYU_JUMP_LEFT_INDEX,
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
            conditions: &[
                MovementTransitionCondition::ActionActivated(ACTION_LEFT),
                MovementTransitionCondition::ActionNotActivated(ACTION_RIGHT),
            ],
            movement: RYU_WALK_LEFT_INDEX,
            wait: true,
        },
        MovementTransition {
            conditions: &[
                MovementTransitionCondition::ActionActivated(ACTION_LEFT),
                MovementTransitionCondition::ActionActivated(ACTION_RIGHT),
            ],
            movement: RYU_STAND_INDEX,
            wait: false,
        },
        MovementTransition {
            conditions: &[MovementTransitionCondition::NoneAction],
            movement: RYU_STAND_INDEX,
            wait: false,
        },
    ],
};

pub const RYU_WALK_RIGHT: Movement = Movement {
    sprites: &[
        MovementSprite {
            sprite: Sprite {
                center: (221, 43),
                area: (207, 20, 237, 93),
            },
            velocity_change: Some(MovementVelocityChange::Horizontal(RYU_WALK_VELOCITY)),
            damage_point: None,
            shield: None,
            spell: None,
            frames: 2,
        },
        MovementSprite {
            sprite: Sprite {
                center: (268, 44),
                area: (249, 15, 288, 94),
            },
            velocity_change: None,
            damage_point: None,
            shield: None,
            spell: None,
            frames: 2,
        },
        MovementSprite {
            sprite: Sprite {
                center: (317, 43),
                area: (302, 14, 341, 93),
            },
            velocity_change: None,
            damage_point: None,
            shield: None,
            spell: None,
            frames: 2,
        },
        MovementSprite {
            sprite: Sprite {
                center: (367, 43),
                area: (353, 15, 387, 93),
            },
            velocity_change: None,
            damage_point: None,
            shield: None,
            spell: None,
            frames: 2,
        },
        MovementSprite {
            sprite: Sprite {
                center: (416, 44),
                area: (402, 15, 435, 94),
            },
            velocity_change: None,
            damage_point: None,
            shield: None,
            spell: None,
            frames: 2,
        },
        MovementSprite {
            sprite: Sprite {
                center: (468, 43),
                area: (454, 20, 485, 93),
            },
            velocity_change: None,
            damage_point: None,
            shield: None,
            spell: None,
            frames: 2,
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
            conditions: &[MovementTransitionCondition::ActionActivated(
                ACTION_JUMP_RIGHT,
            )],
            movement: RYU_JUMP_RIGHT_INDEX,
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
            wait: true,
        },
        MovementTransition {
            conditions: &[
                MovementTransitionCondition::ActionActivated(ACTION_LEFT),
                MovementTransitionCondition::ActionActivated(ACTION_RIGHT),
            ],
            movement: RYU_STAND_INDEX,
            wait: false,
        },
        MovementTransition {
            conditions: &[MovementTransitionCondition::NoneAction],
            movement: RYU_STAND_INDEX,
            wait: false,
        },
    ],
};
