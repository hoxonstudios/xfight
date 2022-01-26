pub mod animation;
pub mod shape;

use self::{animation::AnimationComponent, shape::ShapeComponent};

#[derive(Copy, Clone)]
pub struct Entity {
    pub shape: Option<usize>,
    pub animation: Option<usize>,
}

pub struct AnimatedArchetype {
    pub shape: Vec<ShapeComponent>,
    pub animation: Vec<AnimationComponent>,
}
