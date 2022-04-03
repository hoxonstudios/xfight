use crate::systems::health::HealthAction;

use super::{
    collision::CollisionSystem,
    health::{HealthSystem, Player},
    helpers::ComponentStore,
    position::PositionSystem,
    shape::ShapeSystem,
};

#[derive(Copy, Clone)]
pub struct DamageComponent {
    pub entity: usize,
    pub action: DamageAction,
    pub player: Player,
    pub damage: Option<DamagePoint>,
}
#[derive(Copy, Clone)]
pub enum DamageAction {
    None,
    Inflict { point: DamagePoint },
}
#[derive(Copy, Clone)]
pub struct DamagePoint {
    pub point: (i32, i32),
    pub power: u32,
}

pub struct DamageSystem {
    pub store: ComponentStore<DamageComponent>,
}
impl DamageSystem {
    pub fn init() -> DamageSystem {
        DamageSystem {
            store: ComponentStore::<DamageComponent>::init(),
        }
    }
    pub fn update(
        &mut self,
        health_system: &mut HealthSystem,
        collision_system: &CollisionSystem,
        position_system: &PositionSystem,
        shape_system: &ShapeSystem,
    ) {
        for damage in self.store.data_mut() {
            match damage.action {
                DamageAction::None => {}
                DamageAction::Inflict { point } => {
                    damage.damage = Some(point);
                }
            }
            damage.action = DamageAction::None;
            if let Some(damage_point) = damage.damage {
                if let Some(damage_position) = position_system.store.get_component(damage.entity) {
                    if let Some(damage_shape) = shape_system.store.get_component(damage.entity) {
                        for health in health_system.store.data_mut() {
                            if health.player != damage.player {
                                if let Some(health_shape) =
                                    shape_system.store.get_component(health.entity)
                                {
                                    if let Some(health_position) =
                                        position_system.store.get_component(health.entity)
                                    {
                                        if let Some(health_collision) =
                                            collision_system.store.get_component(health.entity)
                                        {
                                            let rect =
                                                health_shape.sprite.rect(health_shape.flipped);
                                            let x1 = rect.0
                                                + health_position.x as i32
                                                + health_collision.padding;
                                            let y1 = rect.1
                                                + health_position.y as i32
                                                + health_collision.padding;
                                            let x2 = rect.2 + health_position.x as i32
                                                - health_collision.padding;
                                            let y2 = rect.3 + health_position.y as i32
                                                - health_collision.padding;

                                            if DamageSystem::check_collision(
                                                DamageSystem::absolute_damage_point(
                                                    (
                                                        damage_position.x as i32,
                                                        damage_position.y as i32,
                                                    ),
                                                    damage_shape.flipped,
                                                    &damage_point,
                                                ),
                                                (x1, y1, x2, y2),
                                            ) {
                                                health.action = HealthAction::Consume {
                                                    damage: damage_point.power,
                                                };
                                                damage.damage = None;
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
    }

    fn absolute_damage_point(
        position: (i32, i32),
        flipped: (bool, bool),
        damage_point: &DamagePoint,
    ) -> (i32, i32) {
        let (x, y) = position;

        match flipped {
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
