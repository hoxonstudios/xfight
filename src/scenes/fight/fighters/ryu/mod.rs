pub mod sprites;

use crate::{
    scenes::fight::FightScene,
    systems::{
        input::{Controller, InputComponent},
        movement::{AimDirection, MovementAction, MovementComponent},
        physics::{PhysicsComponent, RigidBody, Shape},
        punch::PunchComponent,
        stand::StandComponent,
        walking::WalkingComponent,
    },
};

use self::sprites::{
    RYU_LIGHT_PUNCH_1, RYU_LIGHT_PUNCH_2, RYU_LIGHT_PUNCH_3, RYU_STAND_1, RYU_STAND_2, RYU_STAND_3,
    RYU_STAND_4, RYU_TEXTURE_PATH, RYU_WALKING_1, RYU_WALKING_2, RYU_WALKING_3, RYU_WALKING_4,
    RYU_WALKING_5, RYU_WALKING_6,
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
                    padding: (0.0, 0.0, 0.0, 0.0),
                    solid: true,
                }),
                shape: Shape {
                    flipped: (false, false),
                    texture_index,
                    sprite: RYU_STAND_1,
                },
                gravity: true,
            },
        );
        self.input
            .store
            .insert_component(entity, InputComponent { entity, controller });
        self.movement.store.insert_component(
            entity,
            MovementComponent {
                entity,
                action: MovementAction::None,
                direction: AimDirection::Right,
                grounded: false,
                attacking: false,
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
        self.punch.store.insert_component(
            entity,
            PunchComponent {
                entity,
                active: false,
                sprite_step: (0, 0),
                sprites: [RYU_LIGHT_PUNCH_1, RYU_LIGHT_PUNCH_2, RYU_LIGHT_PUNCH_3],
            },
        );

        self.entity += 1;
    }
}
