pub mod sprites;

use crate::{
    scenes::fight::FightScene,
    systems::{
        drawing::{ShapeComponent, ShapeTexture},
        physics::{PhysicsComponent, RigidBody},
    },
};

use self::sprites::{RYU_STAND_1, RYU_TEXTURE_PATH};

impl<'a> FightScene<'a> {
    pub fn init_ryu(&mut self, position: (f32, f32), flipped: bool) {
        let entity = self.entity;

        self.physics.store.insert_component(
            entity,
            PhysicsComponent {
                entity,
                position,
                velocity: (0.0, 0.0),
                acceleration: (0.0, 0.0),
                rigid_body: Some(RigidBody {
                    size: (25.0, 45.0),
                    solid: true,
                }),
                gravity: true,
            },
        );
        let texture_index = self
            .drawing
            .texture_store
            .load_texture(RYU_TEXTURE_PATH)
            .expect("Failed to load Ryu texture");

        self.drawing.store.insert_component(
            entity,
            ShapeComponent {
                entity,
                size: (50, 90),
                flipped: (flipped, false),
                texture: ShapeTexture {
                    texture_index,
                    sprite: RYU_STAND_1,
                },
            },
        );

        self.entity += 1;
    }
}
