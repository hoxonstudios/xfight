use super::{
    damage::DamageSystem,
    helpers::ComponentStore,
    movement::MovementSystem,
    physics::{PhysicsSystem, Sprite},
};

const STUNT_FRAMES: u32 = 10;

#[derive(Copy, Clone)]
pub struct StunComponent {
    pub entity: usize,
    pub health: Option<u32>,
    pub sprite: Sprite,
    pub stunt_frame: Option<u32>,
}

pub struct StunSystem {
    pub store: ComponentStore<StunComponent>,
}
impl StunSystem {
    pub fn init() -> StunSystem {
        StunSystem {
            store: ComponentStore::<StunComponent>::init(),
        }
    }
    pub fn update(
        &mut self,
        damage_system: &DamageSystem,
        movement_system: &mut MovementSystem,
        physics_system: &mut PhysicsSystem,
    ) {
        for stun in self.store.data_mut() {
            let entity = stun.entity;
            if let Some(health) = damage_system.health_store.get_component(entity) {
                match stun.health {
                    None => {
                        stun.health = Some(health.health);
                    }
                    Some(previous_health) => match &mut stun.stunt_frame {
                        None => {
                            if let Some(movement) = movement_system.store.get_mut_component(entity)
                            {
                                if health.health < previous_health {
                                    movement.stunt = true;
                                    stun.stunt_frame = Some(0);
                                    if let Some(physics) =
                                        physics_system.store.get_mut_component(entity)
                                    {
                                        physics.shape.sprite = stun.sprite;
                                        physics.velocity.0 = 0.0;
                                    }
                                }
                            }
                            stun.health = Some(health.health);
                        }
                        Some(stunt_frame) => {
                            if *stunt_frame < STUNT_FRAMES {
                                *stunt_frame += 1;
                            } else {
                                if let Some(movement) =
                                    movement_system.store.get_mut_component(entity)
                                {
                                    movement.stunt = false;
                                    stun.stunt_frame = None;
                                }
                            }
                        }
                    },
                }
            }
        }
    }
}
