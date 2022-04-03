pub mod sprites;

use crate::{
    scenes::fight::FightScene,
    systems::{
        basic_attack::BasicAttackComponent,
        collision::CollisionComponent,
        damage::{DamageAction, DamageComponent},
        health::{HealthAction, HealthComponent, Player},
        input::{Controller, InputComponent},
        jump::JumpComponent,
        movement::{AimDirection, MovementComponent},
        position::{PositionAction, PositionComponent},
        shape::{ShapeAction, ShapeComponent},
        stand::StandComponent,
        stun::StunComponent,
        velocity::{VelocityAction, VelocityComponent},
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
        let texture = self
            .drawing
            .texture_store
            .load_texture(RYU_TEXTURE_PATH)
            .expect("Failed to load Ryu texture");

        self.position.store.insert_component(
            entity,
            PositionComponent {
                entity,
                action: PositionAction::None,
                x: position.0,
                y: position.1,
            },
        );
        self.shape.store.insert_component(
            entity,
            ShapeComponent {
                entity,
                action: ShapeAction::None,
                texture,
                sprite: RYU_STAND[0],
                flipped: (false, false),
            },
        );
        self.velocity.store.insert_component(
            entity,
            VelocityComponent {
                entity,
                action: VelocityAction::None,
                velocity: (0.0, 0.0),
                acceleration: (0.0, 0.0),
                gravity: true,
            },
        );
        self.collision.store.insert_component(
            entity,
            CollisionComponent {
                entity,
                padding: 0,
                solid: true,
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
        self.jump.store.insert_component(
            entity,
            JumpComponent {
                entity,
                active: false,
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
        self.damage.store.insert_component(
            entity,
            DamageComponent {
                entity,
                action: DamageAction::None,
                player,
                damage: None,
            },
        );
        self.health.store.insert_component(
            entity,
            HealthComponent {
                entity,
                action: HealthAction::None,
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
