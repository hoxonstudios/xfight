use crate::{
    scenes::fight::{
        jobs::JOB_SPAWN_RYU_ADOKEN,
        states::{STATE_DEAD, STATE_STUN},
    },
    systems::{
        damage::{Damage, DamageArea},
        drawing::Sprite,
        movement::{
            Movement, MovementSpellScript, MovementSprite, MovementTransition,
            MovementTransitionCondition, MovementVelocityChange,
        },
    },
};

use super::{RYU_DEFEATED_INDEX, RYU_STAND_INDEX, RYU_STUN_INDEX};

pub const RYU_SPECIAL_ADOKEN: Movement = Movement {
    sprites: &[
        MovementSprite {
            sprite: Sprite {
                center: (30, 663),
                area: (7, 632, 52, 713),
            },
            velocity_change: Some(MovementVelocityChange::Horizontal(0.0)),
            damage_point: None,
            shield: None,
            spell: None,
            frames: 7,
        },
        MovementSprite {
            sprite: Sprite {
                center: (84, 662),
                area: (62, 638, 114, 712),
            },
            velocity_change: None,
            damage_point: None,
            shield: None,
            spell: None,
            frames: 7,
        },
        MovementSprite {
            sprite: Sprite {
                center: (162, 662),
                area: (134, 639, 187, 712),
            },
            velocity_change: None,
            damage_point: None,
            shield: None,
            spell: None,
            frames: 7,
        },
        MovementSprite {
            sprite: Sprite {
                center: (233, 662),
                area: (205, 644, 290, 712),
            },
            damage_point: Some(Damage {
                area: DamageArea::RelativePoint(54, 2),
                power: 8,
                tag: STATE_STUN,
            }),
            velocity_change: None,
            shield: None,
            spell: None,
            frames: 7,
        },
        MovementSprite {
            sprite: Sprite {
                center: (330, 662),
                area: (302, 644, 366, 712),
            },
            damage_point: None,
            velocity_change: None,
            shield: None,
            spell: Some(MovementSpellScript {
                script: JOB_SPAWN_RYU_ADOKEN,
                position: (76.0, 0.0),
            }),
            frames: 35,
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
            conditions: &[],
            movement: RYU_STAND_INDEX,
            wait: true,
        },
    ],
};
