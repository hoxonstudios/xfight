use super::{
    drawing::{DrawingSystem, TextureSprite},
    helpers::ComponentStore,
    movement::{MovementAction, MovementSystem},
};

const PUNCH_SPRITE_COUNT: usize = 3;
const PUNCH_FRAMES: u8 = 3;

#[derive(Copy, Clone)]
pub struct PunchComponent {
    pub entity: usize,
    pub active: bool,
    pub sprites: [TextureSprite; PUNCH_SPRITE_COUNT],
    pub sprite_step: (usize, u8),
}

pub struct PunchSystem {
    pub store: ComponentStore<PunchComponent>,
}
impl PunchSystem {
    pub fn init() -> PunchSystem {
        PunchSystem {
            store: ComponentStore::<PunchComponent>::init(),
        }
    }
    pub fn update<'a>(
        &mut self,
        drawing_system: &mut DrawingSystem<'a>,
        movement_system: &mut MovementSystem,
    ) {
        for punch in self.store.data_mut() {
            let entity = punch.entity;
            let (sprite, frame) = &mut punch.sprite_step;
            if let Some(movement) = movement_system.store.get_mut_component(entity) {
                if punch.active {
                    if *frame >= PUNCH_FRAMES {
                        *frame = 0;
                        *sprite += 1;
                        if *sprite >= PUNCH_SPRITE_COUNT {
                            *sprite = 0;
                            punch.active = false;
                            movement.attacking = false;
                        }
                        if let Some(drawing) = drawing_system.store.get_mut_component(entity) {
                            drawing.texture.sprite = punch.sprites[*sprite];
                        }
                    } else {
                        *frame += 1;
                    }
                }
                if let MovementAction::Punch = movement.action {
                    if !movement.attacking {
                        movement.attacking = true;
                        punch.active = true;
                        *sprite = 0;
                        *frame = 0;
                        if let Some(drawing) = drawing_system.store.get_mut_component(entity) {
                            drawing.texture.sprite = punch.sprites[*sprite];
                        }
                    }
                }
            }
        }
    }
}
