pub mod movements;

use crate::{
    scenes::fight::FightScene,
    systems::{
        aim::{AimComponent, AimDirection},
        collision::CollisionComponent,
        damage::DamageComponent,
        drawing::ShapeComponent,
        health::{HealthAction, HealthComponent, Player},
        input::{Controller, InputComponent},
        movement::MovementComponent,
        position::{PositionAction, PositionComponent},
        tag::{StateTag, TagComponent},
        velocity::VelocityComponent,
    },
};

use self::movements::{RYU_MOVEMENTS, RYU_STAND_INDEX, RYU_TEXTURE_PATH};

impl<'a> FightScene<'a> {
    pub fn init_ryu(&mut self, position: (f32, f32), controller: Controller, player: Player) {
        let entity = self.entity;
        let texture = self
            .drawing
            .texture_store
            .load_texture(RYU_TEXTURE_PATH)
            .expect("Failed to load Ryu texture");

        self.tag.store.insert_component(
            entity,
            TagComponent {
                entity,
                next_state: StateTag(0),
                actual_state: StateTag(0),
            },
        );
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
                sprite: RYU_MOVEMENTS[RYU_STAND_INDEX].sprites[0].sprite,
                flipped: (false, false),
            },
        );
        self.velocity.store.insert_component(
            entity,
            VelocityComponent {
                entity,
                velocity: (0.0, 0.0),
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
        self.movement.store.insert_component(
            entity,
            MovementComponent {
                entity,
                action: None,
                movements: RYU_MOVEMENTS,
                movement: RYU_STAND_INDEX,
                frame: Some((0, 0)),
            },
        );
        self.damage.store.insert_component(
            entity,
            DamageComponent {
                entity,
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
