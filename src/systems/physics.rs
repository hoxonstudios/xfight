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
    pub padding: (i32, i32, i32, i32),
    pub solid: bool,
}
#[derive(Copy, Clone)]
pub struct Shape {
    pub texture_index: usize,
    pub flipped: (bool, bool),
    pub sprite: Sprite,
}
#[derive(Copy, Clone)]
pub struct Sprite {
    pub center: (i32, i32),
    pub area: (i32, i32, i32, i32),
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
                if let Some(rect1) = compare1.absolute_rigid_body() {
                    if let Some(rect2) = compare2.absolute_rigid_body() {
                        let collide = PhysicsSystem::check_collision(rect1, rect2);
                        if collide {
                            let (left, up, right, down) = PhysicsSystem::get_collision_sides(
                                rect1,
                                compare1.velocity,
                                rect2,
                                compare2.velocity,
                            );

                            let collision1 = [left, up, right, down];
                            let collision2 = [right, down, left, up];

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
    fn check_collision(rect1: (i32, i32, i32, i32), rect2: (i32, i32, i32, i32)) -> bool {
        let (r1_x1, r1_y1, r1_x2, r1_y2) = rect1;
        let (r2_x1, r2_y1, r2_x2, r2_y2) = rect2;

        let collide_in_x = r1_x1 <= r2_x2 && r1_x2 >= r2_x1;
        let collide_in_y = r1_y1 <= r2_y2 && r1_y2 >= r2_y1;
        let collide = collide_in_x && collide_in_y;

        return collide;
    }
    fn get_collision_sides(
        rect1: (i32, i32, i32, i32),
        velocity1: (f32, f32),
        rect2: (i32, i32, i32, i32),
        velocity2: (f32, f32),
    ) -> (bool, bool, bool, bool) {
        let (r1_x1, r1_y1, r1_x2, r1_y2) = rect1;
        let (r2_x1, r2_y1, r2_x2, r2_y2) = rect2;
        let (r1_vel_x, r1_vel_y) = velocity1;
        let (r2_vel_x, r2_vel_y) = velocity2;
        let r1_width = r1_x2 - r1_x1;
        let r1_height = r1_y2 - r1_y1;
        let x_vel_delta = r1_vel_x - r2_vel_x;
        let y_vel_delta = r1_vel_y - r2_vel_y;
        let up = (r2_y2 - r1_y1) as f32 / -y_vel_delta;
        let down = (r1_y2 - r2_y1) as f32 / y_vel_delta;
        let left = (r2_x2 - r1_x1) as f32 / -x_vel_delta;
        let right = (r1_x2 - r2_x1) as f32 / x_vel_delta;
        let x_time_delta = left.max(right);
        let y_time_delta = up.max(down);

        let (collided_left, collided_right) = if x_vel_delta == 0.0 || x_time_delta > y_time_delta {
            (false, false)
        } else {
            if right > 0.0 && right < r1_width as f32 {
                (false, true)
            } else if left > 0.0 && left < r1_width as f32 {
                (true, false)
            } else {
                (false, false)
            }
        };

        let (collided_up, collided_down) = if y_vel_delta == 0.0 || y_time_delta > x_time_delta {
            (false, false)
        } else {
            if up > 0.0 && up < r1_height as f32 {
                (true, false)
            } else if down > 0.0 && down < r1_height as f32 {
                (false, true)
            } else {
                (false, false)
            }
        };

        return (collided_left, collided_up, collided_right, collided_down);
    }
}

impl PhysicsComponent {
    pub fn absolute_position(&self) -> (i32, i32, i32, i32) {
        let sprite = self.shape.sprite;
        let x = self.position.0 as i32;
        let y = self.position.1 as i32;
        let left = sprite.area.0 - sprite.center.0;
        let top = sprite.area.1 - sprite.center.1;
        let right = sprite.area.2 - sprite.center.0;
        let bottom = sprite.area.3 - sprite.center.1;

        match self.shape.flipped {
            (false, false) => (x + left, y + top, x + right, y + bottom),
            (true, false) => (x - right, y + top, x - left, y + bottom),
            (false, true) => (x + left, y - bottom, x + right, y - top),
            (true, true) => (x - right, y - bottom, x - left, y - top),
        }
    }
    pub fn absolute_rigid_body(&self) -> Option<(i32, i32, i32, i32)> {
        if let Some(rigid_body) = self.rigid_body {
            let sprite = self.shape.sprite;
            let x = self.position.0 as i32;
            let y = self.position.1 as i32;
            let left = sprite.area.0 - sprite.center.0 + rigid_body.padding.0;
            let top = sprite.area.1 - sprite.center.1 + rigid_body.padding.1;
            let right = sprite.area.2 - sprite.center.0 - rigid_body.padding.2;
            let bottom = sprite.area.3 - sprite.center.1 - rigid_body.padding.3;

            Some(match self.shape.flipped {
                (false, false) => (x + left, y + top, x + right, y + bottom),
                (true, false) => (x - right, y + top, x - left, y + bottom),
                (false, true) => (x + left, y - bottom, x + right, y - top),
                (true, true) => (x - right, y - bottom, x - left, y - top),
            })
        } else {
            None
        }
    }
}
impl Sprite {
    pub fn size(&self) -> (i32, i32) {
        (self.area.2 - self.area.0, self.area.3 - self.area.1)
    }
}
