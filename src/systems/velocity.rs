use super::{
    helpers::ComponentStore,
    position::{PositionAction, PositionSystem},
};

const GRAVITY_ACCELERATION: f32 = 0.5;

#[derive(Copy, Clone)]
pub struct VelocityComponent {
    pub entity: usize,
    pub action: VelocityAction,
    pub velocity: (f32, f32),
    pub acceleration: (f32, f32),
    pub gravity: bool,
}
#[derive(Copy, Clone)]
pub enum VelocityAction {
    None,
    Change { velocity: (f32, f32) },
}

pub struct VelocitySystem {
    pub store: ComponentStore<VelocityComponent>,
}
impl VelocitySystem {
    pub fn init() -> VelocitySystem {
        VelocitySystem {
            store: ComponentStore::<VelocityComponent>::init(),
        }
    }
    pub fn update(&mut self, position_system: &mut PositionSystem) {
        for velocity in self.store.data_mut() {
            let entity = velocity.entity;
            match velocity.action {
                VelocityAction::None => {
                    velocity.velocity.0 += velocity.acceleration.0;
                    velocity.velocity.1 += velocity.acceleration.1
                        + if velocity.gravity {
                            GRAVITY_ACCELERATION
                        } else {
                            0.0
                        };
                }
                VelocityAction::Change {
                    velocity: new_velocity,
                } => {
                    velocity.velocity = new_velocity;
                    velocity.acceleration = (0.0, 0.0);
                }
            }
            velocity.action = VelocityAction::None;
            if let Some(position) = position_system.store.get_mut_component(entity) {
                if let PositionAction::None = position.action {
                    position.action = PositionAction::Move {
                        x: position.x + velocity.velocity.0,
                        y: position.y + velocity.velocity.1,
                    }
                }
            }
        }
    }
}
