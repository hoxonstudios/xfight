use super::{
    helpers::ComponentStore,
    movement::{AimDirection, MovementComponent, MovementSystem},
    physics::{PhysicsSystem, TextureSprite},
};

const STAND_SPRITE_COUNT: usize = 4;
const STAND_FRAMES: u8 = 3;

#[derive(Copy, Clone)]
pub struct StandComponent {
    pub entity: usize,
    pub sprites: [TextureSprite; 4],
    pub sprite_step: (usize, u8),
}

pub struct StandSystem {
    pub store: ComponentStore<StandComponent>,
}
impl StandSystem {
    pub fn init() -> StandSystem {
        StandSystem {
            store: ComponentStore::<StandComponent>::init(),
        }
    }
    pub fn update<'a>(
        &mut self,
        physics_system: &mut PhysicsSystem,
        movement_system: &MovementSystem,
    ) {
        for stand in self.store.data_mut() {
            let entity = stand.entity;
            if let Some(movement) = movement_system.store.get_component(entity) {
                let activated = match movement {
                    MovementComponent {
                        grounded: true,
                        attacking: false,
                        action: None,
                        ..
                    } => true,
                    _ => false,
                };
                if activated {
                    let (sprite, frame) = &mut stand.sprite_step;
                    if let Some(physics) = physics_system.store.get_mut_component(entity) {
                        physics.velocity.0 = 0.0;
                        if *frame >= STAND_FRAMES {
                            *sprite += 1;
                            *frame = 0;
                            if *sprite >= STAND_SPRITE_COUNT {
                                *sprite = 0;
                                *frame = 0;
                            }
                            physics.shape.sprite = stand.sprites[*sprite];
                            physics.shape.flipped.0 = match movement.direction {
                                AimDirection::Left => true,
                                AimDirection::Right => false,
                            };
                        } else {
                            *frame += 1;
                        }
                    }
                }
            }
        }
    }
}
