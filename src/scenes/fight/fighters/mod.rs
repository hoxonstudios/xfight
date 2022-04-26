pub mod ryu;

use sdl2::keyboard::Keycode;

use crate::systems::{
    input::{Controller, JoystickAction, JoystickButton, KeyboardAction},
    movement::MovementAction,
};

// ACTIONS
pub const ACTION_CROUCH: MovementAction = MovementAction(0b1);
pub const ACTION_LEFT: MovementAction = MovementAction(0b10);
pub const ACTION_RIGHT: MovementAction = MovementAction(0b100);
pub const ACTION_LIGHT_PUNCH: MovementAction = MovementAction(0b1000);
pub const ACTION_STRONG_PUNCH: MovementAction = MovementAction(0b10000);
pub const ACTION_LIGHT_KICK: MovementAction = MovementAction(0b100000);
pub const ACTION_STRONG_KICK: MovementAction = MovementAction(0b1000000);
pub const ACTION_BLOCK: MovementAction = MovementAction(0b10000000);
pub const ACTION_JUMP: MovementAction = MovementAction(0b100000000);
pub const ACTION_SPECIAL: MovementAction = MovementAction(0b1000000000);
// COMBINATIONS
pub const ACTION_CROUCH_LIGHT_PUNCH: MovementAction =
    MovementAction(ACTION_CROUCH.0 | ACTION_LIGHT_PUNCH.0);
pub const ACTION_CROUCH_STRONG_PUNCH: MovementAction =
    MovementAction(ACTION_CROUCH.0 | ACTION_STRONG_PUNCH.0);
pub const ACTION_CROUCH_LIGHT_KICK: MovementAction =
    MovementAction(ACTION_CROUCH.0 | ACTION_LIGHT_KICK.0);
pub const ACTION_CROUCH_STRONG_KICK: MovementAction =
    MovementAction(ACTION_CROUCH.0 | ACTION_STRONG_KICK.0);
pub const ACTION_CROUCH_BLOCK: MovementAction = MovementAction(ACTION_CROUCH.0 | ACTION_BLOCK.0);
pub const ACTION_JUMP_LEFT: MovementAction = MovementAction(ACTION_JUMP.0 | ACTION_LEFT.0);
pub const ACTION_JUMP_RIGHT: MovementAction = MovementAction(ACTION_JUMP.0 | ACTION_RIGHT.0);
// STATES
// CONTROLLERS
pub const KEYBOARD_ONE: Controller = Controller::Keyboard(&[
    KeyboardAction {
        code: Keycode::S,
        action: ACTION_CROUCH,
    },
    KeyboardAction {
        code: Keycode::W,
        action: ACTION_JUMP,
    },
    KeyboardAction {
        code: Keycode::A,
        action: ACTION_LEFT,
    },
    KeyboardAction {
        code: Keycode::D,
        action: ACTION_RIGHT,
    },
    KeyboardAction {
        code: Keycode::H,
        action: ACTION_LIGHT_PUNCH,
    },
    KeyboardAction {
        code: Keycode::Y,
        action: ACTION_STRONG_PUNCH,
    },
    KeyboardAction {
        code: Keycode::K,
        action: ACTION_LIGHT_KICK,
    },
    KeyboardAction {
        code: Keycode::I,
        action: ACTION_STRONG_KICK,
    },
    KeyboardAction {
        code: Keycode::U,
        action: ACTION_BLOCK,
    },
    KeyboardAction {
        code: Keycode::J,
        action: ACTION_SPECIAL,
    },
]);

pub const KEYBOARD_TWO: Controller = Controller::Keyboard(&[
    KeyboardAction {
        code: Keycode::Down,
        action: ACTION_CROUCH,
    },
    KeyboardAction {
        code: Keycode::Up,
        action: ACTION_JUMP,
    },
    KeyboardAction {
        code: Keycode::Left,
        action: ACTION_LEFT,
    },
    KeyboardAction {
        code: Keycode::Right,
        action: ACTION_RIGHT,
    },
    KeyboardAction {
        code: Keycode::Kp1,
        action: ACTION_LIGHT_PUNCH,
    },
    KeyboardAction {
        code: Keycode::Kp4,
        action: ACTION_STRONG_PUNCH,
    },
    KeyboardAction {
        code: Keycode::Kp3,
        action: ACTION_LIGHT_KICK,
    },
    KeyboardAction {
        code: Keycode::Kp6,
        action: ACTION_STRONG_KICK,
    },
    KeyboardAction {
        code: Keycode::Kp5,
        action: ACTION_BLOCK,
    },
    KeyboardAction {
        code: Keycode::Kp2,
        action: ACTION_SPECIAL,
    },
]);

pub fn get_joystick(id: u32) -> Controller {
    Controller::Joystick(
        id,
        &[
            JoystickAction {
                button: JoystickButton::Up,
                action: ACTION_JUMP,
            },
            JoystickAction {
                button: JoystickButton::Down,
                action: ACTION_CROUCH,
            },
            JoystickAction {
                button: JoystickButton::Left,
                action: ACTION_LEFT,
            },
            JoystickAction {
                button: JoystickButton::Right,
                action: ACTION_RIGHT,
            },
            JoystickAction {
                button: JoystickButton::Cross,
                action: ACTION_LIGHT_PUNCH,
            },
            JoystickAction {
                button: JoystickButton::Square,
                action: ACTION_STRONG_PUNCH,
            },
            JoystickAction {
                button: JoystickButton::Circle,
                action: ACTION_LIGHT_KICK,
            },
            JoystickAction {
                button: JoystickButton::Triangle,
                action: ACTION_STRONG_KICK,
            },
            JoystickAction {
                button: JoystickButton::R2,
                action: ACTION_BLOCK,
            },
            JoystickAction {
                button: JoystickButton::L2,
                action: ACTION_SPECIAL,
            },
        ],
    )
}
