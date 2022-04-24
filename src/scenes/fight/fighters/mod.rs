pub mod ryu;

use sdl2::keyboard::Keycode;

use crate::systems::{
    input::{Controller, ControllerKey},
    movement::MovementAction,
    tag::StateTag,
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
pub const STATE_STUN: StateTag = StateTag(0b1);
// CONTROLLERS
pub const CONTROLLER_ONE: Controller = Controller {
    keys: &[
        ControllerKey {
            code: Keycode::S,
            action: ACTION_CROUCH,
        },
        ControllerKey {
            code: Keycode::W,
            action: ACTION_JUMP,
        },
        ControllerKey {
            code: Keycode::A,
            action: ACTION_LEFT,
        },
        ControllerKey {
            code: Keycode::D,
            action: ACTION_RIGHT,
        },
        ControllerKey {
            code: Keycode::H,
            action: ACTION_LIGHT_PUNCH,
        },
        ControllerKey {
            code: Keycode::Y,
            action: ACTION_STRONG_PUNCH,
        },
        ControllerKey {
            code: Keycode::K,
            action: ACTION_LIGHT_KICK,
        },
        ControllerKey {
            code: Keycode::I,
            action: ACTION_STRONG_KICK,
        },
        ControllerKey {
            code: Keycode::U,
            action: ACTION_BLOCK,
        },
    ],
};

pub const CONTROLLER_TWO: Controller = Controller {
    keys: &[
        ControllerKey {
            code: Keycode::Down,
            action: ACTION_CROUCH,
        },
        ControllerKey {
            code: Keycode::Up,
            action: ACTION_JUMP,
        },
        ControllerKey {
            code: Keycode::Left,
            action: ACTION_LEFT,
        },
        ControllerKey {
            code: Keycode::Right,
            action: ACTION_RIGHT,
        },
        ControllerKey {
            code: Keycode::Kp1,
            action: ACTION_LIGHT_PUNCH,
        },
        ControllerKey {
            code: Keycode::Kp4,
            action: ACTION_STRONG_PUNCH,
        },
        ControllerKey {
            code: Keycode::Kp3,
            action: ACTION_LIGHT_KICK,
        },
        ControllerKey {
            code: Keycode::Kp6,
            action: ACTION_STRONG_KICK,
        },
        ControllerKey {
            code: Keycode::Kp5,
            action: ACTION_BLOCK,
        },
    ],
};
