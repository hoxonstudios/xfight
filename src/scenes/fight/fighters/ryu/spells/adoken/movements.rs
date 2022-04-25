use crate::{
    scenes::fight::{
        jobs::JOB_DESTROY_RYU_ADOKEN,
        states::{STATE_EXPLODE, STATE_STUN},
    },
    systems::{
        damage::DamagePoint,
        drawing::Sprite,
        movement::{
            Movement, MovementSprite, MovementTransition, MovementTransitionCondition,
            MovementVelocityChange,
        },
    },
};

pub const RYU_ADOKEN_FLYING_INDEX: usize = 0;
pub const RYU_ADOKEN_EXPLODING_INDEX: usize = 1;
pub const RYU_ADOKEN_MOVEMENTS: &'static [Movement] = &[
    Movement {
        sprites: &[MovementSprite {
            sprite: Sprite {
                center: (392, 663),
                area: (373, 638, 409, 688),
            },
            velocity_change: Some(MovementVelocityChange::HorizontalToAim(5.0)),
            damage_point: Some(DamagePoint {
                point: (25, 0),
                power: 10,
                tag: STATE_STUN,
            }),
            shield: None,
            spell: None,
            frames: 30,
        }],
        destroy_script: None,
        transitions: &[MovementTransition {
            conditions: &[MovementTransitionCondition::StateActive(STATE_EXPLODE)],
            movement: RYU_ADOKEN_EXPLODING_INDEX,
            wait: false,
        }],
    },
    Movement {
        sprites: &[MovementSprite {
            sprite: Sprite {
                center: (707, 663),
                area: (697, 649, 721, 677),
            },
            velocity_change: Some(MovementVelocityChange::Horizontal(0.0)),
            damage_point: None,
            shield: None,
            spell: None,
            frames: 3,
        }],
        destroy_script: Some(JOB_DESTROY_RYU_ADOKEN),
        transitions: &[],
    },
];
