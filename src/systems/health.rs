use super::{
    helpers::component_store::ComponentStore,
    tag::{StateTag, TagSystem},
};

#[derive(Copy, Clone)]
pub struct HealthComponent {
    pub entity: usize,
    pub action: HealthAction,
    pub player: Player,
    pub health: i32,
    pub shield: Option<Shield>,
    pub dead_tag: StateTag,
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
    pub fn update(&mut self, tag_system: &mut TagSystem) {
        for health in self.store.data_mut() {
            let entity = health.entity;
            match health.action {
                HealthAction::None => {}
                HealthAction::Consume { damage, shield } => {
                    if health.health > 0 {
                        if let Some(shield) = shield {
                            health.health -= (damage as f32 * shield) as i32;
                        } else {
                            health.health -= damage as i32;
                        }
                        if health.health <= 0 {
                            if let Some(tag) = tag_system.store.get_mut_component(entity) {
                                tag.next_state.0 |= health.dead_tag.0;
                            }
                        }
                        println!("Player {} => {}", entity, health.health);
                    }
                }
            }
            health.action = HealthAction::None;
        }
    }
}
