mod fighters;
mod floor;

use sdl2::{event::Event, keyboard::Keycode};

use crate::systems::{
    basic_attack::BasicAttackSystem,
    collision::CollisionSystem,
    damage::DamageSystem,
    drawing::DrawingSystem,
    health::{HealthSystem, Player},
    input::{Controller, InputSystem},
    movement::MovementSystem,
    position::PositionSystem,
    shape::ShapeSystem,
    stand::StandSystem,
    stun::StunSystem,
    velocity::VelocitySystem,
    walking::WalkingSystem,
};

pub struct FightScene<'a> {
    pub entity: usize,
    pub drawing: DrawingSystem<'a>,
    pub shape: ShapeSystem,
    pub velocity: VelocitySystem,
    pub position: PositionSystem,
    pub collision: CollisionSystem,
    pub input: InputSystem<'a>,
    pub movement: MovementSystem,
    pub stand: StandSystem,
    pub walking: WalkingSystem,
    pub basic_attack: BasicAttackSystem,
    pub damage: DamageSystem,
    pub health: HealthSystem,
    pub stun: StunSystem,
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
            self.stun.update(
                &self.health,
                &mut self.movement,
                &mut self.shape,
                &mut self.velocity,
            );
            self.movement
                .update(&mut self.shape, &self.position, &self.collision);
            self.basic_attack.update(
                &mut self.movement,
                &mut self.velocity,
                &mut self.damage,
                &mut self.shape,
            );
            self.stand
                .update(&self.movement, &mut self.shape, &mut self.velocity);
            self.walking
                .update(&mut self.shape, &mut self.velocity, &mut self.movement);
            self.shape.update();
            self.velocity.update(&mut self.position);
            self.collision.update(&self.shape, &mut self.position);
            self.damage.update(
                &mut self.health,
                &self.collision,
                &self.position,
                &self.shape,
            );
            self.health.update();
            self.position.update();
            self.drawing.update(&self.position, &self.shape)?;
        }

        Ok(())
    }
}
