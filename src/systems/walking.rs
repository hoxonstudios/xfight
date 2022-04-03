use super::{
    helpers::ComponentStore,
    movement::{AimDirection, MovementAction, MovementComponent, MovementSystem},
    shape::{ShapeAction, ShapeSystem, Sprite},
    velocity::{VelocityAction, VelocitySystem},
};

const WALKING_SPRITES_COUNT: usize = 6;
const WALKING_FRAMES: u8 = 2;
const WALKING_VELOCITY: f32 = 2.0;

#[derive(Copy, Clone)]
pub struct WalkingComponent {
    pub entity: usize,
    pub direction: Option<WalkingDirection>,
    pub sprites: [Sprite; WALKING_SPRITES_COUNT],
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
        shape_system: &mut ShapeSystem,
        velocity_system: &mut VelocitySystem,
        movement_system: &MovementSystem,
    ) {
        for walking in self.store.data_mut() {
            let entity = walking.entity;
            if let Some(movement) = movement_system.store.get_component(entity) {
                let direction = match movement {
                    MovementComponent {
                        grounded: true,
                        attacking: false,
                        stunt: false,
                        action: Some(MovementAction::WalkLeft),
                        direction: AimDirection::Left,
                        ..
                    } => Some(WalkingDirection::Forward),
                    MovementComponent {
                        grounded: true,
                        attacking: false,
                        stunt: false,
                        action: Some(MovementAction::WalkRight),
                        direction: AimDirection::Right,
                        ..
                    } => Some(WalkingDirection::Forward),
                    MovementComponent {
                        grounded: true,
                        attacking: false,
                        stunt: false,
                        action: Some(MovementAction::WalkLeft),
                        direction: AimDirection::Right,
                        ..
                    } => Some(WalkingDirection::Backward),
                    MovementComponent {
                        grounded: true,
                        attacking: false,
                        stunt: false,
                        action: Some(MovementAction::WalkRight),
                        direction: AimDirection::Left,
                        ..
                    } => Some(WalkingDirection::Backward),
                    _ => None,
                };

                if direction != walking.direction {
                    walking.direction = direction;
                    walking.sprite_step = (0, 0);
                }
                if let Some(direction) = walking.direction {
                    if walking.sprite_step.1 >= WALKING_FRAMES {
                        walking.sprite_step.0 += 1;
                        walking.sprite_step.1 = 0;
                        if walking.sprite_step.0 >= WALKING_SPRITES_COUNT {
                            walking.sprite_step = (0, 0)
                        }
                    } else {
                        walking.sprite_step.1 += 1;
                    }
                    if let Some(shape) = shape_system.store.get_mut_component(entity) {
                        shape.action = ShapeAction::Update {
                            sprite: walking.sprites[walking.sprite_step.0],
                            flipped: shape.flipped,
                        };
                    }
                    if let Some(velocity) = velocity_system.store.get_mut_component(entity) {
                        let velocity_x = match (direction, movement.direction) {
                            (WalkingDirection::Backward, AimDirection::Right) => -WALKING_VELOCITY,
                            (WalkingDirection::Backward, AimDirection::Left) => WALKING_VELOCITY,
                            (WalkingDirection::Forward, AimDirection::Left) => -WALKING_VELOCITY,
                            (WalkingDirection::Forward, AimDirection::Right) => WALKING_VELOCITY,
                        };
                        velocity.action = VelocityAction::Change {
                            velocity: (velocity_x, velocity.velocity.1),
                        };
                    }
                }
            }
        }
    }
}
