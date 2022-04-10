use self::calculations::get_collision_time;

use super::{
    helpers::ComponentStore,
    position::{PositionAction, PositionComponent, PositionSystem},
    shape::{ShapeComponent, ShapeSystem},
    velocity::VelocitySystem,
};

mod calculations;

#[derive(Copy, Clone)]
pub struct CollisionComponent {
    pub entity: usize,
    pub padding: i32,
    pub solid: bool,
}

#[derive(Copy, Clone)]
pub struct Collision {
    pub entity: usize,
    pub sides: [bool; 4],
    pub solid: bool,
}

pub struct CollisionSystem {
    pub collisions: Vec<(Collision, Collision)>,
    pub store: ComponentStore<CollisionComponent>,
}
impl CollisionSystem {
    pub fn init() -> CollisionSystem {
        CollisionSystem {
            collisions: vec![],
            store: ComponentStore::<CollisionComponent>::init(),
        }
    }
    pub fn update(
        &mut self,
        shape_system: &ShapeSystem,
        position_system: &mut PositionSystem,
        velocity_system: &mut VelocitySystem,
    ) {
        self.collisions.clear();
        let mut rigid_bodies: Vec<Option<(usize, Rectangle, Point)>> = vec![];
        let collisions = self.store.data();
        let length = collisions.len();

        for collision in collisions {
            let entity = collision.entity;
            rigid_bodies.push(get_rigid_body(
                entity,
                collision,
                shape_system.store.get_component(entity),
                position_system.store.get_component(entity),
            ));
        }

        let mut updates: Vec<Option<CollisionUpdate>> = vec![None; collisions.len()];

        for i in 0..length {
            if let Some((_, target, _)) = rigid_bodies[i] {
                let actual = &collisions[i];

                for j in 0..length {
                    if i != j {
                        if let Some((_, source, velocity)) = &mut rigid_bodies[j] {
                            let compare = &collisions[j];
                            let collision = get_collision_time(source, velocity, &target);
                            if let Some(collision) = collision {
                                let (left, top, right, bottom) = collision;
                                if actual.solid && compare.solid {
                                    let actual_update = &mut updates[j];

                                    let entity = compare.entity;
                                    let stop_velocity_x = (velocity.x > 0.0 && right.is_some())
                                        || (velocity.x < 0.0 && left.is_some());
                                    let stop_velocity_y = (velocity.y > 0.0 && bottom.is_some())
                                        || (velocity.y < 0.0 && top.is_some());
                                    let delta_time =
                                        left.or(top.or(right.or(bottom))).or(Some(0.0)).unwrap();

                                    if let Some(actual_update) = actual_update {
                                        actual_update.stop_velocity.0 |= stop_velocity_x;
                                        actual_update.stop_velocity.1 |= stop_velocity_y;
                                        actual_update.delta_time =
                                            actual_update.delta_time.min(delta_time);
                                    } else {
                                        *actual_update = Some(CollisionUpdate {
                                            entity,
                                            stop_velocity: (stop_velocity_x, stop_velocity_y),
                                            delta_time,
                                        });
                                    }
                                }

                                self.collisions.push((
                                    Collision {
                                        entity: compare.entity,
                                        solid: compare.solid,
                                        sides: [
                                            left.is_some(),
                                            top.is_some(),
                                            right.is_some(),
                                            bottom.is_some(),
                                        ],
                                    },
                                    Collision {
                                        entity: actual.entity,
                                        solid: actual.solid,
                                        sides: [
                                            right.is_some(),
                                            bottom.is_some(),
                                            left.is_some(),
                                            top.is_some(),
                                        ],
                                    },
                                ))
                            }
                        }
                    }
                }
            }
        }

        for update in updates {
            if let Some(update) = update {
                let entity = update.entity;
                if let Some(position) = position_system.store.get_mut_component(entity) {
                    position.action = match position.action {
                        PositionAction::Move { x, y } => {
                            let x = if update.stop_velocity.0 {
                                position.x + (x - position.x) * update.delta_time
                            } else {
                                x
                            };
                            let y = if update.stop_velocity.1 {
                                position.y + (y - position.y) * update.delta_time
                            } else {
                                y
                            };

                            PositionAction::Move { x, y }
                        }
                        _ => position.action,
                    }
                }
                if let Some(velocity) = velocity_system.store.get_mut_component(entity) {
                    if update.stop_velocity.0 {
                        velocity.velocity.0 = 0.0;
                    }
                    if update.stop_velocity.1 {
                        velocity.velocity.1 = 0.0;
                    }
                }
            }
        }
    }
}

fn get_rigid_body(
    entity: usize,
    collision: &CollisionComponent,
    shape: Option<&ShapeComponent>,
    position: Option<&PositionComponent>,
) -> Option<(usize, Rectangle, Point)> {
    if let Some(shape) = shape {
        if let Some(position) = position {
            let rect = shape.sprite.rect(shape.flipped);
            let velocity = match position.action {
                PositionAction::Move { x, y } => Point {
                    x: x - position.x,
                    y: y - position.y,
                },
                PositionAction::None => Point { x: 0.0, y: 0.0 },
            };
            let rectangle = Rectangle {
                x0: position.x + (rect.0 + collision.padding) as f32,
                y0: position.y + (rect.1 + collision.padding) as f32,
                x1: position.x + (rect.2 - collision.padding) as f32,
                y1: position.y + (rect.3 - collision.padding) as f32,
            };

            Some((entity, rectangle, velocity))
        } else {
            None
        }
    } else {
        None
    }
}

#[derive(Copy, Clone)]
struct CollisionUpdate {
    entity: usize,
    delta_time: f32,
    stop_velocity: (bool, bool),
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct Rectangle {
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
}
#[derive(Copy, Clone, PartialEq, Debug)]
struct Point {
    x: f32,
    y: f32,
}
