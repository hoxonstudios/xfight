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
        let compare_vec = physics_vec.clone();
        let length = physics_vec.len();

        for i in 0..length {
            let physics = &mut physics_vec[i];
            physics.position.0 += physics.velocity.0;
            physics.position.1 += physics.velocity.1;

            physics.velocity.0 += physics.acceleration.0;
            physics.velocity.1 += physics.acceleration.1 + if physics.gravity { 0.1 } else { 0.0 };

            for j in 0..i {
                let compare = &compare_vec[j];
                if let Some(rb1) = physics.rigid_body {
                    if let Some(rb2) = compare.rigid_body {
                        let r1_x1 = physics.position.0 - rb1.size.0;
                        let r1_x2 = physics.position.0 + rb1.size.0;
                        let r1_y1 = physics.position.1 - rb1.size.1;
                        let r1_y2 = physics.position.1 + rb1.size.1;

                        let r2_x1 = compare.position.0 - rb2.size.0;
                        let r2_x2 = compare.position.0 + rb2.size.0;
                        let r2_y1 = compare.position.1 - rb2.size.1;
                        let r2_y2 = compare.position.1 + rb2.size.1;

                        // Check collision
                        let collide_in_x = r1_x1 <= r2_x2 && r1_x2 >= r2_x1;
                        let collide_in_y = r1_y1 <= r2_y2 && r1_y2 >= r2_y1;
                        let collide = collide_in_x && collide_in_y;

                        if collide {
                            self.collisions.push(Collision {
                                entities: (physics.entity, compare.entity),
                            });
                            if rb1.solid && rb2.solid {
                                // Avoid overlapping
                                if physics.velocity.0 > 0.0 && r1_x1 < r2_x1 {
                                    physics.position.0 -= r1_x2 - r2_x1;
                                    physics.velocity.0 = 0.0;
                                } else if physics.velocity.0 < 0.0 && r1_x2 > r2_x2 {
                                    physics.position.0 += r2_x2 - r1_x1;
                                    physics.velocity.0 = 0.0;
                                }
                                if physics.velocity.1 > 0.0 && r1_y1 < r2_y1 {
                                    physics.position.1 -= r1_y2 - r2_y1;
                                    physics.velocity.1 = 0.0;
                                } else if physics.velocity.1 < 0.0 && r1_y2 > r2_y2 {
                                    physics.position.0 += r2_y2 - r1_y1;
                                    physics.velocity.1 = 0.0;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
