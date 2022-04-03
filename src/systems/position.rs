use super::helpers::ComponentStore;

#[derive(Copy, Clone)]
pub struct PositionComponent {
    pub entity: usize,
    pub action: PositionAction,
    pub x: f32,
    pub y: f32,
}

#[derive(Copy, Clone)]
pub enum PositionAction {
    None,
    Move { x: f32, y: f32 },
}

pub struct PositionSystem {
    pub store: ComponentStore<PositionComponent>,
}
impl PositionSystem {
    pub fn init() -> PositionSystem {
        PositionSystem {
            store: ComponentStore::<PositionComponent>::init(),
        }
    }
    pub fn update(&mut self) {
        for position in self.store.data_mut() {
            match position.action {
                PositionAction::None => {}
                PositionAction::Move { x, y } => {
                    position.x = x;
                    position.y = y;
                }
            }
            position.action = PositionAction::None;
        }
    }
}
