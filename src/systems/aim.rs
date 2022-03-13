use super::{drawing::DrawingSystem, helpers::ComponentStore, physics::PhysicsSystem};

#[derive(Copy, Clone)]
pub struct AimComponent {
    pub entity: usize,
    pub direction: AimDirection,
}
#[derive(Copy, Clone)]
pub enum AimDirection {
    Left,
    Right,
}

pub struct AimSystem {
    pub store: ComponentStore<AimComponent>,
}
impl AimSystem {
    pub fn init() -> AimSystem {
        AimSystem {
            store: ComponentStore::<AimComponent>::init(),
        }
    }
    pub fn update<'a>(
        &mut self,
        physics_system: &PhysicsSystem,
        drawing_system: &mut DrawingSystem<'a>,
    ) {
        match self.store.data_mut().as_mut_slice() {
            [first_aim, second_aim] => {
                if let Some(first) = physics_system.store.get_component(first_aim.entity) {
                    if let Some(second) = physics_system.store.get_component(second_aim.entity) {
                        let first_direction = if first.position.0 < second.position.0 {
                            AimDirection::Right
                        } else {
                            AimDirection::Left
                        };
                        let second_direction = if first.position.0 < second.position.0 {
                            AimDirection::Left
                        } else {
                            AimDirection::Right
                        };
                        first_aim.direction = first_direction;
                        second_aim.direction = second_direction;

                        if let Some(first_shape) =
                            drawing_system.store.get_mut_component(first_aim.entity)
                        {
                            first_shape.flipped.0 = match first_direction {
                                AimDirection::Right => false,
                                AimDirection::Left => true,
                            };
                        }
                        if let Some(second_shape) =
                            drawing_system.store.get_mut_component(second_aim.entity)
                        {
                            second_shape.flipped.0 = match second_direction {
                                AimDirection::Right => false,
                                AimDirection::Left => true,
                            };
                        }
                    }
                }
            }
            _ => {}
        }
    }
}
