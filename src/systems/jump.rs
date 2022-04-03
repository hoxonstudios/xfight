use super::{
    helpers::ComponentStore,
    movement::{MovementAction, MovementSystem},
    shape::Sprite,
    velocity::{VelocityAction, VelocitySystem},
};

#[derive(Copy, Clone)]
pub struct JumpComponent {
    pub entity: usize,
    pub active: bool,
}
#[derive(Copy, Clone)]
pub struct JumpSprite {
    pub velocity_y: f32,
    pub sprite: Sprite,
}

pub struct JumpSystem {
    pub store: ComponentStore<JumpComponent>,
}
impl JumpSystem {
    pub fn init() -> JumpSystem {
        JumpSystem {
            store: ComponentStore::<JumpComponent>::init(),
        }
    }
    pub fn update(
        &mut self,
        movement_system: &mut MovementSystem,
        velocity_system: &mut VelocitySystem,
    ) {
        for jump in self.store.data_mut() {
            let entity = jump.entity;
            if let Some(movement) = movement_system.store.get_mut_component(entity) {
                if movement.grounded && !jump.active {
                    match movement.action {
                        Some(MovementAction::JumpStraight) => {
                            movement.action = None;
                            jump.active = true;
                            if let Some(velocity) = velocity_system.store.get_mut_component(entity)
                            {
                                velocity.action = VelocityAction::Change {
                                    velocity: (velocity.velocity.0, -10.0),
                                };
                            }
                        }
                        _ => {}
                    }
                } else if !movement.grounded && jump.active {
                } else if movement.grounded && jump.active {
                    jump.active = false;
                }
            }
        }
    }
}
