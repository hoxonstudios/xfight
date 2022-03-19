mod fighters;
mod floor;

use sdl2::{event::Event, keyboard::Keycode};

use crate::systems::{
    basic_attack::BasicAttackSystem,
    damage::{DamageSystem, Player},
    drawing::DrawingSystem,
    input::{Controller, InputSystem},
    movement::MovementSystem,
    physics::PhysicsSystem,
    stand::StandSystem,
    stun::StunSystem,
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
    pub basic_attack: BasicAttackSystem,
    pub damage: DamageSystem,
    pub stun: StunSystem,
}

impl<'a> FightScene<'a> {
    pub fn init(&mut self) {
        // INIT SHAPES
        self.init_floor();

        // INIT FIGHTERS
        self.init_ryu((100.0, 350.0), Controller::One, Player::One);
        self.init_ryu((600.0, 350.0), Controller::Two, Player::Two);
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
            self.stun
                .update(&self.damage, &mut self.movement, &mut self.physics);
            self.movement.update(&mut self.physics);
            self.basic_attack
                .update(&mut self.physics, &mut self.movement, &mut self.damage);
            self.stand.update(&mut self.physics, &self.movement);
            self.walking.update(&mut self.physics, &self.movement);
            self.physics.update();
            self.damage.update(&self.physics);
            self.drawing.update(&self.physics)?;
        }

        Ok(())
    }
}
