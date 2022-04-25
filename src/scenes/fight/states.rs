use crate::systems::tag::StateTag;

pub const STATE_STUN: StateTag = StateTag(0b1);
pub const STATE_GROUNDED: StateTag = StateTag(0b10);
pub const STATE_EXPLODE: StateTag = StateTag(0b100);
