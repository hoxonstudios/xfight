mod fighters;
mod floor;

use sdl2::{event::Event, keyboard::Keycode};

use crate::systems::{
    aim::AimSystem,
    collision::CollisionSystem,
    damage::DamageSystem,
    drawing::DrawingSystem,
    health::{HealthSystem, Player},
    input::InputSystem,
    movement::system::MovementSystem,
    position::PositionSystem,
    tag::TagSystem,
    velocity::VelocitySystem,
};

use self::fighters::{CONTROLLER_ONE, CONTROLLER_TWO};

pub struct FightScene<'a> {
    pub entity: usize,
    pub tag: TagSystem,
    pub drawing: DrawingSystem<'a>,
    pub velocity: VelocitySystem,
    pub position: PositionSystem,
    pub collision: CollisionSystem,
    pub input: InputSystem<'a>,
    pub movement: MovementSystem,
    pub damage: DamageSystem,
    pub health: HealthSystem,
    pub aim: AimSystem,
}

impl<'a> FightScene<'a> {
    pub fn init(&mut self) {
        // INIT SHAPES
        self.init_floor();

        // INIT FIGHTERS
        self.init_ryu((100.0, 0.0), CONTROLLER_ONE, Player::One);
        self.init_ryu((600.0, 0.0), CONTROLLER_TWO, Player::Two);
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

            self.tag.update();
            self.input.update(&mut self.movement);
            self.aim.update(&self.position, &mut self.drawing);
            self.movement.update(
                &mut self.drawing,
                &mut self.velocity,
                &mut self.damage,
                &mut self.health,
                &self.aim,
                &self.tag,
            );
            self.velocity.update(&mut self.position);
            self.collision
                .update(&self.drawing, &mut self.position, &mut self.velocity);
            self.damage.update(
                &mut self.health,
                &mut self.tag,
                &self.collision,
                &self.position,
                &self.drawing,
            );
            self.health.update();
            self.position.update();
            self.drawing.update(&self.position)?;
        }

        Ok(())
    }
}
