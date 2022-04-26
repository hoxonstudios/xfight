use crate::systems::{
    drawing::Sprite,
    movement::{Movement, MovementSprite, MovementVelocityChange},
};

pub const RYU_DEFEATED: Movement = Movement {
    sprites: &[
        MovementSprite {
            sprite: Sprite {
                center: (1361, 28),
                area: (1336, 0, 1385, 63),
            },
            velocity_change: Some(MovementVelocityChange::Both(0.0, 0.0)),
            damage_point: None,
            shield: None,
            spell: None,
            frames: 5,
        },
        MovementSprite {
            sprite: Sprite {
                center: (1424, 28),
                area: (1396, 0, 1452, 63),
            },
            velocity_change: Some(MovementVelocityChange::Both(0.0, 0.0)),
            damage_point: None,
            shield: None,
            spell: None,
            frames: 5,
        },
        MovementSprite {
            sprite: Sprite {
                center: (1489, 28),
                area: (1457, 0, 1517, 63),
            },
            velocity_change: Some(MovementVelocityChange::Both(0.0, 0.0)),
            damage_point: None,
            shield: None,
            spell: None,
            frames: 5,
        },
        MovementSprite {
            sprite: Sprite {
                center: (1361, 91),
                area: (1336, 63, 1385, 126),
            },
            velocity_change: Some(MovementVelocityChange::Both(0.0, 0.0)),
            damage_point: None,
            shield: None,
            spell: None,
            frames: 5,
        },
        MovementSprite {
            sprite: Sprite {
                center: (1424, 91),
                area: (1396, 63, 1452, 126),
            },
            velocity_change: Some(MovementVelocityChange::Both(0.0, 0.0)),
            damage_point: None,
            shield: None,
            spell: None,
            frames: 5,
        },
        MovementSprite {
            sprite: Sprite {
                center: (1489, 91),
                area: (1457, 63, 1517, 126),
            },
            velocity_change: Some(MovementVelocityChange::Both(0.0, 0.0)),
            damage_point: None,
            shield: None,
            spell: None,
            frames: 5,
        },
        MovementSprite {
            sprite: Sprite {
                center: (1361, 154),
                area: (1336, 126, 1385, 189),
            },
            velocity_change: Some(MovementVelocityChange::Both(0.0, 0.0)),
            damage_point: None,
            shield: None,
            spell: None,
            frames: 5,
        },
        MovementSprite {
            sprite: Sprite {
                center: (1424, 154),
                area: (1396, 126, 1452, 189),
            },
            velocity_change: Some(MovementVelocityChange::Both(0.0, 0.0)),
            damage_point: None,
            shield: None,
            spell: None,
            frames: 5,
        },
        MovementSprite {
            sprite: Sprite {
                center: (1489, 154),
                area: (1457, 126, 1517, 189),
            },
            velocity_change: Some(MovementVelocityChange::Both(0.0, 0.0)),
            damage_point: None,
            shield: None,
            spell: None,
            frames: 5,
        },
        MovementSprite {
            sprite: Sprite {
                center: (1361, 217),
                area: (1336, 189, 1385, 252),
            },
            velocity_change: Some(MovementVelocityChange::Both(0.0, 0.0)),
            damage_point: None,
            shield: None,
            spell: None,
            frames: 5,
        },
        MovementSprite {
            sprite: Sprite {
                center: (1424, 217),
                area: (1396, 189, 1452, 252),
            },
            velocity_change: Some(MovementVelocityChange::Both(0.0, 0.0)),
            damage_point: None,
            shield: None,
            spell: None,
            frames: 5,
        },
        MovementSprite {
            sprite: Sprite {
                center: (1489, 217),
                area: (1457, 189, 1517, 252),
            },
            velocity_change: Some(MovementVelocityChange::Both(0.0, 0.0)),
            damage_point: None,
            shield: None,
            spell: None,
            frames: 5,
        },
        MovementSprite {
            sprite: Sprite {
                center: (1489, 280),
                area: (1457, 252, 1517, 315),
            },
            velocity_change: Some(MovementVelocityChange::Both(0.0, 0.0)),
            damage_point: None,
            shield: None,
            spell: None,
            frames: 0,
        },
    ],
    destroy_script: None,
    transitions: &[],
};
