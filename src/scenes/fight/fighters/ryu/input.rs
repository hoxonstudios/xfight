use sdl2::keyboard::Keycode;

use super::movements::{
    RYU_ACTION_JUMP_STRAIGHT, RYU_ACTION_MOVE_BACKWARD, RYU_ACTION_MOVE_FORWARD, RYU_ACTION_NONE,
};

pub fn ryu_input(keys: &Vec<Keycode>) -> u32 {
    let right_key = keys.iter().any(|k| matches!(k, Keycode::Right));
    let left_key = keys.iter().any(|k| matches!(k, Keycode::Left));
    let up_key = keys.iter().any(|k| matches!(k, Keycode::Up));

    if right_key {
        RYU_ACTION_MOVE_FORWARD
    } else if left_key {
        RYU_ACTION_MOVE_BACKWARD
    } else if up_key {
        RYU_ACTION_JUMP_STRAIGHT
    } else {
        RYU_ACTION_NONE
    }
}
