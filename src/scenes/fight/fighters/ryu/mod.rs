pub mod sprites;

use crate::components::{physics::PhysicsComponent, shape::ShapeComponent, FighterArchetype};

use self::sprites::RYU_STAND_1;

impl FighterArchetype {
    pub fn init_ryu<'a>(&mut self, position: (f32, f32), flipped: bool) {
        self.shape.push(ShapeComponent {
            position,
            size: (50, 90),
            flipped: (flipped, false),
            texture: RYU_STAND_1,
        });
        self.physics.push(PhysicsComponent {
            velocity: (0.0, 0.0),
            acceleration: (0.0, 0.0),
            gravity: true,
        });
    }
}
