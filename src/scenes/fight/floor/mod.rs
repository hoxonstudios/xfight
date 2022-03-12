pub mod sprites;

use crate::systems::{
    drawing::ShapeComponent,
    physics::{PhysicsComponent, RigidBody},
};

use self::sprites::FLOOR_SPRITE;

use super::FightScene;

impl<'a> FightScene<'a> {
    pub fn init_floor(&mut self) {
        let entity = self.entity;
        self.physics.store.insert_component(
            entity,
            PhysicsComponent {
                entity,
                position: (400.0, 550.0),
                velocity: (0.0, 0.0),
                acceleration: (0.0, 0.0),
                rigid_body: Some(RigidBody {
                    size: (400.0, 50.0),
                }),
                gravity: false,
            },
        );
        self.drawing.store.insert_component(
            entity,
            ShapeComponent {
                entity,
                size: (800, 100),
                flipped: (false, false),
                texture: FLOOR_SPRITE,
            },
        );

        self.entity += 1;
    }
}
