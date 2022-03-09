pub mod physics;
pub mod shape;

use self::{physics::PhysicsComponent, shape::ShapeComponent};

pub struct ShapeArchetype {
    pub shape: Vec<ShapeComponent>,
}
pub struct FighterArchetype {
    pub shape: Vec<ShapeComponent>,
    pub physics: Vec<PhysicsComponent>,
}
