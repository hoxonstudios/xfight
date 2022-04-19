pub mod sprites;

use crate::systems::{
    collision::CollisionComponent,
    drawing::ShapeComponent,
    position::{PositionAction, PositionComponent},
};

use self::sprites::{FLOOR_SPRITE, FLOOR_TEXTURE_PATH};

use super::FightScene;

impl<'a> FightScene<'a> {
    pub fn init_floor(&mut self) {
        let entity = self.entity;

        let texture = self
            .drawing
            .texture_store
            .load_texture(FLOOR_TEXTURE_PATH)
            .expect("Failed to load floor texture");

        self.position.store.insert_component(
            entity,
            PositionComponent {
                entity,
                action: PositionAction::None,
                x: 400.0,
                y: 550.0,
            },
        );
        self.drawing.store.insert_component(
            entity,
            ShapeComponent {
                entity,
                texture,
                sprite: FLOOR_SPRITE,
                flipped: (false, false),
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

        self.entity += 1;
    }
}
