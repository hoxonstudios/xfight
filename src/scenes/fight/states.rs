use crate::systems::tag::StateTag;

pub const STATE_STUN: StateTag = StateTag(0b1);
pub const STATE_GROUNDED: StateTag = StateTag(0b10);
pub const STATE_IMPACTED: StateTag = StateTag(0b100);
pub const STATE_DEAD: StateTag = StateTag(0b1000);
