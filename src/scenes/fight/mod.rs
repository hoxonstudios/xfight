pub mod fighters;

use sdl2::{
    event::Event, keyboard::Keycode, render::TextureCreator, video::WindowContext, EventPump,
};

use crate::{
    components::{AnimatedArchetype, Entity},
    systems::{animation::AnimationSystem, drawing::DrawingSystem},
};

use self::fighters::ryu::{self, RYU_SPRITE_PATH};

pub struct FightScene {
    entities: Vec<Entity>,

    animated: AnimatedArchetype,
}

impl FightScene {
    pub fn init<'a>(
        texture_creator: &'a TextureCreator<WindowContext>,
        drawing_system: &mut DrawingSystem<'a>,
    ) -> FightScene {
        let mut scene = FightScene {
            entities: vec![],

            animated: AnimatedArchetype {
                shape: vec![],
                animation: vec![],
            },
        };
        drawing_system
            .load_texture(&texture_creator, RYU_SPRITE_PATH)
            .unwrap();

        // INIT FIGHTERS
        ryu::init(
            &mut scene.entities,
            &mut scene.animated,
            (100.0, 400.0),
            false,
        );
        ryu::init(
            &mut scene.entities,
            &mut scene.animated,
            (600.0, 400.0),
            true,
        );

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

            AnimationSystem::run(
                self.animated
                    .shape
                    .iter_mut()
                    .zip(self.animated.animation.iter_mut()),
            );
            drawing_system.run(self.animated.shape.iter())?;
        }

        Ok(())
    }
}
