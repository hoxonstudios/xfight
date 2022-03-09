mod fighters;
mod floor;

use sdl2::{
    event::Event, keyboard::Keycode, render::TextureCreator, video::WindowContext, EventPump,
};

use crate::{
    components::{FighterArchetype, ShapeArchetype},
    systems::{self, drawing::DrawingSystem},
};

use self::{fighters::ryu::sprites::RYU_SPRITE_PATH, floor::sprites::FLOOR_SPRITE_PATH};

pub struct FightScene {
    shapes: ShapeArchetype,
    fighters: FighterArchetype,
}

impl FightScene {
    pub fn init<'a>(
        texture_creator: &'a TextureCreator<WindowContext>,
        drawing_system: &mut DrawingSystem<'a>,
    ) -> FightScene {
        let mut scene = FightScene {
            shapes: ShapeArchetype { shape: vec![] },
            fighters: FighterArchetype {
                shape: vec![],
                physics: vec![],
            },
        };
        drawing_system
            .load_textures(&texture_creator, &[RYU_SPRITE_PATH, FLOOR_SPRITE_PATH])
            .unwrap();

        // INIT SHAPES
        scene.shapes.init_floor();

        // INIT FIGHTERS
        scene.fighters.init_ryu((100.0, 350.0), false);
        scene.fighters.init_ryu((600.0, 350.0), true);

        return scene;
    }

    pub fn run<'a>(
        &mut self,
        event_pump: &mut EventPump,
        drawing_system: &mut DrawingSystem<'a>,
    ) -> Result<(), String> {
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }
            let _ = event_pump
                .keyboard_state()
                .pressed_scancodes()
                .filter_map(Keycode::from_scancode)
                .collect::<Vec<Keycode>>();

            systems::physics::run(
                self.fighters
                    .physics
                    .iter_mut()
                    .zip(self.fighters.shape.iter_mut()),
            );
            drawing_system.run(self.shapes.shape.iter().chain(self.fighters.shape.iter()))?;
        }

        Ok(())
    }
}
