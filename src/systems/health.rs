use super::helpers::ComponentStore;

#[derive(Copy, Clone)]
pub struct HealthComponent {
    pub entity: usize,
    pub action: HealthAction,
    pub player: Player,
    pub health: u32,
}
#[derive(Copy, Clone)]
pub enum HealthAction {
    None,
    Consume { damage: u32 },
}
#[derive(Copy, Clone, PartialEq)]
pub enum Player {
    One,
    Two,
}

pub struct HealthSystem {
    pub store: ComponentStore<HealthComponent>,
}
impl HealthSystem {
    pub fn init() -> HealthSystem {
        HealthSystem {
            store: ComponentStore::<HealthComponent>::init(),
        }
    }
    pub fn update(&mut self) {
        for health in self.store.data_mut() {
            match health.action {
                HealthAction::None => {}
                HealthAction::Consume { damage } => {
                    health.health -= damage;
                }
            }
            health.action = HealthAction::None;
        }
    }
}
