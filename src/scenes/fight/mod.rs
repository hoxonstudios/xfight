pub mod fighters;

use sdl2::{
    event::Event, keyboard::Keycode, render::TextureCreator, video::WindowContext, EventPump,
};

use crate::{
    components::{
        fighter::{Direction, Player},
        Entity, FighterArchetype,
    },
    systems::{animation::AnimationSystem, drawing::DrawingSystem, fight::FightSystem},
};

use self::fighters::ryu::{self, RYU_SPRITE_PATH};

pub struct FightScene {
    entities: Vec<Entity>,

    fighter: FighterArchetype,
}

impl FightScene {
    pub fn init<'a>(
        texture_creator: &'a TextureCreator<WindowContext>,
        drawing_system: &mut DrawingSystem<'a>,
    ) -> FightScene {
        let mut scene = FightScene {
            entities: vec![],

            fighter: FighterArchetype {
                shape: vec![],
                animation: vec![],
                fighter: vec![],
                physics: vec![],
            },
        };
        drawing_system
            .load_texture(&texture_creator, RYU_SPRITE_PATH)
            .unwrap();

        // INIT FIGHTERS
        ryu::init(
            &mut scene.entities,
            &mut scene.fighter,
            (100.0, 400.0),
            Player::One,
            Direction::Right,
        );
        ryu::init(
            &mut scene.entities,
            &mut scene.fighter,
            (600.0, 400.0),
            Player::Two,
            Direction::Left,
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
            let keys = event_pump
                .keyboard_state()
                .pressed_scancodes()
                .filter_map(Keycode::from_scancode)
                .collect::<Vec<Keycode>>();

            FightSystem::run(
                &keys,
                self.fighter
                    .fighter
                    .iter_mut()
                    .zip(self.fighter.animation.iter_mut())
                    .zip(self.fighter.physics.iter_mut()),
            );
            AnimationSystem::run(
                self.fighter
                    .shape
                    .iter_mut()
                    .zip(self.fighter.animation.iter_mut()),
            );
            drawing_system.run(self.fighter.shape.iter())?;
        }

        Ok(())
    }
}
