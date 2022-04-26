use crate::{
    scenes::fight::{
        jobs::JOB_DESTROY_RYU_ADOKEN,
        states::{STATE_IMPACTED, STATE_STUN},
    },
    systems::{
        damage::{Damage, DamageArea},
        drawing::Sprite,
        movement::{
            Movement, MovementSprite, MovementTransition, MovementTransitionCondition,
            MovementVelocityChange,
        },
    },
};

pub const RYU_ADOKEN_START_INDEX: usize = 0;
pub const RYU_ADOKEN_FLYING_INDEX: usize = 1;
pub const RYU_ADOKEN_EXPLODING_INDEX: usize = 2;
pub const RYU_ADOKEN_MOVEMENTS: &'static [Movement] = &[
    Movement {
        sprites: &[
            MovementSprite {
                sprite: Sprite {
                    center: (406, 663),
                    area: (373, 638, 409, 688),
                },
                velocity_change: Some(MovementVelocityChange::HorizontalToAim(5.0)),
                damage_point: None,
                shield: None,
                spell: None,
                frames: 3,
            },
            MovementSprite {
                sprite: Sprite {
                    center: (448, 663),
                    area: (414, 648, 453, 678),
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
                conditions: &[MovementTransitionCondition::StateActive(STATE_IMPACTED)],
                movement: RYU_ADOKEN_EXPLODING_INDEX,
                wait: false,
            },
            MovementTransition {
                conditions: &[],
                movement: RYU_ADOKEN_FLYING_INDEX,
                wait: true,
            },
        ],
    },
    Movement {
        sprites: &[
            MovementSprite {
                sprite: Sprite {
                    center: (490, 655),
                    area: (460, 642, 495, 668),
                },
                velocity_change: None,
                damage_point: None,
                shield: None,
                spell: None,
                frames: 3,
            },
            MovementSprite {
                sprite: Sprite {
                    center: (583, 692),
                    area: (553, 679, 588, 705),
                },
                velocity_change: None,
                damage_point: None,
                shield: None,
                spell: None,
                frames: 3,
            },
            MovementSprite {
                sprite: Sprite {
                    center: (513, 692),
                    area: (479, 679, 517, 705),
                },
                velocity_change: None,
                damage_point: None,
                shield: None,
                spell: None,
                frames: 3,
            },
            MovementSprite {
                sprite: Sprite {
                    center: (583, 692),
                    area: (553, 679, 588, 705),
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
                conditions: &[MovementTransitionCondition::StateActive(STATE_IMPACTED)],
                movement: RYU_ADOKEN_EXPLODING_INDEX,
                wait: false,
            },
            MovementTransition {
                conditions: &[],
                movement: RYU_ADOKEN_FLYING_INDEX,
                wait: true,
            },
        ],
    },
    Movement {
        sprites: &[MovementSprite {
            sprite: Sprite {
                center: (707, 663),
                area: (697, 649, 721, 677),
            },
            velocity_change: Some(MovementVelocityChange::Horizontal(0.0)),
            damage_point: Some(Damage {
                area: DamageArea::EntireShape,
                power: 10,
                tag: STATE_STUN,
            }),
            shield: None,
            spell: None,
            frames: 3,
        }],
        destroy_script: Some(JOB_DESTROY_RYU_ADOKEN),
        transitions: &[],
    },
];
