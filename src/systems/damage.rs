use crate::systems::{health::HealthAction, movement::MovementAction};

use super::{
    collision::CollisionSystem,
    health::{HealthSystem, Player, Shield},
    helpers::ComponentStore,
    movement::MovementSystem,
    position::PositionSystem,
    shape::{ShapeSystem, Sprite},
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
        movement_system: &mut MovementSystem,
    ) {
        for damage in self.store.data_mut() {
            match damage.action {
                DamageAction::None => {}
                DamageAction::Inflict { point } => {
                    damage.damage = Some(point);
                }
            }
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
                                            let point = absolute_damage_point(
                                                (
                                                    damage_position.x as i32,
                                                    damage_position.y as i32,
                                                ),
                                                damage_shape.flipped,
                                                &damage_point,
                                            );
                                            let health_rect = absolute_health_rect(
                                                &health_shape.sprite,
                                                health_collision.padding,
                                                (health_position.x, health_position.y),
                                                health_shape.flipped,
                                            );

                                            if check_collision(point, health_rect) {
                                                let shield = match health.shield {
                                                    None => None,
                                                    Some(shield) => {
                                                        let shield_rect = absolute_shield_rect(
                                                            &shield,
                                                            (health_position.x, health_position.y),
                                                            health_shape.flipped,
                                                        );

                                                        if check_collision(point, shield_rect) {
                                                            Some(shield.reduction)
                                                        } else {
                                                            None
                                                        }
                                                    }
                                                };

                                                let damage_power = match shield {
                                                    None => damage_point.power,
                                                    Some(shield) => {
                                                        (damage_point.power as f32 * shield) as u32
                                                    }
                                                };

                                                health.action = HealthAction::Consume {
                                                    damage: damage_power,
                                                };
                                                damage.damage = None;
                                                println!("({}) = {}", health.entity, health.health);

                                                if let Some(movement) = movement_system
                                                    .store
                                                    .get_mut_component(health.entity)
                                                {
                                                    movement.action = if shield.is_some() {
                                                        Some(MovementAction::BlockedHit)
                                                    } else {
                                                        Some(MovementAction::Hit)
                                                    };
                                                    println!(
                                                        "({}) = {} | {:?}",
                                                        health.entity,
                                                        health.health,
                                                        movement.action
                                                    );
                                                };
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            damage.action = DamageAction::None;
            damage.damage = None;
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
fn absolute_health_rect(
    sprite: &Sprite,
    padding: i32,
    position: (f32, f32),
    flipped: (bool, bool),
) -> (i32, i32, i32, i32) {
    let rect = sprite.rect(flipped);
    let x1 = rect.0 + position.0 as i32 + padding;
    let y1 = rect.1 + position.1 as i32 + padding;
    let x2 = rect.2 + position.0 as i32 - padding;
    let y2 = rect.3 + position.1 as i32 - padding;

    (x1, y1, x2, y2)
}
fn absolute_shield_rect(
    shield: &Shield,
    position: (f32, f32),
    flipped: (bool, bool),
) -> (i32, i32, i32, i32) {
    let left = if flipped.0 { -shield.x1 } else { shield.x0 };
    let top = if flipped.1 { -shield.y1 } else { shield.y0 };
    let right = if flipped.0 { -shield.x0 } else { shield.x1 };
    let bottom = if flipped.1 { -shield.y0 } else { shield.y1 };

    (
        position.0 as i32 + left,
        position.1 as i32 + top,
        position.0 as i32 + right,
        position.1 as i32 + bottom,
    )
}
fn check_collision(damage_point: (i32, i32), rigid_body: (i32, i32, i32, i32)) -> bool {
    damage_point.0 >= rigid_body.0
        && damage_point.0 <= rigid_body.2
        && damage_point.1 >= rigid_body.1
        && damage_point.1 <= rigid_body.3
}
