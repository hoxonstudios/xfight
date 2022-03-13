use sdl2::{keyboard::Keycode, EventPump};

use super::{
    helpers::ComponentStore,
    movement::{MovementAction, MovementSystem},
};

#[derive(Copy, Clone)]
pub struct InputComponent {
    pub entity: usize,
    pub controller: Controller,
}

#[derive(Copy, Clone)]
pub enum Controller {
    One,
    Two,
}

pub struct InputSystem<'a> {
    pub event_pump: &'a mut EventPump,
    pub store: ComponentStore<InputComponent>,
}
impl<'a> InputSystem<'a> {
    pub fn init(event_pump: &'a mut EventPump) -> InputSystem {
        InputSystem {
            event_pump,
            store: ComponentStore::<InputComponent>::init(),
        }
    }
    pub fn update(&self, movement_system: &mut MovementSystem) {
        let pressed_keys = self.get_pressed_keys();

        for input in self.store.data() {
            let entity = input.entity;
            if let Some(movement) = movement_system.store.get_mut_component(entity) {
                let controller = match input.controller {
                    Controller::One => pressed_keys.controllers[0],
                    Controller::Two => pressed_keys.controllers[1],
                };
                movement.action = match controller {
                    ControllerMap { punch: true, .. } => MovementAction::Punch,
                    ControllerMap { left: true, .. } => MovementAction::WalkLeft,
                    ControllerMap { right: true, .. } => MovementAction::WalkRight,
                    _ => MovementAction::None,
                };
            }
        }
    }

    fn get_pressed_keys(&self) -> PressedKeys {
        let keys: Vec<Keycode> = self
            .event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        PressedKeys {
            controllers: [
                ControllerMap {
                    left: keys.contains(&Keycode::A),
                    right: keys.contains(&Keycode::D),
                    punch: keys.contains(&Keycode::T),
                },
                ControllerMap {
                    left: keys.contains(&Keycode::Left),
                    right: keys.contains(&Keycode::Right),
                    punch: keys.contains(&Keycode::Kp4),
                },
            ],
        }
    }
}

#[derive(Copy, Clone)]
struct PressedKeys {
    controllers: [ControllerMap; 2],
}
#[derive(Copy, Clone)]
struct ControllerMap {
    left: bool,
    right: bool,
    punch: bool,
}
