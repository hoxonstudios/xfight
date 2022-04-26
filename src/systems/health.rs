use super::helpers::component_store::ComponentStore;

#[derive(Copy, Clone)]
pub struct HealthComponent {
    pub entity: usize,
    pub action: HealthAction,
    pub player: Player,
    pub health: u32,
    pub shield: Option<Shield>,
}
#[derive(Copy, Clone)]
pub struct Shield {
    pub x0: i32,
    pub x1: i32,
    pub y0: i32,
    pub y1: i32,
    pub reduction: f32,
}

#[derive(Copy, Clone)]
pub enum HealthAction {
    None,
    Consume { damage: u32, shield: Option<f32> },
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
                HealthAction::Consume { damage, shield } => {
                    if let Some(shield) = shield {
                        health.health -= (damage as f32 * shield) as u32;
                    } else {
                        health.health -= damage;
                    }
                    println!("Player {} => {}", health.entity, health.health);
                }
            }
            health.action = HealthAction::None;
        }
    }
}
