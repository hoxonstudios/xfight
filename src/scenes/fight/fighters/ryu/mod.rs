pub mod sprites;

use crate::{
    scenes::fight::FightScene,
    systems::{
        drawing::{ShapeComponent, ShapeTexture},
        input::{Controller, InputComponent},
        movement::{AimDirection, MovementAction, MovementComponent},
        physics::{PhysicsComponent, RigidBody},
        stand::StandComponent,
        walking::WalkingComponent,
    },
};

use self::sprites::{
    RYU_STAND_1, RYU_STAND_2, RYU_STAND_3, RYU_STAND_4, RYU_TEXTURE_PATH, RYU_WALKING_1,
    RYU_WALKING_2, RYU_WALKING_3, RYU_WALKING_4, RYU_WALKING_5, RYU_WALKING_6,
};

impl<'a> FightScene<'a> {
    pub fn init_ryu(&mut self, position: (f32, f32), controller: Controller) {
        let entity = self.entity;
        let texture_index = self
            .drawing
            .texture_store
            .load_texture(RYU_TEXTURE_PATH)
            .expect("Failed to load Ryu texture");

        self.physics.store.insert_component(
            entity,
            PhysicsComponent {
                entity,
                position,
                velocity: (0.0, 0.0),
                acceleration: (0.0, 0.0),
                rigid_body: Some(RigidBody {
                    size: (10.0, 45.0),
                    solid: true,
                }),
                gravity: true,
            },
        );
        self.drawing.store.insert_component(
            entity,
            ShapeComponent {
                entity,
                size: (50, 90),
                flipped: (false, false),
                texture: ShapeTexture {
                    texture_index,
                    sprite: RYU_STAND_1,
                },
            },
        );
        self.input
            .store
            .insert_component(entity, InputComponent { entity, controller });
        self.movement.store.insert_component(
            entity,
            MovementComponent {
                entity,
                incoming: MovementAction::None,
                action: MovementAction::None,
                direction: AimDirection::Right,
                grounded: false,
            },
        );
        self.stand.store.insert_component(
            entity,
            StandComponent {
                entity,
                sprites: [RYU_STAND_1, RYU_STAND_2, RYU_STAND_3, RYU_STAND_4],
                sprite_step: (0, 0),
            },
        );
        self.walking.store.insert_component(
            entity,
            WalkingComponent {
                entity,
                direction: None,
                sprites: [
                    RYU_WALKING_1,
                    RYU_WALKING_2,
                    RYU_WALKING_3,
                    RYU_WALKING_4,
                    RYU_WALKING_5,
                    RYU_WALKING_6,
                ],
                sprite_step: (0, 0),
            },
        );

        self.entity += 1;
    }
}
