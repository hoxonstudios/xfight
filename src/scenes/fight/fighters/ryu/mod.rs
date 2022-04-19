pub mod sprites;

use crate::{
    scenes::fight::FightScene,
    systems::{
        aim::{AimComponent, AimDirection},
        collision::CollisionComponent,
        damage::{DamageAction, DamageComponent},
        drawing::ShapeComponent,
        ground::GroundComponent,
        health::{HealthAction, HealthComponent, Player},
        input::{Controller, InputComponent},
        movement::{MovementComponent, MovementSprites, MovementState},
        position::{PositionAction, PositionComponent},
        velocity::VelocityComponent,
    },
};

use self::sprites::{
    RYU_BLOCK, RYU_CROUCH, RYU_CROUCH_BLOCK, RYU_CROUCH_LIGHT_KICK, RYU_CROUCH_LIGHT_PUNCH,
    RYU_CROUCH_STRONG_KICK, RYU_CROUCH_STRONG_PUNCH, RYU_LIGHT_KICK, RYU_LIGHT_PUNCH, RYU_STAND,
    RYU_STRONG_KICK, RYU_STRONG_PUNCH, RYU_STUNT, RYU_TEXTURE_PATH, RYU_WALKING,
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
        self.drawing.store.insert_component(
            entity,
            ShapeComponent {
                entity,
                texture,
                sprite: RYU_STAND[0],
                flipped: (false, false),
            },
        );
        self.velocity.store.insert_component(
            entity,
            VelocityComponent {
                entity,
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
        self.aim.store.insert_component(
            entity,
            AimComponent {
                entity,
                direction: AimDirection::Right,
            },
        );
        self.ground
            .store
            .insert_component(entity, GroundComponent { entity });
        self.movement.store.insert_component(
            entity,
            MovementComponent {
                entity,
                action: None,
                state: MovementState::Standing { frame: (0, 0) },
                sprites: MovementSprites {
                    standing: RYU_STAND,
                    walking: RYU_WALKING,
                    stunt: RYU_STUNT,

                    block: RYU_BLOCK,
                    crouch_block: RYU_CROUCH_BLOCK,

                    light_punch: RYU_LIGHT_PUNCH,
                    strong_punch: RYU_STRONG_PUNCH,
                    light_kick: RYU_LIGHT_KICK,
                    strong_kick: RYU_STRONG_KICK,

                    crouching: RYU_CROUCH,
                    crouch_light_punch: RYU_CROUCH_LIGHT_PUNCH,
                    crouch_strong_punch: RYU_CROUCH_STRONG_PUNCH,
                    crouch_light_kick: RYU_CROUCH_LIGHT_KICK,
                    crouch_strong_kick: RYU_CROUCH_STRONG_KICK,
                },
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
                shield: None,
            },
        );

        self.entity += 1;
    }
}
