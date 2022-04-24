use super::{
    helpers::component_store::ComponentStore,
    position::{PositionAction, PositionSystem},
};

const GRAVITY_ACCELERATION: f32 = 1.0;

#[derive(Copy, Clone)]
pub struct VelocityComponent {
    pub entity: usize,
    pub velocity: (f32, f32),
    pub gravity: bool,
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

            if let Some(position) = position_system.store.get_mut_component(entity) {
                if let PositionAction::None = position.action {
                    position.action = PositionAction::Move {
                        x: position.x + velocity.velocity.0,
                        y: position.y + velocity.velocity.1,
                    }
                }
            }

            velocity.velocity.1 += if velocity.gravity {
                GRAVITY_ACCELERATION
            } else {
                0.0
            };
        }
    }
}
