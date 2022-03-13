use super::{
    drawing::{DrawingSystem, TextureSprite},
    helpers::ComponentStore,
};

const STAND_SPRITE_COUNT: usize = 4;
const STAND_FRAMES: u8 = 3;

#[derive(Copy, Clone)]
pub struct StandComponent {
    pub entity: usize,
    pub activated: bool,
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
    pub fn update<'a>(&mut self, drawing_system: &mut DrawingSystem<'a>) {
        for stand in self.store.data_mut() {
            if stand.activated {
                let (sprite, frame) = &mut stand.sprite_step;
                if *frame >= STAND_FRAMES {
                    *sprite += 1;
                    *frame = 0;
                    if *sprite >= STAND_SPRITE_COUNT {
                        *sprite = 0;
                        *frame = 0;
                    }
                    if let Some(shape) = drawing_system.store.get_mut_component(stand.entity) {
                        shape.texture.sprite = stand.sprites[*sprite];
                    }
                } else {
                    *frame += 1;
                }
            }
        }
    }
}
