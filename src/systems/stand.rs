use super::{
    helpers::ComponentStore,
    movement::{AimDirection, MovementComponent, MovementSystem},
    shape::{ShapeAction, ShapeSystem, Sprite},
    velocity::{VelocityAction, VelocitySystem},
};

const STAND_SPRITE_COUNT: usize = 4;
const STAND_FRAMES: u8 = 3;

#[derive(Copy, Clone)]
pub struct StandComponent {
    pub entity: usize,
    pub sprites: [Sprite; 4],
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
        movement_system: &MovementSystem,
        shape_system: &mut ShapeSystem,
        velocity_system: &mut VelocitySystem,
    ) {
        for stand in self.store.data_mut() {
            let entity = stand.entity;
            if let Some(movement) = movement_system.store.get_component(entity) {
                let activated = match movement {
                    MovementComponent {
                        grounded: true,
                        attacking: false,
                        stunt: false,
                        action: None,
                        ..
                    } => true,
                    _ => false,
                };
                if activated {
                    let (sprite, frame) = &mut stand.sprite_step;
                    if let Some(velocity) = velocity_system.store.get_mut_component(entity) {
                        velocity.action = VelocityAction::Change {
                            velocity: (0.0, velocity.velocity.1),
                        };
                    }
                    if *frame >= STAND_FRAMES {
                        *sprite += 1;
                        *frame = 0;
                        if *sprite >= STAND_SPRITE_COUNT {
                            *sprite = 0;
                            *frame = 0;
                        }
                        if let Some(shape) = shape_system.store.get_mut_component(entity) {
                            let flipped_x = match movement.direction {
                                AimDirection::Left => true,
                                AimDirection::Right => false,
                            };
                            shape.action = ShapeAction::Update {
                                sprite: stand.sprites[*sprite],
                                flipped: (flipped_x, shape.flipped.1),
                            };
                        }
                    } else {
                        *frame += 1;
                    }
                }
            }
        }
    }
}
