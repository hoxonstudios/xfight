use super::helpers::ComponentStore;

#[derive(Copy, Clone)]
pub struct PhysicsComponent {
    pub entity: usize,
    pub position: (f32, f32),
    pub velocity: (f32, f32),
    pub acceleration: (f32, f32),
    pub rigid_body: Option<RigidBody>,
    pub gravity: bool,
}

#[derive(Copy, Clone)]
pub struct RigidBody {
    pub size: (f32, f32),
    pub solid: bool,
}

pub struct Collision {
    pub entities: (usize, usize),
}

pub struct PhysicsSystem {
    pub collisions: Vec<Collision>,
    pub store: ComponentStore<PhysicsComponent>,
}
impl PhysicsSystem {
    pub fn init() -> PhysicsSystem {
        PhysicsSystem {
            collisions: vec![],
            store: ComponentStore::<PhysicsComponent>::init(),
        }
    }
    pub fn update(&mut self) {
        self.collisions.clear();
        let physics_vec = self.store.data_mut();
        let mut destination_vec = physics_vec.clone();
        let mut collisions_rollback = vec![(0.0, 0.0); destination_vec.len()];
        let length = physics_vec.len();

        for destination in &mut destination_vec {
            PhysicsSystem::move_object(destination);
        }

        for i in 0..length {
            for j in 0..i {
                let compare1 = &destination_vec[i];
                let compare2 = &destination_vec[j];
                if let Some(rb1) = compare1.rigid_body {
                    if let Some(rb2) = compare2.rigid_body {
                        let r1_x1 = compare1.position.0 - rb1.size.0;
                        let r1_x2 = compare1.position.0 + rb1.size.0;
                        let r1_y1 = compare1.position.1 - rb1.size.1;
                        let r1_y2 = compare1.position.1 + rb1.size.1;

                        let r2_x1 = compare2.position.0 - rb2.size.0;
                        let r2_x2 = compare2.position.0 + rb2.size.0;
                        let r2_y1 = compare2.position.1 - rb2.size.1;
                        let r2_y2 = compare2.position.1 + rb2.size.1;

                        let collide = PhysicsSystem::check_collision(
                            (r1_x1, r1_y1, r1_x2, r1_y2),
                            (r2_x1, r2_y1, r2_x2, r2_y2),
                        );

                        if collide {
                            let x_vel_difference = compare1.velocity.0 - compare2.velocity.0;
                            let y_vel_difference = compare1.velocity.1 - compare2.velocity.1;
                            let x_collide_side = if x_vel_difference < 0.0 {
                                XCollisionSide::Left
                            } else if x_vel_difference > 0.0 {
                                XCollisionSide::Right
                            } else {
                                XCollisionSide::None
                            };
                            let y_collide_side = if y_vel_difference < 0.0 {
                                YCollisionSide::Up
                            } else if y_vel_difference > 0.0 {
                                YCollisionSide::Down
                            } else {
                                YCollisionSide::None
                            };
                            let rollback_time_y = match y_collide_side {
                                YCollisionSide::None => 0.0,
                                YCollisionSide::Up => (r2_y2 - r1_y1) / -y_vel_difference,
                                YCollisionSide::Down => (r1_y2 - r2_y1) / y_vel_difference,
                            };
                            let rollback_time_x = match x_collide_side {
                                XCollisionSide::None => 0.0,
                                XCollisionSide::Left => (r2_x2 - r1_x1) / -x_vel_difference,
                                XCollisionSide::Right => (r1_x2 - r2_x1) / x_vel_difference,
                            };

                            collisions_rollback[i] = (
                                if rollback_time_x <= 1.0
                                    && rollback_time_x > collisions_rollback[i].0
                                {
                                    rollback_time_x
                                } else {
                                    collisions_rollback[i].0
                                },
                                if rollback_time_y <= 1.0
                                    && rollback_time_y > collisions_rollback[i].1
                                {
                                    rollback_time_y
                                } else {
                                    collisions_rollback[i].1
                                },
                            );
                            collisions_rollback[j] = (
                                if rollback_time_x <= 1.0
                                    && rollback_time_x > collisions_rollback[j].0
                                {
                                    rollback_time_x
                                } else {
                                    collisions_rollback[j].0
                                },
                                if rollback_time_y <= 1.0
                                    && rollback_time_y > collisions_rollback[j].1
                                {
                                    rollback_time_y
                                } else {
                                    collisions_rollback[j].1
                                },
                            );

                            self.collisions.push(Collision {
                                entities: (compare1.entity, compare2.entity),
                            });
                        }
                    }
                }
            }
        }

        for i in 0..length {
            let physics = &mut physics_vec[i];
            let (rollback_x, rollback_y) = collisions_rollback[i];

            if rollback_x <= 1.0 {
                physics.velocity.0 = physics.velocity.0 * (1.0 - rollback_x);
                physics.acceleration.0 = 0.0;
            }
            if rollback_y <= 1.0 {
                physics.velocity.1 = physics.velocity.1 * (1.0 - rollback_y);
                physics.acceleration.1 = 0.0;
            }

            PhysicsSystem::move_object(physics);
        }
    }
    fn move_object(physics: &mut PhysicsComponent) {
        physics.position.0 += physics.velocity.0;
        physics.position.1 += physics.velocity.1;

        physics.velocity.0 += physics.acceleration.0;
        physics.velocity.1 += physics.acceleration.1 + if physics.gravity { 0.2 } else { 0.0 };
    }
    fn check_collision(rect1: (f32, f32, f32, f32), rect2: (f32, f32, f32, f32)) -> bool {
        let (r1_x1, r1_y1, r1_x2, r1_y2) = rect1;
        let (r2_x1, r2_y1, r2_x2, r2_y2) = rect2;

        let collide_in_x = r1_x1 <= r2_x2 && r1_x2 >= r2_x1;
        let collide_in_y = r1_y1 <= r2_y2 && r1_y2 >= r2_y1;
        let collide = collide_in_x && collide_in_y;

        return collide;
    }
}

enum XCollisionSide {
    Left,
    Right,
    None,
}
enum YCollisionSide {
    Up,
    Down,
    None,
}
