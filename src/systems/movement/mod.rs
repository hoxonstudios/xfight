pub mod system;

use super::{damage::DamagePoint, drawing::Sprite, health::Shield, tag::StateTag};

#[derive(Copy, Clone)]
pub struct MovementComponent {
    pub entity: usize,
    pub action: Option<MovementAction>,
    pub movements: &'static [Movement],
    pub movement: usize,
    pub frame: Option<(usize, u8)>,
}
#[derive(Copy, Clone)]
pub struct Movement {
    pub sprites: &'static [MovementSprite],
    pub next: Option<usize>,
    pub transitions: &'static [MovementTransition],
}
#[derive(Copy, Clone)]
pub struct MovementSprite {
    pub sprite: Sprite,
    pub velocity_change: Option<MovementVelocityChange>,
    pub damage_point: Option<DamagePoint>,
    pub shield: Option<Shield>,
    pub frames: u8,
}
#[derive(Copy, Clone)]
pub enum MovementVelocityChange {
    Horizontal(f32),
    HorizontalToAim(f32),
    Both(f32, f32),
}
#[derive(Copy, Clone)]
pub struct MovementTransition {
    pub conditions: &'static [MovementTransitionCondition],
    pub movement: usize,
}

#[derive(Copy, Clone)]
pub enum MovementTransitionCondition {
    NoneAction,
    ActionActivated(MovementAction),
    ActionNotActivated(MovementAction),
    StateActive(StateTag),
    StateInactive(StateTag),
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MovementAction(pub u128);
