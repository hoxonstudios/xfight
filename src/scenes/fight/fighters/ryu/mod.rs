pub mod sprites;

use crate::{
    scenes::fight::FightScene,
    systems::{
        basic_attack::BasicAttackComponent,
        damage::{DamageComponent, HealthComponent, Player},
        input::{Controller, InputComponent},
        movement::{AimDirection, MovementComponent},
        physics::{PhysicsComponent, RigidBody, Shape},
        stand::StandComponent,
        stun::StunComponent,
        walking::WalkingComponent,
    },
};

use self::sprites::{
    RYU_LIGHT_KICK, RYU_LIGHT_PUNCH, RYU_STAND, RYU_STRONG_KICK, RYU_STRONG_PUNCH, RYU_STUNT,
    RYU_TEXTURE_PATH, RYU_WALKING,
};

impl<'a> FightScene<'a> {
    pub fn init_ryu(&mut self, position: (f32, f32), controller: Controller, player: Player) {
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
                    padding: (0, 0, 0, 0),
                    solid: true,
                }),
                shape: Shape {
                    flipped: (false, false),
                    texture_index,
                    sprite: RYU_STAND[0],
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
                action: None,
                direction: AimDirection::Right,
                grounded: false,
                attacking: false,
                stunt: false,
            },
        );
        self.stand.store.insert_component(
            entity,
            StandComponent {
                entity,
                sprites: RYU_STAND,
                sprite_step: (0, 0),
            },
        );
        self.walking.store.insert_component(
            entity,
            WalkingComponent {
                entity,
                direction: None,
                sprites: RYU_WALKING,
                sprite_step: (0, 0),
            },
        );
        self.damage.damage_store.insert_component(
            entity,
            DamageComponent {
                entity,
                player,
                consumed: true,
                damage: None,
            },
        );
        self.damage.health_store.insert_component(
            entity,
            HealthComponent {
                entity,
                player,
                health: 100,
            },
        );
        self.stun.store.insert_component(
            entity,
            StunComponent {
                entity,
                health: None,
                sprite: RYU_STUNT,
                stunt_frame: None,
            },
        );
        self.basic_attack.store.insert_component(
            entity,
            BasicAttackComponent {
                entity,
                active: None,
                sprite_step: (0, 0),
                light_punch: RYU_LIGHT_PUNCH,
                strong_punch: RYU_STRONG_PUNCH,
                light_kick: RYU_LIGHT_KICK,
                strong_kick: RYU_STRONG_KICK,
            },
        );

        self.entity += 1;
    }
}
