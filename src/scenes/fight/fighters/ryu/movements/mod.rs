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
mod crouch_stun;

mod jump;
mod jump_light_kick;
mod jump_light_punch;
mod jump_strong_kick;
mod jump_strong_punch;
mod jump_stun;

mod special_adoken;

use crate::systems::movement::Movement;

use self::{
    block::RYU_BLOCK,
    crouch::RYU_CROUCH,
    crouch_block::RYU_CROUCH_BLOCK,
    crouch_light_kick::RYU_CROUCH_LIGHT_KICK,
    crouch_light_punch::RYU_CROUCH_LIGHT_PUNCH,
    crouch_strong_kick::RYU_CROUCH_STRONG_KICK,
    crouch_strong_punch::RYU_CROUCH_STRONG_PUNCH,
    crouch_stun::RYU_CROUCH_STUN,
    jump::{RYU_JUMP, RYU_JUMP_LEFT, RYU_JUMP_RIGHT},
    jump_light_kick::RYU_JUMP_LIGHT_KICK,
    jump_light_punch::RYU_JUMP_LIGHT_PUNCH,
    jump_strong_kick::RYU_JUMP_STRONG_KICK,
    jump_strong_punch::RYU_JUMP_STRONG_PUNCH,
    jump_stun::RYU_JUMP_STUN,
    light_kick::RYU_LIGHT_KICK,
    light_punch::RYU_LIGHT_PUNCH,
    special_adoken::RYU_SPECIAL_ADOKEN,
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
pub const RYU_JUMP_STRONG_PUNCH_INDEX: usize = 18;
pub const RYU_JUMP_LIGHT_PUNCH_INDEX: usize = 19;
pub const RYU_JUMP_STRONG_KICK_INDEX: usize = 20;
pub const RYU_JUMP_LIGHT_KICK_INDEX: usize = 21;
pub const RYU_JUMP_STUN_INDEX: usize = 22;
pub const RYU_CROUCH_STUN_INDEX: usize = 23;
pub const RYU_SPECIAL_ADOKEN_INDEX: usize = 24;
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
    RYU_JUMP_STRONG_PUNCH,
    RYU_JUMP_LIGHT_PUNCH,
    RYU_JUMP_STRONG_KICK,
    RYU_JUMP_LIGHT_KICK,
    RYU_JUMP_STUN,
    RYU_CROUCH_STUN,
    RYU_SPECIAL_ADOKEN,
];
