pub mod fighters;

use sdl2::{
    event::Event, keyboard::Keycode, render::TextureCreator, video::WindowContext, EventPump,
};
use xfight_ecs::{
    components::{
        AnimationComponent, Entity, InputComponent, MovementComponent, PhysicsComponent,
        ShapeComponent,
    },
    systems::{
        animation::AnimationSystem, drawing::DrawingSystem, input::InputSystem,
        movement::MovementSystem, physics::PhysicsSystem,
    },
};

use self::fighters::ryu::{self, RYU_SPRITE_PATH};

pub struct FightScene {
    entities: Vec<Entity>,
    inputs: Vec<InputComponent>,
    shapes: Vec<ShapeComponent>,
    physics: Vec<PhysicsComponent>,
    animations: Vec<AnimationComponent>,
    movements: Vec<MovementComponent>,
}

impl FightScene {
    pub fn init<'a>(
        texture_creator: &'a TextureCreator<WindowContext>,
        drawing_system: &mut DrawingSystem<'a>,
    ) -> FightScene {
        let mut scene = FightScene {
            entities: vec![],
            inputs: vec![],
            shapes: vec![],
            physics: vec![],
            animations: vec![],
            movements: vec![],
        };
        drawing_system
            .load_texture(&texture_creator, RYU_SPRITE_PATH)
            .unwrap();

        ryu::init(
            &mut scene.entities,
            &mut scene.shapes,
            &mut scene.physics,
            &mut scene.animations,
            &mut scene.movements,
            &mut scene.inputs,
            (100.0, 400.0),
            false,
        );
        ryu::init(
            &mut scene.entities,
            &mut scene.shapes,
            &mut scene.physics,
            &mut scene.animations,
            &mut scene.movements,
            &mut scene.inputs,
            (600.0, 400.0),
            true,
        );

        return scene;
    }

    pub fn run<'a>(
        &mut self,
        event_pump: &mut EventPump,
        drawing_system: &mut DrawingSystem<'a>,
    ) -> Result<(), String> {
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

            InputSystem::run(
                event_pump,
                &self.entities,
                &mut self.inputs,
                &mut self.movements,
            );
            MovementSystem::run(&mut self.movements);
            AnimationSystem::run(
                &self.entities,
                &mut self.animations,
                &mut self.shapes,
                &self.physics,
                &self.movements,
            );
            PhysicsSystem::run(
                &self.entities,
                &mut self.physics,
                &mut self.shapes,
                &self.movements,
            );
            drawing_system.run(&self.shapes)?;
        }

        Ok(())
    }
}
