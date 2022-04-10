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
                    ControllerMap {
                        down: true,
                        light_punch: true,
                        ..
                    } => Some(MovementAction::CrunchLightPunch),
                    ControllerMap {
                        down: true,
                        strong_punch: true,
                        ..
                    } => Some(MovementAction::CrunchStrongPunch),
                    ControllerMap {
                        down: true,
                        light_kick: true,
                        ..
                    } => Some(MovementAction::CrunchLightKick),
                    ControllerMap {
                        down: true,
                        strong_kick: true,
                        ..
                    } => Some(MovementAction::CrunchStrongKick),
                    ControllerMap { down: true, .. } => Some(MovementAction::Crunch),
                    ControllerMap {
                        strong_punch: true, ..
                    } => Some(MovementAction::StrongPunch),
                    ControllerMap {
                        light_punch: true, ..
                    } => Some(MovementAction::LightPunch),
                    ControllerMap {
                        strong_kick: true, ..
                    } => Some(MovementAction::StrongKick),
                    ControllerMap {
                        light_kick: true, ..
                    } => Some(MovementAction::LightKick),
                    ControllerMap { up: true, .. } => Some(MovementAction::JumpStraight),
                    ControllerMap { left: true, .. } => Some(MovementAction::WalkLeft),
                    ControllerMap { right: true, .. } => Some(MovementAction::WalkRight),
                    _ => None,
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
                    up: keys.contains(&Keycode::W),
                    down: keys.contains(&Keycode::S),
                    left: keys.contains(&Keycode::A),
                    right: keys.contains(&Keycode::D),
                    strong_punch: keys.contains(&Keycode::T),
                    light_punch: keys.contains(&Keycode::G),
                    strong_kick: keys.contains(&Keycode::U),
                    light_kick: keys.contains(&Keycode::J),
                },
                ControllerMap {
                    up: keys.contains(&Keycode::Up),
                    down: keys.contains(&Keycode::Down),
                    left: keys.contains(&Keycode::Left),
                    right: keys.contains(&Keycode::Right),
                    strong_punch: keys.contains(&Keycode::Kp4),
                    light_punch: keys.contains(&Keycode::Kp1),
                    strong_kick: keys.contains(&Keycode::Kp6),
                    light_kick: keys.contains(&Keycode::Kp3),
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
    down: bool,
    up: bool,
    light_punch: bool,
    strong_punch: bool,
    light_kick: bool,
    strong_kick: bool,
}
