extern crate sdl2;

mod scenes;
mod systems;

use scenes::fight::FightScene;
use sdl2::image::InitFlag;
use systems::{
    aim::AimSystem, collision::CollisionSystem, damage::DamageSystem, drawing::DrawingSystem,
    health::HealthSystem, input::InputSystem, movement::system::MovementSystem,
    position::PositionSystem, tag::TagSystem, velocity::VelocitySystem,
};

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    sdl2::image::init(InitFlag::PNG)?;
    let window = video_subsystem
        .window("X-Fight", 800, 600)
        .fullscreen()
        .position_centered()
        .build()
        .expect("Failed to create window");
    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Failed to create canvas");
    let texture_creator = canvas.texture_creator();
    // JOYSTICKS
    let joystick_subsystem = sdl_context.joystick()?;
    let num_joysticks = joystick_subsystem
        .num_joysticks()
        .expect("No joysticks available");
    println!("> {} Joysticks", num_joysticks);
    let joy1 = joystick_subsystem
        .open(0)
        .expect("Failed to open controller");
    let joy2 = joystick_subsystem
        .open(1)
        .expect("Failed to open controller");
    let mut event_pump = sdl_context.event_pump()?;

    // SCENE
    let mut scene = FightScene {
        entity: 0,
        jobs: vec![],
        tag: TagSystem::init(),
        drawing: DrawingSystem::init(&mut canvas, &texture_creator)?,
        position: PositionSystem::init(),
        velocity: VelocitySystem::init(),
        collision: CollisionSystem::init(),
        input: InputSystem::init(&mut event_pump, Some((&joy1, &joy2))),
        movement: MovementSystem::init(),
        damage: DamageSystem::init(),
        health: HealthSystem::init(),
        aim: AimSystem::init(),
    };
    scene.init();
    scene.run()
}
