mod animations;
mod sprites;

use crate::components::{
    animation::AnimationComponent, shape::ShapeComponent, AnimatedArchetype, Entity,
};

use self::{
    animations::{RYU_TRANSITIONS, RYU_TRANSITION_STAND_INDEX},
    sprites::RYU_STAND_1,
};

pub const RYU_SPRITE_PATH: &'static str = "assets/ryu.png";

pub fn init<'a>(
    entities: &mut Vec<Entity>,
    archetype: &mut AnimatedArchetype,
    position: (f32, f32),
    flipped: bool,
) {
    let entity = entities.len();
    // SHAPE
    let shape = Some(archetype.shape.len());
    archetype.shape.push(ShapeComponent {
        entity,
        position,
        size: (50, 90),
        flipped: (flipped, false),
        texture: RYU_STAND_1,
    });

    // ANIMATION
    let animation = Some(archetype.animation.len());
    archetype.animation.push(AnimationComponent {
        state: RYU_TRANSITION_STAND_INDEX,
        frame: 0,
        sprite: 0,
        infinite: true,
        transitions: &RYU_TRANSITIONS,
    });

    // ENTITY
    entities.push(Entity { shape, animation });
}
