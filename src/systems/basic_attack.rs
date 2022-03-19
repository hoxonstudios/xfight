use super::{
    damage::{DamagePoint, DamageSystem},
    helpers::ComponentStore,
    movement::{MovementAction, MovementSystem},
    physics::{PhysicsSystem, Sprite},
};

const LIGHT_PUNCH_SPRITES_COUNT: (usize, u8) = (3, 2);
const STRONG_PUNCH_SPRITES_COUNT: (usize, u8) = (5, 2);
const LIGHT_KICK_SPRITES_COUNT: (usize, u8) = (3, 2);
const STRONG_KICK_SPRITES_COUNT: (usize, u8) = (3, 2);

#[derive(Copy, Clone)]
pub struct BasicAttackComponent {
    pub entity: usize,
    pub active: Option<BasicAttackMovement>,
    pub sprite_step: (usize, u8),
    pub light_punch: [(Sprite, Option<DamagePoint>); LIGHT_PUNCH_SPRITES_COUNT.0],
    pub strong_punch: [(Sprite, Option<DamagePoint>); STRONG_PUNCH_SPRITES_COUNT.0],
    pub light_kick: [(Sprite, Option<DamagePoint>); LIGHT_KICK_SPRITES_COUNT.0],
    pub strong_kick: [(Sprite, Option<DamagePoint>); STRONG_KICK_SPRITES_COUNT.0],
}
#[derive(Copy, Clone)]
pub enum BasicAttackMovement {
    LightPunch,
    StrongPunch,
    LightKick,
    StrongKick,
}

pub struct BasicAttackSystem {
    pub store: ComponentStore<BasicAttackComponent>,
}
impl BasicAttackSystem {
    pub fn init() -> BasicAttackSystem {
        BasicAttackSystem {
            store: ComponentStore::<BasicAttackComponent>::init(),
        }
    }
    pub fn update<'a>(
        &mut self,
        physics_system: &mut PhysicsSystem,
        movement_system: &mut MovementSystem,
        damage_system: &mut DamageSystem,
    ) {
        for attack in self.store.data_mut() {
            let entity = attack.entity;
            let (sprite_index, frame) = &mut attack.sprite_step;
            if let Some(movement) = movement_system.store.get_mut_component(entity) {
                if let Some(physics) = physics_system.store.get_mut_component(entity) {
                    let able_to_attack = !movement.attacking && !movement.stunt;
                    if able_to_attack {
                        if let Some(action) = movement.action {
                            let attack_movement = match action {
                                MovementAction::LightPunch => Some(BasicAttackMovement::LightPunch),
                                MovementAction::StrongPunch => {
                                    Some(BasicAttackMovement::StrongPunch)
                                }
                                MovementAction::LightKick => Some(BasicAttackMovement::LightKick),
                                MovementAction::StrongKick => Some(BasicAttackMovement::StrongKick),
                                _ => None,
                            };
                            if let Some(_) = attack_movement {
                                movement.attacking = true;
                                attack.active = attack_movement;
                                *sprite_index = 0;
                                *frame = 0;
                                physics.velocity.0 = 0.0;
                            }
                        }
                    }
                    if let Some(active) = attack.active {
                        let (sprites, frames) = BasicAttackSystem::get_movement_sprites(&active);
                        let (sprite, damage_point) = match active {
                            BasicAttackMovement::LightPunch => attack.light_punch[*sprite_index],
                            BasicAttackMovement::StrongPunch => attack.strong_punch[*sprite_index],
                            BasicAttackMovement::LightKick => attack.light_kick[*sprite_index],
                            BasicAttackMovement::StrongKick => attack.strong_kick[*sprite_index],
                        };
                        if *frame == 0 {
                            physics.shape.sprite = sprite;
                            if let Some(damage) =
                                damage_system.damage_store.get_mut_component(entity)
                            {
                                damage.damage = damage_point;
                                damage.consumed = false;
                            }
                        }
                        if *frame < frames {
                            *frame += 1;
                        } else {
                            *frame = 0;
                            *sprite_index += 1;
                            if *sprite_index >= sprites {
                                *sprite_index = 0;
                                attack.active = None;
                                movement.attacking = false;
                            }
                        }
                    }
                }
            }
        }
    }

    fn get_movement_sprites(movement: &BasicAttackMovement) -> (usize, u8) {
        match &movement {
            BasicAttackMovement::LightPunch => LIGHT_PUNCH_SPRITES_COUNT,
            BasicAttackMovement::StrongPunch => STRONG_PUNCH_SPRITES_COUNT,
            BasicAttackMovement::LightKick => LIGHT_KICK_SPRITES_COUNT,
            BasicAttackMovement::StrongKick => STRONG_KICK_SPRITES_COUNT,
        }
    }
}
