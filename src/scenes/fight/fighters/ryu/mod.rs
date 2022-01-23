mod animations;
mod sprites;

use xfight_ecs::components::{AnimationComponent, Entity, PhysicsComponent, ShapeComponent};

use self::{animations::RYU_STAND, sprites::RYU_STAND_1};

pub const RYU_SPRITE_PATH: &'static str = "assets/ryu.png";

pub fn init<'a>(
    entities: &mut Vec<Entity>,
    shapes: &mut Vec<ShapeComponent>,
    physics: &mut Vec<PhysicsComponent>,
    animations: &mut Vec<AnimationComponent>,
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
        acceleration: (0.01, 0.0),
    });
    // ANIMATION
    let animation = Some(animations.len());
    animations.push(AnimationComponent {
        entity,
        animation: &RYU_STAND,
        actual_sprite: 0,
        actual_frame: RYU_STAND.sprites[0].duration_frames,
        infinite: true,
    });

    entities.push(Entity {
        shape,
        physic,
        animation,
    });
}
