use super::{
    helpers::ComponentStore,
    position::{PositionAction, PositionSystem},
    shape::ShapeSystem,
};

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
    pub delta_time: (f32, f32),
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
    pub fn update(&mut self, shape_system: &ShapeSystem, position_system: &mut PositionSystem) {
        self.collisions.clear();
        let mut collisions: Vec<CollisionRect> = vec![];

        for collision in self.store.data_mut() {
            let entity = collision.entity;
            if let Some(shape) = shape_system.store.get_component(entity) {
                if let Some(position) = position_system.store.get_component(entity) {
                    let velocity = match position.action {
                        PositionAction::None => (0.0, 0.0),
                        PositionAction::Move { x, y } => (x - position.x, y - position.y),
                    };
                    let position = match position.action {
                        PositionAction::None => (position.x, position.y),
                        PositionAction::Move { x, y } => (x, y),
                    };

                    let rect = shape.sprite.rect(shape.flipped);
                    let x1 = position.0 + (rect.0 + collision.padding) as f32;
                    let y1 = position.1 + (rect.1 + collision.padding) as f32;
                    let x2 = position.0 + (rect.2 - collision.padding) as f32;
                    let y2 = position.1 + (rect.3 - collision.padding) as f32;

                    collisions.push(CollisionRect {
                        entity,
                        rect: (x1, y1, x2, y2),
                        solid: collision.solid,
                        velocity,
                    });
                }
            }
        }

        for i in 0..collisions.len() {
            for j in 0..i {
                let compare1 = &collisions[i];
                let compare2 = &collisions[j];
                let collide = CollisionSystem::check_collision(compare1.rect, compare2.rect);
                if collide {
                    let (delta_time, (left, up, right, down)) =
                        CollisionSystem::get_collision_sides(
                            compare1.rect,
                            compare1.velocity,
                            compare2.rect,
                            compare2.velocity,
                        );

                    self.collisions.push((
                        Collision {
                            entity: compare1.entity,
                            sides: [left, up, right, down],
                            solid: compare1.solid,
                            delta_time,
                        },
                        Collision {
                            entity: compare2.entity,
                            sides: [right, down, left, up],
                            solid: compare2.solid,
                            delta_time,
                        },
                    ));
                }
            }
        }

        for collision in self.store.data() {
            let entity = collision.entity;
            let delta_time: (f32, f32) =
                self.collisions.iter().fold((0.0, 0.0), |acc, (c1, c2)| {
                    if c1.solid && c2.solid {
                        if c1.entity == entity {
                            (acc.0.min(c1.delta_time.0), acc.1.min(c1.delta_time.1))
                        } else if c2.entity == entity {
                            (acc.0.min(c2.delta_time.0), acc.1.min(c2.delta_time.1))
                        } else {
                            acc
                        }
                    } else {
                        acc
                    }
                });

            if delta_time.0 < 0.0 || delta_time.1 < 0.0 {
                if let Some(position) = position_system.store.get_mut_component(entity) {
                    match position.action {
                        PositionAction::None => {}
                        PositionAction::Move { x, y } => {
                            let corrected_x = if delta_time.0 < -1.0 {
                                position.x
                            } else if delta_time.0 < 0.0 {
                                x + (x - position.x) * delta_time.0
                            } else {
                                x
                            };
                            let corrected_y = if delta_time.1 < -1.0 {
                                position.y
                            } else if delta_time.1 < 0.0 {
                                y + (y - position.y) * delta_time.1
                            } else {
                                y
                            };
                            position.action = PositionAction::Move {
                                x: corrected_x,
                                y: corrected_y,
                            };
                        }
                    }
                }
            }
        }
    }

    fn check_collision(rect1: (f32, f32, f32, f32), rect2: (f32, f32, f32, f32)) -> bool {
        let (r1_x1, r1_y1, r1_x2, r1_y2) = rect1;
        let (r2_x1, r2_y1, r2_x2, r2_y2) = rect2;

        let collide_in_x = r1_x1 <= r2_x2 && r1_x2 >= r2_x1;
        let collide_in_y = r1_y1 <= r2_y2 && r1_y2 >= r2_y1;
        let collide = collide_in_x && collide_in_y;

        return collide;
    }
    fn get_collision_sides(
        rect1: (f32, f32, f32, f32),
        velocity1: (f32, f32),
        rect2: (f32, f32, f32, f32),
        velocity2: (f32, f32),
    ) -> ((f32, f32), (bool, bool, bool, bool)) {
        let velocity_delta_x = velocity2.0 - velocity1.0;
        let velocity_delta_y = velocity2.1 - velocity1.1;
        let left = if velocity_delta_x != 0.0 {
            (rect1.0 - rect2.2) / velocity_delta_x
        } else {
            0.0
        };
        let top = if velocity_delta_y != 0.0 {
            (rect1.1 - rect2.3) / velocity_delta_y
        } else {
            0.0
        };
        let right = if velocity_delta_x != 0.0 {
            (rect1.2 - rect2.0) / velocity_delta_x
        } else {
            0.0
        };
        let bottom = if velocity_delta_y != 0.0 {
            (rect1.3 - rect2.1) / velocity_delta_y
        } else {
            0.0
        };
        let correction_x = if left < 0.0 && left > right {
            left
        } else if right < 0.0 {
            right
        } else {
            0.0
        };
        let correction_y = if top < 0.0 && top > bottom {
            top
        } else if bottom < 0.0 {
            bottom
        } else {
            0.0
        };

        let collided = (
            left < 0.0 && (left > correction_y || correction_y >= 0.0),
            top < 0.0 && (top > correction_x || correction_x >= 0.0),
            right < 0.0 && (right > correction_y || correction_y >= 0.0),
            bottom < 0.0 && (bottom > correction_x || correction_x >= 0.0),
        );
        let time_delta = (
            if collided.0 {
                left
            } else if collided.2 {
                right
            } else {
                0.0
            },
            if collided.1 {
                top
            } else if collided.3 {
                bottom
            } else {
                0.0
            },
        );

        return (time_delta, collided);
    }
}

#[derive(Copy, Clone)]
struct CollisionRect {
    entity: usize,
    rect: (f32, f32, f32, f32),
    solid: bool,
    velocity: (f32, f32),
}
