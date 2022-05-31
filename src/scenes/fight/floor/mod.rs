pub mod sprites;

use crate::systems::{
    collision::CollisionComponent,
    drawing::ShapeComponent,
    job::FightJobParameters,
    position::{PositionAction, PositionComponent},
    tag::{StateTag, TagComponent},
};

use self::sprites::{FLOOR_SPRITE, FLOOR_TEXTURE_PATH};

use super::{kinds::KIND_FLOOR, FightScene};

pub fn job_spawn_floor(scene: &mut FightScene, params: FightJobParameters) {
    if let FightJobParameters::None = params {
        let entity = scene.entity;

        let texture = scene
            .drawing
            .texture_store
            .load_texture(FLOOR_TEXTURE_PATH)
            .expect("Failed to load floor texture");

        scene.tag.store.insert_component(
            entity,
            TagComponent {
                entity,
                kind: KIND_FLOOR,
                next_state: StateTag(0),
                actual_state: StateTag(0),
            },
        );
        scene.position.store.insert_component(
            entity,
            PositionComponent {
                entity,
                action: PositionAction::None,
                x: 400.0,
                y: 500.0,
            },
        );
        scene.drawing.store.insert_component(
            entity,
            ShapeComponent {
                entity,
                texture,
                sprite: FLOOR_SPRITE,
                flipped: (false, false),
            },
        );
        scene.collision.store.insert_component(
            entity,
            CollisionComponent {
                entity,
                padding: (0, 500, 0, 0),
                kinds: &[],
            },
        );

        scene.entity += 1;
    }
}
