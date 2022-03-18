use super::{
    helpers::ComponentStore,
    physics::{PhysicsComponent, PhysicsSystem},
};

#[derive(Copy, Clone)]
pub struct DamageComponent {
    pub entity: usize,
    pub player: Player,
    pub consumed: bool,
    pub damage: Option<DamagePoint>,
}
#[derive(Copy, Clone)]
pub struct DamagePoint {
    pub point: (i32, i32),
    pub power: u32,
}

#[derive(Copy, Clone)]
pub struct HealthComponent {
    pub entity: usize,
    pub player: Player,
    pub health: u32,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Player {
    One,
    Two,
}

pub struct DamageSystem {
    pub damage_store: ComponentStore<DamageComponent>,
    pub health_store: ComponentStore<HealthComponent>,
}
impl DamageSystem {
    pub fn init() -> DamageSystem {
        DamageSystem {
            damage_store: ComponentStore::<DamageComponent>::init(),
            health_store: ComponentStore::<HealthComponent>::init(),
        }
    }
    pub fn update(&mut self, physics_system: &PhysicsSystem) {
        for damage in self.damage_store.data_mut() {
            if !damage.consumed {
                if let Some(damage_point) = damage.damage {
                    if let Some(damage_physics) = physics_system.store.get_component(damage.entity)
                    {
                        for health in self.health_store.data_mut() {
                            if health.player != damage.player {
                                if let Some(health_physics) =
                                    physics_system.store.get_component(health.entity)
                                {
                                    if let Some(rigid_body) = health_physics.absolute_rigid_body() {
                                        if DamageSystem::check_collision(
                                            DamageSystem::absolute_damage_point(
                                                damage_physics,
                                                &damage_point,
                                            ),
                                            rigid_body,
                                        ) {
                                            health.health -= damage_point.power;
                                            damage.consumed = true;
                                            println!("({}) = {}", health.entity, health.health);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn absolute_damage_point(physics: &PhysicsComponent, damage_point: &DamagePoint) -> (i32, i32) {
        let x = physics.position.0 as i32;
        let y = physics.position.1 as i32;
        match physics.shape.flipped {
            (false, false) => (x + damage_point.point.0, y + damage_point.point.1),
            (true, false) => (x - damage_point.point.0, y + damage_point.point.1),
            (false, true) => (x + damage_point.point.0, y - damage_point.point.1),
            (true, true) => (x - damage_point.point.0, y - damage_point.point.1),
        }
    }
    fn check_collision(damage_point: (i32, i32), rigid_body: (i32, i32, i32, i32)) -> bool {
        damage_point.0 >= rigid_body.0
            && damage_point.0 <= rigid_body.2
            && damage_point.1 >= rigid_body.1
            && damage_point.1 <= rigid_body.3
    }
}
