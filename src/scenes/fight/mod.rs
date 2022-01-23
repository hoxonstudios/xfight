pub mod fighters;

use sdl2::{
    event::Event, keyboard::Keycode, render::TextureCreator, video::WindowContext, EventPump,
};
use xfight_ecs::{
    components::{AnimationComponent, Entity, PhysicsComponent, ShapeComponent},
    systems::{
        animation::AnimationSystem, drawing::DrawingSystem, input::InputSystem,
        physics::PhysicsSystem,
    },
};

use self::fighters::ryu::{self, RYU_SPRITE_PATH};

pub struct FightScene {
    keys: Vec<Keycode>,

    entities: Vec<Entity>,
    shapes: Vec<ShapeComponent>,
    physics: Vec<PhysicsComponent>,
    animations: Vec<AnimationComponent>,
}

impl FightScene {
    pub fn init<'a>(
        texture_creator: &'a TextureCreator<WindowContext>,
        drawing_system: &mut DrawingSystem<'a>,
    ) -> FightScene {
        let mut scene = FightScene {
            keys: vec![],

            entities: vec![],
            shapes: vec![],
            physics: vec![],
            animations: vec![],
        };
        drawing_system
            .load_texture(&texture_creator, RYU_SPRITE_PATH)
            .unwrap();

        ryu::init(
            &mut scene.entities,
            &mut scene.shapes,
            &mut scene.physics,
            &mut scene.animations,
            (100.0, 400.0),
            false,
        );
        ryu::init(
            &mut scene.entities,
            &mut scene.shapes,
            &mut scene.physics,
            &mut scene.animations,
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

            self.keys = InputSystem::run(event_pump);
            PhysicsSystem::run(&self.entities, &mut self.physics, &mut self.shapes);
            AnimationSystem::run(&self.entities, &mut self.animations, &mut self.shapes);
            drawing_system.run(&self.shapes)?;
        }

        Ok(())
    }
}
