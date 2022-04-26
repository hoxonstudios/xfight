pub mod movements;
pub mod spells;

use crate::{
    scenes::fight::{
        kinds::{KIND_FIGHTER, KIND_FLOOR},
        states::{STATE_DEAD, STATE_GROUNDED},
        FightScene,
    },
    systems::{
        aim::{AimComponent, AimDirection},
        collision::{CollisionComponent, CollisionKind},
        damage::DamageComponent,
        drawing::ShapeComponent,
        health::{HealthAction, HealthComponent},
        input::InputComponent,
        job::FightJobParameters,
        movement::MovementComponent,
        position::{PositionAction, PositionComponent},
        tag::{StateTag, TagComponent},
        velocity::VelocityComponent,
    },
};

use self::movements::{RYU_MOVEMENTS, RYU_STAND_INDEX, RYU_TEXTURE_PATH};

pub fn job_spawn_ryu(scene: &mut FightScene, params: FightJobParameters) {
    if let FightJobParameters::SpawnFighter {
        position,
        controller,
        player,
    } = params
    {
        let entity = scene.entity;
        let texture = scene
            .drawing
            .texture_store
            .load_texture(RYU_TEXTURE_PATH)
            .expect("Failed to load Ryu texture");

        scene.tag.store.insert_component(
            entity,
            TagComponent {
                entity,
                kind: KIND_FIGHTER,
                next_state: StateTag(0),
                actual_state: StateTag(0),
            },
        );
        scene.position.store.insert_component(
            entity,
            PositionComponent {
                entity,
                action: PositionAction::None,
                x: position.0,
                y: position.1,
            },
        );
        scene.drawing.store.insert_component(
            entity,
            ShapeComponent {
                entity,
                texture,
                sprite: RYU_MOVEMENTS[RYU_STAND_INDEX].sprites[0].sprite,
                flipped: (false, false),
            },
        );
        scene.velocity.store.insert_component(
            entity,
            VelocityComponent {
                entity,
                velocity: (0.0, 0.0),
                gravity: true,
            },
        );
        scene.collision.store.insert_component(
            entity,
            CollisionComponent {
                entity,
                padding: 0,
                kinds: &[CollisionKind {
                    kind: KIND_FLOOR,
                    state: STATE_GROUNDED,
                    solid: true,
                }],
            },
        );
        scene
            .input
            .store
            .insert_component(entity, InputComponent { entity, controller });
        scene.aim.store.insert_component(
            entity,
            AimComponent {
                entity,
                direction: AimDirection::Right,
            },
        );
        scene.movement.store.insert_component(
            entity,
            MovementComponent {
                entity,
                action: None,
                movements: RYU_MOVEMENTS,
                movement: RYU_STAND_INDEX,
                frame: Some((0, 0)),
            },
        );
        scene.damage.store.insert_component(
            entity,
            DamageComponent {
                entity,
                player,
                damage: None,
            },
        );
        scene.health.store.insert_component(
            entity,
            HealthComponent {
                entity,
                action: HealthAction::None,
                player,
                health: 100,
                shield: None,
                dead_tag: STATE_DEAD,
            },
        );

        scene.entity += 1;
    }
}
