extern crate sdl2;

mod scenes;
mod systems;

use scenes::fight::FightScene;
use sdl2::image::InitFlag;
use systems::{
    basic_attack::BasicAttackSystem, drawing::DrawingSystem, input::InputSystem,
    movement::MovementSystem, physics::PhysicsSystem, stand::StandSystem, walking::WalkingSystem,
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
        physics: PhysicsSystem::init(),
        drawing: DrawingSystem::init(&mut canvas, &texture_creator)?,
        input: InputSystem::init(&mut event_pump),
        movement: MovementSystem::init(),
        stand: StandSystem::init(),
        walking: WalkingSystem::init(),
        basic_attack: BasicAttackSystem::init(),
    };
    scene.init();
    scene.run()
}
