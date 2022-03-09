pub mod sprites;

use crate::components::{shape::ShapeComponent, ShapeArchetype};

use self::sprites::FLOOR_SPRITE;

impl ShapeArchetype {
    pub fn init_floor(&mut self) {
        self.shape.push(ShapeComponent {
            position: (400.0, 550.0),
            size: (800, 100),
            flipped: (false, false),
            texture: FLOOR_SPRITE,
        })
    }
}
