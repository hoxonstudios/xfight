mod fighters;
mod floor;

use sdl2::{event::Event, keyboard::Keycode};

use crate::systems::{
    drawing::DrawingSystem,
    input::{Controller, InputSystem},
    movement::MovementSystem,
    physics::PhysicsSystem,
    punch::PunchSystem,
    stand::StandSystem,
    walking::WalkingSystem,
};

pub struct FightScene<'a> {
    pub entity: usize,
    pub physics: PhysicsSystem,
    pub drawing: DrawingSystem<'a>,
    pub input: InputSystem<'a>,
    pub movement: MovementSystem,
    pub stand: StandSystem,
    pub walking: WalkingSystem,
    pub punch: PunchSystem,
}

impl<'a> FightScene<'a> {
    pub fn init(&mut self) {
        // INIT SHAPES
        self.init_floor();

        // INIT FIGHTERS
        self.init_ryu((100.0, 350.0), Controller::One);
        self.init_ryu((600.0, 350.0), Controller::Two);
    }

    pub fn run(&mut self) -> Result<(), String> {
        'running: loop {
            for event in self.input.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }

            self.input.update(&mut self.movement);
            self.movement.update(&self.physics, &mut self.drawing);
            self.punch.update(&mut self.drawing, &mut self.movement);
            self.stand
                .update(&mut self.physics, &mut self.drawing, &self.movement);
            self.walking
                .update(&mut self.physics, &mut self.drawing, &self.movement);
            self.physics.update();
            self.drawing.update(&self.physics)?;
        }

        Ok(())
    }
}
