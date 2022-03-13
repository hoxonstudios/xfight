use super::{
    drawing::{DrawingSystem, TextureSprite},
    helpers::ComponentStore,
    physics::PhysicsSystem,
};

const WALKING_SPRITES_COUNT: usize = 6;
const WALKING_FRAMES_PER_SPRITE: u8 = 2;
const WALKING_VELOCITY: f32 = 1.0;

#[derive(Copy, Clone)]
pub struct WalkingComponent {
    pub entity: usize,
    pub activated: Option<WalkingDirection>,
    pub direction: Option<WalkingDirection>,
    pub sprites: [TextureSprite; WALKING_SPRITES_COUNT],
    pub sprite_step: (usize, u8),
}

#[derive(Copy, Clone, PartialEq)]
pub enum WalkingDirection {
    ForwardToRight,
    ForwardToLeft,
    BackwardToRight,
    BackwardToLeft,
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
    ) {
        for walking in self.store.data_mut() {
            if walking.activated != walking.direction {
                walking.direction = walking.activated;
                walking.sprite_step = (0, 0);
            } else {
                if walking.sprite_step.1 >= WALKING_FRAMES_PER_SPRITE {
                    walking.sprite_step.0 += 1;
                    walking.sprite_step.1 = 0;
                } else {
                    walking.sprite_step.1 += 1;
                }
                if walking.sprite_step.0 >= WALKING_SPRITES_COUNT {
                    walking.sprite_step = (0, 0)
                }
            }
            if let Some(direction) = walking.direction {
                if let Some(drawing) = drawing_system.store.get_mut_component(walking.entity) {
                    drawing.texture.sprite = walking.sprites[walking.sprite_step.0];
                    drawing.flipped = match direction {
                        WalkingDirection::ForwardToLeft => (true, false),
                        WalkingDirection::BackwardToLeft => (false, false),
                        WalkingDirection::BackwardToRight => (true, false),
                        WalkingDirection::ForwardToRight => (false, false),
                    };
                }
                if let Some(physics) = physics_system.store.get_mut_component(walking.entity) {
                    physics.acceleration = (0.0, 0.0);
                    physics.velocity.0 = match direction {
                        WalkingDirection::BackwardToLeft => -WALKING_VELOCITY,
                        WalkingDirection::BackwardToRight => WALKING_VELOCITY,
                        WalkingDirection::ForwardToLeft => -WALKING_VELOCITY,
                        WalkingDirection::ForwardToRight => WALKING_VELOCITY,
                    };
                }
            }
        }
    }
}
