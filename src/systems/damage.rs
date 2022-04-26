use crate::systems::health::HealthAction;

use super::{
    collision::CollisionSystem,
    drawing::{DrawingSystem, Sprite},
    health::{HealthSystem, Player, Shield},
    helpers::component_store::ComponentStore,
    position::PositionSystem,
    tag::{StateTag, TagSystem},
};

#[derive(Copy, Clone)]
pub struct DamageComponent {
    pub entity: usize,
    pub player: Player,
    pub damage: Option<Damage>,
}
#[derive(Copy, Clone)]
pub struct Damage {
    pub area: DamageArea,
    pub power: u32,
    pub tag: StateTag,
}
#[derive(Copy, Clone)]
pub enum DamageArea {
    RelativePoint(i32, i32),
    EntireShape,
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
        tag_system: &mut TagSystem,
        collision_system: &CollisionSystem,
        position_system: &PositionSystem,
        drawing_system: &DrawingSystem,
    ) {
        for damage in self.store.data_mut() {
            if let Some(damage_point) = damage.damage {
                if let Some(damage_position) =
                    position_system.store.get_component_ref(damage.entity)
                {
                    if let Some(damage_shape) =
                        drawing_system.store.get_component_ref(damage.entity)
                    {
                        for health in health_system.store.data_mut() {
                            if health.player != damage.player {
                                if let Some(health_shape) =
                                    drawing_system.store.get_component_ref(health.entity)
                                {
                                    if let Some(health_position) =
                                        position_system.store.get_component_ref(health.entity)
                                    {
                                        if let Some(health_collision) =
                                            collision_system.store.get_component_ref(health.entity)
                                        {
                                            let health_rect = absolute_rect(
                                                &health_shape.sprite,
                                                health_collision.padding,
                                                (health_position.x, health_position.y),
                                                health_shape.flipped,
                                            );

                                            let collide = match damage_point.area {
                                                DamageArea::RelativePoint(damage_x, damage_y) => {
                                                    let point = absolute_damage_point(
                                                        (
                                                            damage_position.x as i32,
                                                            damage_position.y as i32,
                                                        ),
                                                        damage_shape.flipped,
                                                        (damage_x, damage_y),
                                                    );
                                                    check_collision(point, health_rect)
                                                }
                                                DamageArea::EntireShape => {
                                                    let damage_rect = absolute_rect(
                                                        &damage_shape.sprite,
                                                        0,
                                                        (damage_position.x, damage_position.y),
                                                        damage_shape.flipped,
                                                    );
                                                    check_rect_collision(damage_rect, health_rect)
                                                }
                                            };

                                            if collide {
                                                let shield = match health.shield {
                                                    None => None,
                                                    Some(shield) => {
                                                        let shield_rect = absolute_shield_rect(
                                                            &shield,
                                                            (health_position.x, health_position.y),
                                                            health_shape.flipped,
                                                        );

                                                        let shield_collide = match damage_point.area
                                                        {
                                                            DamageArea::RelativePoint(
                                                                damage_x,
                                                                damage_y,
                                                            ) => {
                                                                let point = absolute_damage_point(
                                                                    (
                                                                        damage_position.x as i32,
                                                                        damage_position.y as i32,
                                                                    ),
                                                                    damage_shape.flipped,
                                                                    (damage_x, damage_y),
                                                                );
                                                                check_collision(point, shield_rect)
                                                            }
                                                            DamageArea::EntireShape => {
                                                                let damage_rect = absolute_rect(
                                                                    &damage_shape.sprite,
                                                                    0,
                                                                    (
                                                                        damage_position.x,
                                                                        damage_position.y,
                                                                    ),
                                                                    damage_shape.flipped,
                                                                );
                                                                check_rect_collision(
                                                                    damage_rect,
                                                                    shield_rect,
                                                                )
                                                            }
                                                        };

                                                        if shield_collide {
                                                            Some(shield.reduction)
                                                        } else {
                                                            None
                                                        }
                                                    }
                                                };

                                                health.action = HealthAction::Consume {
                                                    damage: damage_point.power,
                                                    shield,
                                                };

                                                if let Some(tag) = tag_system
                                                    .store
                                                    .get_mut_component(health.entity)
                                                {
                                                    tag.next_state.0 |= damage_point.tag.0;
                                                }
                                                damage.damage = None;
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
}

fn absolute_damage_point(
    position: (i32, i32),
    flipped: (bool, bool),
    damage_point: (i32, i32),
) -> (i32, i32) {
    let (x, y) = position;

    match flipped {
        (false, false) => (x + damage_point.0, y + damage_point.1),
        (true, false) => (x - damage_point.0, y + damage_point.1),
        (false, true) => (x + damage_point.0, y - damage_point.1),
        (true, true) => (x - damage_point.0, y - damage_point.1),
    }
}
fn absolute_rect(
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
fn check_rect_collision(
    damage_rect: (i32, i32, i32, i32),
    rigid_body: (i32, i32, i32, i32),
) -> bool {
    damage_rect.0 <= rigid_body.2
        && damage_rect.2 >= rigid_body.0
        && damage_rect.1 <= rigid_body.3
        && damage_rect.3 >= rigid_body.1
}
