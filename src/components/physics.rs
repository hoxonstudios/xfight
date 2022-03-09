#[derive(Copy, Clone)]
pub struct PhysicsComponent {
    pub velocity: (f32, f32),
    pub acceleration: (f32, f32),
    pub gravity: bool,
}
