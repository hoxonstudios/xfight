use crate::systems::{
    drawing::Sprite,
    movement::{Movement, MovementSprite, MovementVelocityChange},
};

use super::RYU_STAND_INDEX;

pub const RYU_JUMP: Movement = Movement {
    sprites: &[
        MovementSprite {
            sprite: Sprite {
                center: (512, 47),
                area: (498, 5, 533, 97),
            },
            velocity_change: Some(MovementVelocityChange::Both(0.0, -20.0)),
            damage_point: None,
            shield: None,
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
            frames: 10,
        },
    ],
    next: Some(RYU_STAND_INDEX),
    transitions: &[],
};

pub const RYU_JUMP_LEFT: Movement = Movement {
    sprites: &[
        MovementSprite {
            sprite: Sprite {
                center: (512, 47),
                area: (498, 5, 533, 97),
            },
            velocity_change: Some(MovementVelocityChange::Both(-5.0, -20.0)),
            damage_point: None,
            shield: None,
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
            frames: 10,
        },
    ],
    next: Some(RYU_STAND_INDEX),
    transitions: &[],
};

pub const RYU_JUMP_RIGHT: Movement = Movement {
    sprites: &[
        MovementSprite {
            sprite: Sprite {
                center: (512, 47),
                area: (498, 5, 533, 97),
            },
            velocity_change: Some(MovementVelocityChange::Both(5.0, -20.0)),
            damage_point: None,
            shield: None,
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
            frames: 10,
        },
    ],
    next: Some(RYU_STAND_INDEX),
    transitions: &[],
};
