mod block;
mod stand;
mod stun;
mod walk;

mod light_kick;
mod light_punch;
mod strong_kick;
mod strong_punch;

mod crouch;
mod crouch_block;
mod crouch_light_kick;
mod crouch_light_punch;
mod crouch_strong_kick;
mod crouch_strong_punch;

mod jump;

use crate::systems::movement::Movement;

use self::{
    block::RYU_BLOCK,
    crouch::RYU_CROUCH,
    crouch_block::RYU_CROUCH_BLOCK,
    crouch_light_kick::RYU_CROUCH_LIGHT_KICK,
    crouch_light_punch::RYU_CROUCH_LIGHT_PUNCH,
    crouch_strong_kick::RYU_CROUCH_STRONG_KICK,
    crouch_strong_punch::RYU_CROUCH_STRONG_PUNCH,
    jump::{RYU_JUMP, RYU_JUMP_LEFT, RYU_JUMP_RIGHT},
    light_kick::RYU_LIGHT_KICK,
    light_punch::RYU_LIGHT_PUNCH,
    stand::RYU_STAND,
    strong_kick::RYU_STRONG_KICK,
    strong_punch::RYU_STRONG_PUNCH,
    stun::RYU_STUN,
    walk::{RYU_WALK_LEFT, RYU_WALK_RIGHT},
};

pub const RYU_TEXTURE_PATH: &'static str = "assets/ryu.png";
// CONSTANTS
pub const RYU_WALK_VELOCITY: f32 = 4.0;
// INDEXES
pub const RYU_STAND_INDEX: usize = 0;
pub const RYU_CROUCH_INDEX: usize = 1;
pub const RYU_WALK_LEFT_INDEX: usize = 2;
pub const RYU_WALK_RIGHT_INDEX: usize = 3;
pub const RYU_LIGHT_PUNCH_INDEX: usize = 4;
pub const RYU_STRONG_PUNCH_INDEX: usize = 5;
pub const RYU_LIGHT_KICK_INDEX: usize = 6;
pub const RYU_STRONG_KICK_INDEX: usize = 7;
pub const RYU_CROUCH_LIGHT_PUNCH_INDEX: usize = 8;
pub const RYU_CROUCH_STRONG_PUNCH_INDEX: usize = 9;
pub const RYU_CROUCH_LIGHT_KICK_INDEX: usize = 10;
pub const RYU_CROUCH_STRONG_KICK_INDEX: usize = 11;
pub const RYU_BLOCK_INDEX: usize = 12;
pub const RYU_CROUCH_BLOCK_INDEX: usize = 13;
pub const RYU_JUMP_INDEX: usize = 14;
pub const RYU_JUMP_LEFT_INDEX: usize = 15;
pub const RYU_JUMP_RIGHT_INDEX: usize = 16;
pub const RYU_STUN_INDEX: usize = 17;
// STAND
pub const RYU_MOVEMENTS: &'static [Movement] = &[
    RYU_STAND,
    RYU_CROUCH,
    RYU_WALK_LEFT,
    RYU_WALK_RIGHT,
    RYU_LIGHT_PUNCH,
    RYU_STRONG_PUNCH,
    RYU_LIGHT_KICK,
    RYU_STRONG_KICK,
    RYU_CROUCH_LIGHT_PUNCH,
    RYU_CROUCH_STRONG_PUNCH,
    RYU_CROUCH_LIGHT_KICK,
    RYU_CROUCH_STRONG_KICK,
    RYU_BLOCK,
    RYU_CROUCH_BLOCK,
    RYU_JUMP,
    RYU_JUMP_LEFT,
    RYU_JUMP_RIGHT,
    RYU_STUN,
];
