mod movements;

use crate::{
    scenes::fight::{
        fighters::ryu::movements::RYU_TEXTURE_PATH,
        kinds::{KIND_FIGHTER, KIND_SPELL},
        states::STATE_EXPLODE,
        FightScene,
    },
    systems::{
        aim::AimDirection,
        collision::{CollisionComponent, CollisionKind},
        damage::DamageComponent,
        drawing::ShapeComponent,
        job::FightJobParameters,
        movement::MovementComponent,
        position::{PositionAction, PositionComponent},
        tag::{KindTag, StateTag, TagComponent},
        velocity::VelocityComponent,
    },
};

use self::movements::{RYU_ADOKEN_FLYING_INDEX, RYU_ADOKEN_MOVEMENTS};

pub fn job_spawn_ryu_adoken(scene: &mut FightScene, params: FightJobParameters) {
    if let FightJobParameters::SpawnSpell {
        position,
        direction,
        player,
    } = params
    {
        let entity = scene.entity;
        let (flipped_x, velocity_x) = match direction {
            AimDirection::Right => (false, 5.0),
            AimDirection::Left => (true, -5.0),
        };
        let texture = scene
            .drawing
            .texture_store
            .load_texture(RYU_TEXTURE_PATH)
            .expect("Failed to load Ryu texture");

        scene.tag.store.insert_component(
            entity,
            TagComponent {
                entity,
                kind: KIND_SPELL,
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
                sprite: RYU_ADOKEN_MOVEMENTS[RYU_ADOKEN_FLYING_INDEX].sprites[0].sprite,
                flipped: (flipped_x, false),
            },
        );
        scene.velocity.store.insert_component(
            entity,
            VelocityComponent {
                entity,
                velocity: (velocity_x, 0.0),
                gravity: false,
            },
        );
        scene.collision.store.insert_component(
            entity,
            CollisionComponent {
                entity,
                padding: 0,
                kinds: &[
                    CollisionKind {
                        kind: KIND_FIGHTER,
                        state: STATE_EXPLODE,
                        solid: false,
                    },
                    CollisionKind {
                        kind: KIND_SPELL,
                        state: STATE_EXPLODE,
                        solid: false,
                    },
                ],
            },
        );
        scene.movement.store.insert_component(
            entity,
            MovementComponent {
                entity,
                action: None,
                movements: RYU_ADOKEN_MOVEMENTS,
                movement: RYU_ADOKEN_FLYING_INDEX,
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

        scene.entity += 1;
    }
}

pub fn job_destroy_ryu_adoken(scene: &mut FightScene, params: FightJobParameters) {
    if let FightJobParameters::DestroyEntity { entity } = params {
        scene.tag.store.delete_component(entity, |e| e.entity);
        scene.position.store.delete_component(entity, |e| e.entity);
        scene.drawing.store.delete_component(entity, |e| e.entity);
        scene.velocity.store.delete_component(entity, |e| e.entity);
        scene.collision.store.delete_component(entity, |e| e.entity);
        scene.movement.store.delete_component(entity, |e| e.entity);
        scene.damage.store.delete_component(entity, |e| e.entity);
    }
}
