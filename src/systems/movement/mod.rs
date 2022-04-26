pub mod system;

use super::{damage::Damage, drawing::Sprite, health::Shield, job::FightJob, tag::StateTag};

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
    pub transitions: &'static [MovementTransition],
    pub destroy_script: Option<usize>,
}
#[derive(Copy, Clone)]
pub struct MovementSprite {
    pub sprite: Sprite,
    pub velocity_change: Option<MovementVelocityChange>,
    pub damage_point: Option<Damage>,
    pub shield: Option<Shield>,
    pub spell: Option<MovementSpellScript>,
    pub frames: u8,
}
#[derive(Copy, Clone)]
pub struct MovementSpellScript {
    pub position: (f32, f32),
    pub script: usize,
}
#[derive(Copy, Clone)]
pub enum MovementVelocityChange {
    Horizontal(f32),
    HorizontalToAim(f32),
    BothToAim(f32, f32),
    Both(f32, f32),
}
#[derive(Copy, Clone)]
pub struct MovementTransition {
    pub conditions: &'static [MovementTransitionCondition],
    pub movement: usize,
    pub wait: bool,
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
