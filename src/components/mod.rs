pub mod animation;
pub mod fighter;
pub mod physics;
pub mod shape;

use self::{
    animation::AnimationComponent, fighter::FighterComponent, physics::PhysicsComponent,
    shape::ShapeComponent,
};

#[derive(Copy, Clone)]
pub struct Entity {
    pub shape: Option<usize>,
    pub animation: Option<usize>,
    pub fighter: Option<usize>,
    pub physics: Option<usize>,
}

pub struct FighterArchetype {
    pub shape: Vec<ShapeComponent>,
    pub animation: Vec<AnimationComponent>,
    pub fighter: Vec<FighterComponent>,
    pub physics: Vec<PhysicsComponent>,
}
