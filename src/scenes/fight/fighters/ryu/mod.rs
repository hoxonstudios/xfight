mod animations;
mod sprites;

use crate::components::{
    animation::AnimationComponent,
    fighter::{Direction, FighterComponent, FighterState, Player},
    physics::PhysicsComponent,
    shape::ShapeComponent,
    Entity, FighterArchetype,
};

use self::{animations::RYU_TRANSITIONS, sprites::RYU_STAND_1};

pub const RYU_SPRITE_PATH: &'static str = "assets/ryu.png";

pub fn init<'a>(
    entities: &mut Vec<Entity>,
    archetype: &mut FighterArchetype,
    position: (f32, f32),
    player: Player,
    direction: Direction,
) {
    let entity = entities.len();
    // SHAPE
    let shape = Some(archetype.shape.len());
    archetype.shape.push(ShapeComponent {
        entity,
        position,
        size: (50, 90),
        flipped: (matches!(direction, Direction::Left), false),
        texture: RYU_STAND_1,
    });

    // ANIMATION
    let animation = Some(archetype.animation.len());
    archetype.animation.push(AnimationComponent {
        state: FighterState::Standing.key(),
        frame: 0,
        sprite: 0,
        infinite: true,
        transitions: &RYU_TRANSITIONS,
    });

    // FIGHTER
    let fighter = Some(archetype.fighter.len());
    archetype.fighter.push(FighterComponent {
        player,
        direction,
        state: FighterState::Standing,
    });

    // PHYSICS
    let physics = Some(archetype.physics.len());
    archetype.physics.push(PhysicsComponent {});

    // ENTITY
    entities.push(Entity {
        shape,
        animation,
        fighter,
        physics,
    });
}
