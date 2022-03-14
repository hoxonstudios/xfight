use super::{
    helpers::ComponentStore,
    movement::{MovementAction, MovementSystem},
    physics::{PhysicsSystem, TextureSprite},
};

const LIGHT_PUNCH_SPRITES_COUNT: (usize, u8) = (3, 2);
const STRONG_PUNCH_SPRITES_COUNT: (usize, u8) = (5, 2);

#[derive(Copy, Clone)]
pub struct BasicAttackComponent {
    pub entity: usize,
    pub active: Option<BasicAttackMovement>,
    pub sprite_step: (usize, u8),
    pub light_punch: [TextureSprite; LIGHT_PUNCH_SPRITES_COUNT.0],
    pub strong_punch: [TextureSprite; STRONG_PUNCH_SPRITES_COUNT.0],
}
#[derive(Copy, Clone)]
pub enum BasicAttackMovement {
    LightPunch,
    StrongPunch,
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
    ) {
        for attack in self.store.data_mut() {
            let entity = attack.entity;
            let (sprite, frame) = &mut attack.sprite_step;
            if let Some(movement) = movement_system.store.get_mut_component(entity) {
                if let Some(physics) = physics_system.store.get_mut_component(entity) {
                    if let Some(active) = attack.active {
                        let (sprites, frames) = BasicAttackSystem::get_movement_sprites(&active);
                        if *frame >= frames {
                            *frame = 0;
                            *sprite += 1;
                            if *sprite >= sprites {
                                *sprite = 0;
                                attack.active = None;
                                movement.attacking = false;
                            }
                            match active {
                                BasicAttackMovement::LightPunch => {
                                    physics.shape.sprite = attack.light_punch[*sprite];
                                }
                                BasicAttackMovement::StrongPunch => {
                                    physics.shape.sprite = attack.strong_punch[*sprite];
                                }
                            }
                        } else {
                            *frame += 1;
                        }
                    } else {
                        if !movement.attacking {
                            if let Some(action) = movement.action {
                                match action {
                                    MovementAction::LightPunch => {
                                        movement.attacking = true;
                                        attack.active = Some(BasicAttackMovement::LightPunch);
                                        *sprite = 0;
                                        *frame = 0;
                                        physics.velocity.0 = 0.0;
                                        physics.shape.sprite = attack.light_punch[*sprite];
                                    }
                                    MovementAction::StrongPunch => {
                                        movement.attacking = true;
                                        attack.active = Some(BasicAttackMovement::StrongPunch);
                                        *sprite = 0;
                                        *frame = 0;
                                        physics.velocity.0 = 0.0;
                                        physics.shape.sprite = attack.strong_punch[*sprite];
                                    }
                                    _ => {}
                                };
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
        }
    }
}
