use super::helpers::ComponentStore;

#[derive(Copy, Clone)]
pub struct PhysicsComponent {
    pub entity: usize,
    pub position: (f32, f32),
    pub velocity: (f32, f32),
    pub acceleration: (f32, f32),
    pub rigid_body: Option<RigidBody>,
    pub shape: Shape,
    pub gravity: bool,
}

#[derive(Copy, Clone)]
pub struct RigidBody {
    pub padding: (f32, f32, f32, f32),
    pub solid: bool,
}
#[derive(Copy, Clone)]
pub struct Shape {
    pub texture_index: usize,
    pub flipped: (bool, bool),
    pub sprite: TextureSprite,
}
#[derive(Copy, Clone)]
pub struct TextureSprite {
    pub center: (u32, u32),
    pub area: (u32, u32, u32, u32),
}

#[derive(Copy, Clone)]
pub struct Collision {
    pub entities: (usize, usize),
    pub sides: ([bool; 4], [bool; 4]),
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
        let mut collisions = vec![[false; 4]; physics_vec.len()];
        let mut destination_vec = physics_vec.clone();
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
                        let r1_padding = compare1.shape.sprite.padding(compare1.shape.flipped);
                        let r2_padding = compare2.shape.sprite.padding(compare2.shape.flipped);
                        let r1_x1 = compare1.position.0 - (r1_padding.0 as f32 - rb1.padding.0);
                        let r1_x2 = compare1.position.0 + (r1_padding.2 as f32 - rb1.padding.2);
                        let r1_y1 = compare1.position.1 - (r1_padding.1 as f32 - rb1.padding.1);
                        let r1_y2 = compare1.position.1 + (r1_padding.3 as f32 - rb1.padding.3);

                        let r2_x1 = compare2.position.0 - (r2_padding.0 as f32 - rb2.padding.0);
                        let r2_x2 = compare2.position.0 + (r2_padding.2 as f32 - rb2.padding.2);
                        let r2_y1 = compare2.position.1 - (r2_padding.1 as f32 - rb2.padding.1);
                        let r2_y2 = compare2.position.1 + (r2_padding.3 as f32 - rb2.padding.3);

                        let collide = PhysicsSystem::check_collision(
                            (r1_x1, r1_y1, r1_x2, r1_y2),
                            (r2_x1, r2_y1, r2_x2, r2_y2),
                        );

                        if collide {
                            let r1_width = r1_x2 - r1_x1;
                            let r1_height = r1_y2 - r1_y1;
                            let x_vel_delta = compare1.velocity.0 - compare2.velocity.0;
                            let y_vel_delta = compare1.velocity.1 - compare2.velocity.1;
                            let up = (r2_y2 - r1_y1) / -y_vel_delta;
                            let down = (r1_y2 - r2_y1) / y_vel_delta;
                            let left = (r2_x2 - r1_x1) / -x_vel_delta;
                            let right = (r1_x2 - r2_x1) / x_vel_delta;
                            let x_time_delta = left.max(right);
                            let y_time_delta = up.max(down);

                            let (collided_left, collided_right) =
                                if x_vel_delta == 0.0 || x_time_delta > y_time_delta {
                                    (false, false)
                                } else {
                                    if right > 0.0 && right < r1_width {
                                        (false, true)
                                    } else if left > 0.0 && left < r1_width {
                                        (true, false)
                                    } else {
                                        (false, false)
                                    }
                                };

                            let (collided_up, collided_down) =
                                if y_vel_delta == 0.0 || y_time_delta > x_time_delta {
                                    (false, false)
                                } else {
                                    if up > 0.0 && up < r1_height {
                                        (true, false)
                                    } else if down > 0.0 && down < r1_height {
                                        (false, true)
                                    } else {
                                        (false, false)
                                    }
                                };

                            let collision1 =
                                [collided_left, collided_up, collided_right, collided_down];
                            let collision2 =
                                [collided_right, collided_down, collided_left, collided_up];

                            for k in 0..4 {
                                collisions[i][k] |= collision1[k];
                                collisions[j][k] |= collision2[k];
                            }

                            self.collisions.push(Collision {
                                entities: (compare1.entity, compare2.entity),
                                sides: (collision1, collision2),
                            });
                        }
                    }
                }
            }
        }

        for i in 0..length {
            let physics = &mut physics_vec[i];
            let [left_collision, up_collision, right_collision, down_collision] = collisions[i];

            if physics.velocity.0 > 0.0 && right_collision
                || physics.velocity.0 < 0.0 && left_collision
            {
                physics.velocity.0 = 0.0;
                physics.acceleration.0 = 0.0;
            }
            if physics.velocity.1 > 0.0 && down_collision
                || physics.velocity.1 < 0.0 && up_collision
            {
                physics.velocity.1 = 0.0;
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

impl TextureSprite {
    pub fn size(&self) -> (u32, u32) {
        (self.area.2 - self.area.0, self.area.3 - self.area.1)
    }
    pub fn padding(&self, flipped: (bool, bool)) -> (u32, u32, u32, u32) {
        let left = self.center.0 - self.area.0;
        let top = self.center.1 - self.area.1;
        let right = self.area.2 - self.center.0;
        let bottom = self.area.3 - self.center.1;

        if flipped.0 {
            if flipped.1 {
                (right, bottom, left, top)
            } else {
                (right, top, left, bottom)
            }
        } else {
            if flipped.1 {
                (left, bottom, right, top)
            } else {
                (left, top, right, bottom)
            }
        }
    }
}
