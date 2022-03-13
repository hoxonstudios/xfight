pub mod sprites;

use crate::systems::physics::{PhysicsComponent, RigidBody, Shape};

use self::sprites::{FLOOR_SPRITE, FLOOR_TEXTURE_PATH};

use super::FightScene;

impl<'a> FightScene<'a> {
    pub fn init_floor(&mut self) {
        let entity = self.entity;

        let texture_index = self
            .drawing
            .texture_store
            .load_texture(FLOOR_TEXTURE_PATH)
            .expect("Failed to load floor texture");

        self.physics.store.insert_component(
            entity,
            PhysicsComponent {
                entity,
                position: (400.0, 550.0),
                velocity: (0.0, 0.0),
                acceleration: (0.0, 0.0),
                rigid_body: Some(RigidBody {
                    padding: (0.0, 0.0, 0.0, 0.0),
                    solid: true,
                }),
                shape: Shape {
                    flipped: (false, false),
                    texture_index,
                    sprite: FLOOR_SPRITE,
                },
                gravity: false,
            },
        );

        self.entity += 1;
    }
}
