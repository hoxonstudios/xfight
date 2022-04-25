use crate::systems::{
    aim::{AimDirection, AimSystem},
    damage::DamageSystem,
    drawing::DrawingSystem,
    health::HealthSystem,
    helpers::component_store::ComponentStore,
    job::{FightJob, FightJobParameters},
    position::PositionSystem,
    tag::TagSystem,
    velocity::VelocitySystem,
};

use super::{MovementComponent, MovementTransitionCondition, MovementVelocityChange};

pub struct MovementSystem {
    pub store: ComponentStore<MovementComponent>,
}
impl MovementSystem {
    pub fn init() -> MovementSystem {
        MovementSystem {
            store: ComponentStore::<MovementComponent>::init(),
        }
    }
    pub fn update<'a>(
        &mut self,
        drawing_system: &mut DrawingSystem,
        velocity_system: &mut VelocitySystem,
        damage_system: &mut DamageSystem,
        health_system: &mut HealthSystem,
        aim_system: &AimSystem,
        tag_system: &TagSystem,
        position_system: &PositionSystem,
        jobs: &mut Vec<FightJob>,
    ) {
        for component in self.store.data_mut() {
            let entity = component.entity;

            let tag = tag_system.store.get_component(entity);
            let transition = component.movements[component.movement]
                .transitions
                .iter()
                .find(|t| match (component.frame, t.wait) {
                    (Some(_), true) => false,
                    _ => t.conditions.iter().all(|c| match c {
                        MovementTransitionCondition::NoneAction => match component.action {
                            None => true,
                            Some(_) => false,
                        },
                        MovementTransitionCondition::ActionNotActivated(action) => {
                            match component.action {
                                None => true,
                                Some(movement_action) => (action.0 & movement_action.0) == 0,
                            }
                        }
                        MovementTransitionCondition::ActionActivated(action) => {
                            match component.action {
                                None => false,
                                Some(movement_action) => (action.0 & movement_action.0) == action.0,
                            }
                        }
                        MovementTransitionCondition::StateActive(state) => match tag {
                            None => false,
                            Some(tag) => (state.0 & tag.actual_state.0) == state.0,
                        },
                        MovementTransitionCondition::StateInactive(state) => match tag {
                            None => true,
                            Some(tag) => (state.0 & tag.actual_state.0) == 0,
                        },
                    }),
                });

            if let Some(transition) = transition {
                component.frame = Some((0, 0));
                component.movement = transition.movement;
            } else if let Some(destroy_script) =
                component.movements[component.movement].destroy_script
            {
                jobs.push(FightJob {
                    script: destroy_script,
                    parameters: FightJobParameters::DestroyEntity { entity },
                });
            }

            let movement = &component.movements[component.movement];

            if let Some(frame) = component.frame {
                let movement_sprite = &movement.sprites[frame.0];
                let sprites = movement.sprites.len();

                if frame.1 == 0 {
                    if let Some(shape) = drawing_system.store.get_mut_component(entity) {
                        shape.sprite = movement_sprite.sprite;
                    }
                    if let Some(velocity_change) = movement_sprite.velocity_change {
                        if let Some(velocity) = velocity_system.store.get_mut_component(entity) {
                            match velocity_change {
                                MovementVelocityChange::Horizontal(velocity_x) => {
                                    velocity.velocity.0 = velocity_x;
                                }
                                MovementVelocityChange::Both(velocity_x, velocity_y) => {
                                    velocity.velocity.0 = velocity_x;
                                    velocity.velocity.1 = velocity_y;
                                }
                                MovementVelocityChange::HorizontalToAim(velocity_x) => {
                                    if let Some(aim) = aim_system.store.get_component(entity) {
                                        velocity.velocity.0 = match aim.direction {
                                            AimDirection::Left => -velocity_x,
                                            AimDirection::Right => velocity_x,
                                        };
                                    }
                                }
                            }
                        }
                    }

                    if let Some(damage) = damage_system.store.get_mut_component(entity) {
                        damage.damage = movement_sprite.damage_point;
                        if let Some(spell) = movement_sprite.spell {
                            if let Some(position) = position_system.store.get_component(entity) {
                                if let Some(aim) = aim_system.store.get_component(entity) {
                                    let position_x = match aim.direction {
                                        AimDirection::Left => position.x - spell.position.0,
                                        AimDirection::Right => position.x + spell.position.0,
                                    };
                                    jobs.push(FightJob {
                                        script: spell.script,
                                        parameters: FightJobParameters::SpawnSpell {
                                            position: (position_x, position.y + spell.position.1),
                                            direction: aim.direction,
                                            player: damage.player,
                                        },
                                    });
                                }
                            }
                        }
                    }

                    if let Some(health) = health_system.store.get_mut_component(entity) {
                        health.shield = movement_sprite.shield;
                    }
                }

                component.frame = next_frame(frame, (sprites, movement_sprite.frames));
            }
            component.action = None;
        }
    }
}

fn next_frame(frame: (usize, u8), frames: (usize, u8)) -> Option<(usize, u8)> {
    let next_frame = frame.1 + 1;
    if next_frame < frames.1 {
        Some((frame.0, next_frame))
    } else {
        let next_sprite = frame.0 + 1;
        if next_sprite < frames.0 {
            Some((next_sprite, 0))
        } else {
            None
        }
    }
}
