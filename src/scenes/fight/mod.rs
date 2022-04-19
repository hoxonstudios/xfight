mod fighters;
mod floor;

use sdl2::{event::Event, keyboard::Keycode};

use crate::systems::{
    aim::AimSystem,
    collision::CollisionSystem,
    damage::DamageSystem,
    drawing::DrawingSystem,
    ground::GroundSystem,
    health::{HealthSystem, Player},
    input::{Controller, InputSystem},
    movement::MovementSystem,
    position::PositionSystem,
    velocity::VelocitySystem,
};

pub struct FightScene<'a> {
    pub entity: usize,
    pub drawing: DrawingSystem<'a>,
    pub velocity: VelocitySystem,
    pub position: PositionSystem,
    pub collision: CollisionSystem,
    pub input: InputSystem<'a>,
    pub movement: MovementSystem,
    pub damage: DamageSystem,
    pub health: HealthSystem,
    pub aim: AimSystem,
    pub ground: GroundSystem,
}

impl<'a> FightScene<'a> {
    pub fn init(&mut self) {
        // INIT SHAPES
        self.init_floor();

        // INIT FIGHTERS
        self.init_ryu((100.0, 0.0), Controller::One, Player::One);
        self.init_ryu((600.0, 0.0), Controller::Two, Player::Two);
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
            self.aim.update(&self.position, &mut self.drawing);
            self.ground.update(&mut self.movement, &self.collision);
            self.damage.update(
                &mut self.health,
                &self.collision,
                &self.position,
                &self.drawing,
                &mut self.movement,
            );
            self.movement.update(
                &mut self.velocity,
                &mut self.drawing,
                &mut self.damage,
                &mut self.health,
            );
            self.velocity.update(&mut self.position);
            self.collision
                .update(&self.drawing, &mut self.position, &mut self.velocity);
            self.health.update();
            self.position.update();
            self.drawing.update(&self.position)?;
        }

        Ok(())
    }
}
