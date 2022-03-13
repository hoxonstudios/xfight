use super::{
    drawing::{DrawingSystem, TextureSprite},
    helpers::ComponentStore,
    movement::{AimDirection, MovementAction, MovementComponent, MovementSystem},
    physics::PhysicsSystem,
};

const WALKING_SPRITES_COUNT: usize = 6;
const WALKING_FRAMES: u8 = 2;
const WALKING_VELOCITY: f32 = 1.0;

#[derive(Copy, Clone)]
pub struct WalkingComponent {
    pub entity: usize,
    pub direction: Option<WalkingDirection>,
    pub sprites: [TextureSprite; WALKING_SPRITES_COUNT],
    pub sprite_step: (usize, u8),
}

#[derive(Copy, Clone, PartialEq)]
pub enum WalkingDirection {
    Forward,
    Backward,
}

pub struct WalkingSystem {
    pub store: ComponentStore<WalkingComponent>,
}
impl WalkingSystem {
    pub fn init() -> WalkingSystem {
        WalkingSystem {
            store: ComponentStore::<WalkingComponent>::init(),
        }
    }
    pub fn update<'a>(
        &mut self,
        physics_system: &mut PhysicsSystem,
        drawing_system: &mut DrawingSystem<'a>,
        movement_system: &MovementSystem,
    ) {
        for walking in self.store.data_mut() {
            let entity = walking.entity;
            if let Some(movement) = movement_system.store.get_component(entity) {
                let direction = match movement {
                    MovementComponent {
                        grounded: true,
                        attacking: false,
                        action: MovementAction::WalkLeft,
                        direction: AimDirection::Left,
                        ..
                    } => Some(WalkingDirection::Forward),
                    MovementComponent {
                        grounded: true,
                        attacking: false,
                        action: MovementAction::WalkRight,
                        direction: AimDirection::Right,
                        ..
                    } => Some(WalkingDirection::Forward),
                    MovementComponent {
                        grounded: true,
                        attacking: false,
                        action: MovementAction::WalkLeft,
                        direction: AimDirection::Right,
                        ..
                    } => Some(WalkingDirection::Backward),
                    MovementComponent {
                        grounded: true,
                        attacking: false,
                        action: MovementAction::WalkRight,
                        direction: AimDirection::Left,
                        ..
                    } => Some(WalkingDirection::Backward),
                    _ => None,
                };

                if direction != walking.direction {
                    walking.direction = direction;
                    walking.sprite_step = (0, 0);
                } else {
                    if walking.sprite_step.1 >= WALKING_FRAMES {
                        walking.sprite_step.0 += 1;
                        walking.sprite_step.1 = 0;
                        if walking.sprite_step.0 >= WALKING_SPRITES_COUNT {
                            walking.sprite_step = (0, 0)
                        }
                    } else {
                        walking.sprite_step.1 += 1;
                    }
                }
                if let Some(direction) = walking.direction {
                    if let Some(drawing) = drawing_system.store.get_mut_component(walking.entity) {
                        drawing.texture.sprite = walking.sprites[walking.sprite_step.0];
                    }
                    if let Some(physics) = physics_system.store.get_mut_component(walking.entity) {
                        physics.acceleration = (0.0, 0.0);
                        physics.velocity.0 = match (direction, movement.direction) {
                            (WalkingDirection::Backward, AimDirection::Right) => -WALKING_VELOCITY,
                            (WalkingDirection::Backward, AimDirection::Left) => WALKING_VELOCITY,
                            (WalkingDirection::Forward, AimDirection::Left) => -WALKING_VELOCITY,
                            (WalkingDirection::Forward, AimDirection::Right) => WALKING_VELOCITY,
                        };
                    }
                }
            }
        }
    }
}
