use sdl2::{
    controller::{Axis, Button},
    event::Event,
    joystick::Joystick,
    keyboard::Keycode,
    EventPump,
};

use super::{
    helpers::component_store::ComponentStore,
    movement::{system::MovementSystem, MovementAction},
};

#[derive(Copy, Clone)]
pub struct InputComponent {
    pub entity: usize,
    pub controller: Controller,
}

#[derive(Copy, Clone)]
pub enum Controller {
    Keyboard(&'static [KeyboardAction]),
    Joystick(u32, &'static [JoystickAction]),
}
#[derive(Copy, Clone)]
pub struct JoystickAction {
    pub button: JoystickButton,
    pub action: MovementAction,
}
#[derive(Copy, Clone, PartialEq)]
pub enum JoystickButton {
    Up,
    Down,
    Left,
    Right,
    Square,
    Triangle,
    Circle,
    Cross,
    L1,
    R1,
}
#[derive(Copy, Clone)]
pub struct KeyboardAction {
    pub code: Keycode,
    pub action: MovementAction,
}

pub struct InputSystem<'a> {
    pub event_pump: &'a mut EventPump,
    pub joysticks: Option<(&'a Joystick, &'a Joystick)>,
    pub store: ComponentStore<InputComponent>,
}
impl<'a> InputSystem<'a> {
    pub fn init(
        event_pump: &'a mut EventPump,
        joysticks: Option<(&'a Joystick, &'a Joystick)>,
    ) -> InputSystem<'a> {
        InputSystem {
            event_pump,
            joysticks,
            store: ComponentStore::<InputComponent>::init(),
        }
    }
    pub fn update(&self, movement_system: &mut MovementSystem) {
        let pressed_keys = self.get_pressed_keys();

        for input in self.store.data() {
            let entity = input.entity;

            if let Some(movement) = movement_system.store.get_mut_component(entity) {
                let mut action = match movement.action {
                    None => 0,
                    Some(action) => action.0,
                };

                match input.controller {
                    Controller::Keyboard(keys) => {
                        for key in keys {
                            if pressed_keys.contains(&key.code) {
                                action |= key.action.0;
                            }
                        }
                    }
                    Controller::Joystick(id, keys) => {
                        let pressed_buttons = self.get_pressed_buttons(id);

                        for key in keys {
                            if pressed_buttons.contains(&key.button) {
                                action |= key.action.0;
                            }
                        }
                    }
                }

                let none_action = action == 0;
                movement.action = if none_action {
                    None
                } else {
                    Some(MovementAction(action))
                };
            }
        }
    }

    fn get_pressed_keys(&self) -> Vec<Keycode> {
        self.event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect()
    }
    fn get_pressed_buttons(&self, id: u32) -> Vec<JoystickButton> {
        if let Some((joy1, joy2)) = self.joysticks {
            let mut buttons: Vec<JoystickButton> = vec![];
            let joystick = if id == 0 { joy1 } else { joy2 };

            for i in 0..JOYSTICK_MAPPING.len() {
                if joystick.button(i as u32).unwrap() {
                    buttons.push(JOYSTICK_MAPPING[i]);
                }
            }
            let horizontal = joystick.axis(0).unwrap();
            let vertical = joystick.axis(1).unwrap();

            if horizontal < 0 {
                buttons.push(JoystickButton::Left)
            } else if horizontal > 0 {
                buttons.push(JoystickButton::Right)
            }

            if vertical < 0 {
                buttons.push(JoystickButton::Up);
            } else if vertical > 0 {
                buttons.push(JoystickButton::Down);
            }

            return buttons;
        } else {
            return vec![];
        }
    }
}

const JOYSTICK_MAPPING: &'static [JoystickButton] = &[
    JoystickButton::Triangle,
    JoystickButton::Circle,
    JoystickButton::Cross,
    JoystickButton::Square,
    JoystickButton::L1,
    JoystickButton::R1,
];
