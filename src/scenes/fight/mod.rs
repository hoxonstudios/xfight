mod fighters;
mod floor;

use sdl2::{
    event::Event, keyboard::Keycode, render::TextureCreator, video::WindowContext, EventPump,
};

use crate::systems::{drawing::DrawingSystem, physics::PhysicsSystem, walking::WalkingSystem};

pub struct FightScene<'a> {
    pub entity: usize,
    pub physics: PhysicsSystem,
    pub drawing: DrawingSystem<'a>,
    pub walking: WalkingSystem,
}

impl<'a> FightScene<'a> {
    pub fn init(&mut self) {
        // INIT SHAPES
        self.init_floor();

        // INIT FIGHTERS
        self.init_ryu((100.0, 350.0), false);
        self.init_ryu((600.0, 350.0), true);
    }

    pub fn run(&mut self, event_pump: &mut EventPump) -> Result<(), String> {
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

            self.walking.update(&mut self.physics, &mut self.drawing);
            self.physics.update();
            self.drawing.update(&self.physics)?;
        }

        Ok(())
    }
}
