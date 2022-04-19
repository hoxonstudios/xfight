extern crate sdl2;

mod scenes;
mod systems;

use scenes::fight::FightScene;
use sdl2::image::InitFlag;
use systems::{
    aim::AimSystem, collision::CollisionSystem, damage::DamageSystem, drawing::DrawingSystem,
    ground::GroundSystem, health::HealthSystem, input::InputSystem, movement::MovementSystem,
    position::PositionSystem, velocity::VelocitySystem,
};

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    sdl2::image::init(InitFlag::PNG)?;
    let window = video_subsystem
        .window("X-Fight", 800, 600)
        .position_centered()
        .build()
        .expect("Failed to create window");
    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Failed to create canvas");
    let texture_creator = canvas.texture_creator();
    let mut event_pump = sdl_context.event_pump()?;

    // SCENE
    let mut scene = FightScene {
        entity: 0,
        drawing: DrawingSystem::init(&mut canvas, &texture_creator)?,
        position: PositionSystem::init(),
        velocity: VelocitySystem::init(),
        collision: CollisionSystem::init(),
        input: InputSystem::init(&mut event_pump),
        movement: MovementSystem::init(),
        damage: DamageSystem::init(),
        health: HealthSystem::init(),
        aim: AimSystem::init(),
        ground: GroundSystem::init(),
    };
    scene.init();
    scene.run()
}
