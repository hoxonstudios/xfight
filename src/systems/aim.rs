use super::{
    drawing::DrawingSystem, helpers::component_store::ComponentStore, position::PositionSystem,
};

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
    pub fn update(&mut self, position_system: &PositionSystem, drawing_system: &mut DrawingSystem) {
        let max_position = self.get_max_position(position_system);
        for aim in self.store.data_mut() {
            let entity = aim.entity;
            if let Some(shape) = drawing_system.store.get_mut_component(entity) {
                if let Some(position) = position_system.store.get_component(entity) {
                    let (flipped_x, direction) = if position.x < max_position {
                        (false, AimDirection::Right)
                    } else {
                        (true, AimDirection::Left)
                    };
                    aim.direction = direction;
                    shape.flipped.0 = flipped_x;
                }
            }
        }
    }
    fn get_max_position(&self, position_system: &PositionSystem) -> f32 {
        self.store
            .data()
            .iter()
            .map(|m| {
                if let Some(position) = position_system.store.get_component(m.entity) {
                    position.x
                } else {
                    0.0
                }
            })
            .reduce(f32::max)
            .unwrap_or_default()
    }
}
