use crate::{
    scenes::fight::{
        fighters::{
            ACTION_LIGHT_KICK, ACTION_LIGHT_PUNCH, ACTION_STRONG_KICK, ACTION_STRONG_PUNCH,
        },
        states::{STATE_GROUNDED, STATE_STUN},
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
    RYU_JUMP_LIGHT_KICK_INDEX, RYU_JUMP_LIGHT_PUNCH_INDEX, RYU_JUMP_STRONG_KICK_INDEX,
    RYU_JUMP_STRONG_PUNCH_INDEX, RYU_JUMP_STUN_INDEX, RYU_STAND_INDEX,
};

pub const RYU_JUMP: Movement = Movement {
    sprites: &[
        MovementSprite {
            sprite: Sprite {
                center: (512, 47),
                area: (498, 5, 533, 97),
            },
            velocity_change: Some(MovementVelocityChange::Both(0.0, -17.0)),
            damage_point: None,
            shield: None,
            spell: None,
            frames: 9,
        },
        MovementSprite {
            sprite: Sprite {
                center: (554, 43),
                area: (540, 13, 571, 93),
            },
            velocity_change: None,
            damage_point: None,
            shield: None,
            spell: None,
            frames: 9,
        },
        MovementSprite {
            sprite: Sprite {
                center: (591, 34),
                area: (577, 15, 610, 84),
            },
            velocity_change: None,
            damage_point: None,
            shield: None,
            spell: None,
            frames: 9,
        },
        MovementSprite {
            sprite: Sprite {
                center: (628, 43),
                area: (614, 13, 645, 93),
            },
            velocity_change: None,
            damage_point: None,
            shield: None,
            spell: None,
            frames: 7,
        },
    ],
    destroy_script: None,
    transitions: &[
        MovementTransition {
            conditions: &[MovementTransitionCondition::StateActive(STATE_STUN)],
            movement: RYU_JUMP_STUN_INDEX,
            wait: false,
        },
        MovementTransition {
            conditions: &[MovementTransitionCondition::StateActive(STATE_GROUNDED)],
            movement: RYU_STAND_INDEX,
            wait: true,
        },
        MovementTransition {
            conditions: &[MovementTransitionCondition::ActionActivated(
                ACTION_LIGHT_PUNCH,
            )],
            movement: RYU_JUMP_LIGHT_PUNCH_INDEX,
            wait: false,
        },
        MovementTransition {
            conditions: &[MovementTransitionCondition::ActionActivated(
                ACTION_LIGHT_KICK,
            )],
            movement: RYU_JUMP_LIGHT_KICK_INDEX,
            wait: false,
        },
        MovementTransition {
            conditions: &[MovementTransitionCondition::ActionActivated(
                ACTION_STRONG_PUNCH,
            )],
            movement: RYU_JUMP_STRONG_PUNCH_INDEX,
            wait: false,
        },
        MovementTransition {
            conditions: &[MovementTransitionCondition::ActionActivated(
                ACTION_STRONG_KICK,
            )],
            movement: RYU_JUMP_STRONG_KICK_INDEX,
            wait: false,
        },
    ],
};

pub const RYU_JUMP_LEFT: Movement = Movement {
    sprites: &[
        MovementSprite {
            sprite: Sprite {
                center: (512, 47),
                area: (498, 5, 533, 97),
            },
            velocity_change: Some(MovementVelocityChange::Both(-4.0, -17.0)),
            damage_point: None,
            shield: None,
            spell: None,
            frames: 10,
        },
        MovementSprite {
            sprite: Sprite {
                center: (554, 43),
                area: (540, 13, 571, 93),
            },
            velocity_change: None,
            damage_point: None,
            shield: None,
            spell: None,
            frames: 10,
        },
        MovementSprite {
            sprite: Sprite {
                center: (591, 34),
                area: (577, 15, 610, 84),
            },
            velocity_change: None,
            damage_point: None,
            shield: None,
            spell: None,
            frames: 10,
        },
        MovementSprite {
            sprite: Sprite {
                center: (628, 43),
                area: (614, 13, 645, 93),
            },
            velocity_change: None,
            damage_point: None,
            shield: None,
            spell: None,
            frames: 10,
        },
    ],
    destroy_script: None,
    transitions: &[
        MovementTransition {
            conditions: &[MovementTransitionCondition::StateActive(STATE_STUN)],
            movement: RYU_JUMP_STUN_INDEX,
            wait: false,
        },
        MovementTransition {
            conditions: &[MovementTransitionCondition::StateActive(STATE_GROUNDED)],
            movement: RYU_STAND_INDEX,
            wait: true,
        },
        MovementTransition {
            conditions: &[MovementTransitionCondition::ActionActivated(
                ACTION_LIGHT_PUNCH,
            )],
            movement: RYU_JUMP_LIGHT_PUNCH_INDEX,
            wait: false,
        },
        MovementTransition {
            conditions: &[MovementTransitionCondition::ActionActivated(
                ACTION_LIGHT_KICK,
            )],
            movement: RYU_JUMP_LIGHT_KICK_INDEX,
            wait: false,
        },
        MovementTransition {
            conditions: &[MovementTransitionCondition::ActionActivated(
                ACTION_STRONG_PUNCH,
            )],
            movement: RYU_JUMP_STRONG_PUNCH_INDEX,
            wait: false,
        },
        MovementTransition {
            conditions: &[MovementTransitionCondition::ActionActivated(
                ACTION_STRONG_KICK,
            )],
            movement: RYU_JUMP_STRONG_KICK_INDEX,
            wait: false,
        },
    ],
};

pub const RYU_JUMP_RIGHT: Movement = Movement {
    sprites: &[
        MovementSprite {
            sprite: Sprite {
                center: (512, 47),
                area: (498, 5, 533, 97),
            },
            velocity_change: Some(MovementVelocityChange::Both(4.0, -17.0)),
            damage_point: None,
            shield: None,
            spell: None,
            frames: 10,
        },
        MovementSprite {
            sprite: Sprite {
                center: (554, 43),
                area: (540, 13, 571, 93),
            },
            velocity_change: None,
            damage_point: None,
            shield: None,
            spell: None,
            frames: 10,
        },
        MovementSprite {
            sprite: Sprite {
                center: (591, 34),
                area: (577, 15, 610, 84),
            },
            velocity_change: None,
            damage_point: None,
            shield: None,
            spell: None,
            frames: 10,
        },
        MovementSprite {
            sprite: Sprite {
                center: (628, 43),
                area: (614, 13, 645, 93),
            },
            velocity_change: None,
            damage_point: None,
            shield: None,
            spell: None,
            frames: 10,
        },
    ],
    destroy_script: None,
    transitions: &[
        MovementTransition {
            conditions: &[MovementTransitionCondition::StateActive(STATE_STUN)],
            movement: RYU_JUMP_STUN_INDEX,
            wait: false,
        },
        MovementTransition {
            conditions: &[MovementTransitionCondition::StateActive(STATE_GROUNDED)],
            movement: RYU_STAND_INDEX,
            wait: true,
        },
        MovementTransition {
            conditions: &[MovementTransitionCondition::ActionActivated(
                ACTION_LIGHT_PUNCH,
            )],
            movement: RYU_JUMP_LIGHT_PUNCH_INDEX,
            wait: false,
        },
        MovementTransition {
            conditions: &[MovementTransitionCondition::ActionActivated(
                ACTION_LIGHT_KICK,
            )],
            movement: RYU_JUMP_LIGHT_KICK_INDEX,
            wait: false,
        },
        MovementTransition {
            conditions: &[MovementTransitionCondition::ActionActivated(
                ACTION_STRONG_PUNCH,
            )],
            movement: RYU_JUMP_STRONG_PUNCH_INDEX,
            wait: false,
        },
        MovementTransition {
            conditions: &[MovementTransitionCondition::ActionActivated(
                ACTION_STRONG_KICK,
            )],
            movement: RYU_JUMP_STRONG_KICK_INDEX,
            wait: false,
        },
    ],
};
