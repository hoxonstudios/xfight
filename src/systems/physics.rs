use crate::components::{physics::PhysicsComponent, shape::ShapeComponent};

pub fn run<'a>(entities: impl Iterator<Item = (&'a mut PhysicsComponent, &'a mut ShapeComponent)>) {
    for (physics, shape) in entities {
        shape.position.0 += physics.velocity.0;
        shape.position.1 += physics.velocity.1;

        physics.velocity.0 += physics.acceleration.0;
        physics.velocity.1 += physics.acceleration.1 + if physics.gravity { 10.0 } else { 0.0 };
    }
}
