use crate::{
    scenes::fight::states::STATE_DEAD,
    systems::{
        drawing::Sprite,
        movement::{
            Movement, MovementSprite, MovementTransition, MovementTransitionCondition,
            MovementVelocityChange,
        },
    },
};

use super::{RYU_DEFEATED_INDEX, RYU_STAND_INDEX};

pub const RYU_STUN: Movement = Movement {
    sprites: &[
        MovementSprite {
            sprite: Sprite {
                center: (177, 780),
                area: (160, 750, 201, 830),
            },
            velocity_change: Some(MovementVelocityChange::HorizontalToAim(-1.5)),
            damage_point: None,
            shield: None,
            spell: None,
            frames: 3,
        },
        MovementSprite {
            sprite: Sprite {
                center: (233, 781),
                area: (212, 753, 253, 831),
            },
            velocity_change: None,
            damage_point: None,
            shield: None,
            spell: None,
            frames: 3,
        },
        MovementSprite {
            sprite: Sprite {
                center: (292, 781),
                area: (266, 753, 310, 831),
            },
            velocity_change: None,
            damage_point: None,
            shield: None,
            spell: None,
            frames: 3,
        },
        MovementSprite {
            sprite: Sprite {
                center: (353, 781),
                area: (323, 749, 372, 831),
            },
            velocity_change: None,
            damage_point: None,
            shield: None,
            spell: None,
            frames: 3,
        },
        MovementSprite {
            sprite: Sprite {
                center: (409, 780),
                area: (388, 750, 429, 830),
            },
            velocity_change: Some(MovementVelocityChange::Horizontal(0.0)),
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
            conditions: &[],
            movement: RYU_STAND_INDEX,
            wait: true,
        },
    ],
};
