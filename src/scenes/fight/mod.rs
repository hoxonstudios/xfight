mod fighters;
mod floor;
pub mod jobs;
mod kinds;
mod states;

use sdl2::{event::Event, joystick::Joystick, keyboard::Keycode};

use crate::systems::{
    aim::AimSystem,
    collision::CollisionSystem,
    damage::DamageSystem,
    drawing::DrawingSystem,
    health::{HealthSystem, Player},
    input::InputSystem,
    job::{FightJob, FightJobParameters},
    movement::system::MovementSystem,
    position::PositionSystem,
    tag::TagSystem,
    velocity::VelocitySystem,
};

use self::{
    fighters::{get_joystick, KEYBOARD_ONE, KEYBOARD_TWO},
    jobs::{FIGHT_JOBS, JOB_SPAWN_FLOOR_INDEX, JOB_SPAWN_RYU_INDEX},
};

pub struct FightScene<'a> {
    pub entity: usize,
    pub jobs: Vec<FightJob>,
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
        self.jobs.push(FightJob {
            script: JOB_SPAWN_FLOOR_INDEX,
            parameters: FightJobParameters::None,
        });

        // INIT FIGHTERS
        self.jobs.push(FightJob {
            script: JOB_SPAWN_RYU_INDEX,
            parameters: FightJobParameters::SpawnFighter {
                position: (100.0, 0.0),
                controller: get_joystick(0),
                player: Player::One,
            },
        });
        self.jobs.push(FightJob {
            script: JOB_SPAWN_RYU_INDEX,
            parameters: FightJobParameters::SpawnFighter {
                position: (600.0, 0.0),
                controller: get_joystick(1),
                player: Player::Two,
            },
        });
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

            self.run_jobs();
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
                &self.position,
                &mut self.jobs,
            );
            self.velocity.update(&mut self.position);
            self.collision.update(
                &self.drawing,
                &mut self.position,
                &mut self.velocity,
                &mut self.tag,
            );
            self.damage.update(
                &mut self.health,
                &mut self.tag,
                &self.collision,
                &self.position,
                &self.drawing,
            );
            self.health.update(&mut self.tag);
            self.position.update();
            self.drawing.update(&self.position)?;
        }

        Ok(())
    }

    fn run_jobs(&mut self) {
        for job in &self.jobs.clone() {
            let script = FIGHT_JOBS[job.script];
            script(self, job.parameters);
        }
        self.jobs.clear();
    }
}
