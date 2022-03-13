use super::{helpers::ComponentStore, physics::PhysicsSystem};

#[derive(Copy, Clone)]
pub struct MovementComponent {
    pub entity: usize,
    pub action: MovementAction,
    pub direction: AimDirection,
    pub grounded: bool,
    pub attacking: bool,
}
#[derive(Copy, Clone)]
pub enum MovementAction {
    None,
    WalkRight,
    WalkLeft,
    Punch,
}
#[derive(Copy, Clone)]
pub enum AimDirection {
    Left,
    Right,
}

pub struct MovementSystem {
    pub store: ComponentStore<MovementComponent>,
}
impl MovementSystem {
    pub fn init() -> MovementSystem {
        MovementSystem {
            store: ComponentStore::<MovementComponent>::init(),
        }
    }
    pub fn update<'a>(&mut self, physics_system: &mut PhysicsSystem) {
        let max_position = self.get_max_position(physics_system);
        for movement in self.store.data_mut() {
            let entity = movement.entity;
            movement.grounded = MovementSystem::is_grounded(entity, physics_system);

            if let Some(physics) = physics_system.store.get_mut_component(entity) {
                movement.direction = if physics.position.0 < max_position {
                    AimDirection::Right
                } else {
                    AimDirection::Left
                };
                physics.shape.flipped.0 = match movement.direction {
                    AimDirection::Right => false,
                    AimDirection::Left => true,
                };
            }
        }
    }
    fn is_grounded(entity: usize, physics_system: &PhysicsSystem) -> bool {
        physics_system
            .collisions
            .iter()
            .find(|&c| {
                (c.entities.0 == entity && c.sides.0[3]) || (c.entities.1 == entity && c.sides.1[3])
            })
            .is_some()
    }
    fn get_max_position(&self, physics_system: &PhysicsSystem) -> f32 {
        self.store
            .data()
            .iter()
            .map(|m| {
                if let Some(physics) = physics_system.store.get_component(m.entity) {
                    physics.position.0
                } else {
                    0.0
                }
            })
            .reduce(f32::max)
            .unwrap_or_default()
    }
}
