mod animations;
mod input;
mod movements;
mod sprites;

use xfight_ecs::components::{
    AnimationComponent, Entity, InputComponent, MovementComponent, PhysicsComponent, ShapeComponent,
};

use self::{
    animations::ryu_animation,
    input::ryu_input,
    movements::{movement_logic, physics_logic, RYU_ACTION_NONE, RYU_STATE_STAND_UP},
    sprites::RYU_STAND_1,
};

pub const RYU_SPRITE_PATH: &'static str = "assets/ryu.png";

pub fn init<'a>(
    entities: &mut Vec<Entity>,
    shapes: &mut Vec<ShapeComponent>,
    physics: &mut Vec<PhysicsComponent>,
    animations: &mut Vec<AnimationComponent>,
    movements: &mut Vec<MovementComponent>,
    inputs: &mut Vec<InputComponent>,
    position: (f32, f32),
    flipped: bool,
) {
    let entity = entities.len();
    // SHAPE
    let shape = Some(shapes.len());
    shapes.push(ShapeComponent {
        entity,
        position,
        size: (50, 90),
        flipped: (flipped, false),
        texture: &RYU_STAND_1,
    });

    // PHYSICS
    let physic = Some(physics.len());
    physics.push(PhysicsComponent {
        entity,
        speed: (0.0, 0.0),
        acceleration: (0.0, 0.0),
        logic: physics_logic,
    });

    // ANIMATION
    let animation = Some(animations.len());
    animations.push(AnimationComponent {
        entity,
        logic: ryu_animation,
    });

    // MOVEMENTS
    let movement = Some(movements.len());
    movements.push(MovementComponent {
        entity,
        action: RYU_ACTION_NONE,
        state: RYU_STATE_STAND_UP,
        logic: movement_logic,
        frame: 0,
    });

    // INPUT
    let input = Some(inputs.len());
    inputs.push(InputComponent {
        entity,
        logic: ryu_input,
    });

    // ENTITY
    entities.push(Entity {
        shape,
        physic,
        animation,
        movement,
        input,
    });
}
