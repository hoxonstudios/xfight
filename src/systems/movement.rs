use super::{
    damage::{DamageAction, DamagePoint, DamageSystem},
    helpers::ComponentStore,
    shape::{ShapeAction, ShapeSystem, Sprite},
    velocity::VelocitySystem,
};

const STANDING_FRAMES: (usize, u8) = (4, 3);
const WALKING_FRAMES: (usize, u8) = (6, 2);
const STUNT_FRAMES: (usize, u8) = (1, 10);
const LIGHT_PUNCH_FRAMES: (usize, u8) = (3, 3);
const STRONG_PUNCH_FRAMES: (usize, u8) = (5, 3);
const LIGHT_KICK_FRAMES: (usize, u8) = (3, 3);
const STRONG_KICK_FRAMES: (usize, u8) = (3, 3);

const WALKING_VELOCITY: f32 = 2.0;

#[derive(Copy, Clone)]
pub struct MovementComponent {
    pub entity: usize,
    pub action: Option<MovementAction>,
    pub state: MovementState,
    pub sprites: MovementSprites,
}
#[derive(Copy, Clone)]
pub struct MovementSprites {
    pub standing: [Sprite; STANDING_FRAMES.0],
    pub walking: [Sprite; WALKING_FRAMES.0],
    pub stunt: Sprite,
    pub light_punch: [(Sprite, Option<DamagePoint>); LIGHT_PUNCH_FRAMES.0],
    pub strong_punch: [(Sprite, Option<DamagePoint>); STRONG_PUNCH_FRAMES.0],
    pub light_kick: [(Sprite, Option<DamagePoint>); LIGHT_KICK_FRAMES.0],
    pub strong_kick: [(Sprite, Option<DamagePoint>); STRONG_KICK_FRAMES.0],
}
#[derive(Copy, Clone, Debug)]
pub enum MovementAction {
    WalkRight,
    WalkLeft,
    LightPunch,
    StrongPunch,
    LightKick,
    StrongKick,
    JumpStraight,
    Land,
    Hit,
}
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum MovementState {
    Standing {
        frame: (usize, u8),
    },
    Stunt {
        frame: u8,
    },

    Walking {
        frame: (usize, u8),
        direction: WalkingDirection,
    },

    LightPunching {
        frame: (usize, u8),
    },
    StrongPunching {
        frame: (usize, u8),
    },
    LightKicking {
        frame: (usize, u8),
    },
    StrongKicking {
        frame: (usize, u8),
    },
    JumpingStraight {
        starting: bool,
    },
}
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum WalkingDirection {
    Left,
    Right,
}

pub struct MovementSystem {
    pub store: ComponentStore<MovementComponent>,
}
impl MovementSystem {
    pub fn init() -> MovementSystem {
        MovementSystem {
            store: ComponentStore::<MovementComponent>::init(),
        }
    }
    pub fn update<'a>(
        &mut self,
        velocity_system: &mut VelocitySystem,
        shape_system: &mut ShapeSystem,
        damage_system: &mut DamageSystem,
    ) {
        for movement in self.store.data_mut() {
            let entity = movement.entity;
            let transition = calculate_transition(movement.state, movement.action);

            if transition != movement.state {
                /*println!(
                    "{} | ({:?}, {:?}) => {:?}",
                    entity, movement.state, movement.action, transition
                );*/
            }

            movement.state = transition;
            movement.action = None;

            match movement.state {
                MovementState::Standing {
                    frame: (sprite, frame),
                } => {
                    if frame == 0 {
                        if sprite == 0 {
                            if let Some(velocity) = velocity_system.store.get_mut_component(entity)
                            {
                                velocity.velocity.0 = 0.0;
                            }
                        }
                        if let Some(shape) = shape_system.store.get_mut_component(entity) {
                            shape.action = ShapeAction::Update {
                                sprite: movement.sprites.standing[sprite],
                                flipped: shape.flipped,
                            };
                        }
                    }
                }
                MovementState::Stunt { frame } => {
                    if frame == 0 {
                        if let Some(shape) = shape_system.store.get_mut_component(entity) {
                            shape.action = ShapeAction::Update {
                                sprite: movement.sprites.stunt,
                                flipped: shape.flipped,
                            };
                        }
                        if let Some(velocity) = velocity_system.store.get_mut_component(entity) {
                            velocity.velocity.0 = 0.0;
                        }
                    }
                }
                MovementState::Walking {
                    frame: (sprite, frame),
                    direction,
                } => {
                    if frame == 0 {
                        if sprite == 0 {
                            if let Some(velocity) = velocity_system.store.get_mut_component(entity)
                            {
                                match direction {
                                    WalkingDirection::Left => {
                                        velocity.velocity.0 = -WALKING_VELOCITY;
                                    }
                                    WalkingDirection::Right => {
                                        velocity.velocity.0 = WALKING_VELOCITY;
                                    }
                                }
                            }
                        }
                        if let Some(shape) = shape_system.store.get_mut_component(entity) {
                            shape.action = ShapeAction::Update {
                                sprite: movement.sprites.walking[sprite],
                                flipped: shape.flipped,
                            };
                        }
                    }
                }
                MovementState::LightKicking {
                    frame: (sprite, frame),
                } => {
                    if frame == 0 {
                        if let Some(velocity) = velocity_system.store.get_mut_component(entity) {
                            velocity.velocity.0 = 0.0;
                        }
                        let sprite = movement.sprites.light_kick[sprite];
                        if let Some(shape) = shape_system.store.get_mut_component(entity) {
                            shape.action = ShapeAction::Update {
                                sprite: sprite.0,
                                flipped: shape.flipped,
                            };
                        }
                        if let Some(damage_point) = sprite.1 {
                            if let Some(damage) = damage_system.store.get_mut_component(entity) {
                                if let DamageAction::None = damage.action {
                                    damage.action = DamageAction::Inflict {
                                        point: damage_point,
                                    };
                                }
                            }
                        }
                    }
                }
                MovementState::LightPunching {
                    frame: (sprite, frame),
                } => {
                    if frame == 0 {
                        if let Some(velocity) = velocity_system.store.get_mut_component(entity) {
                            velocity.velocity.0 = 0.0;
                        }
                        let sprite = movement.sprites.light_punch[sprite];
                        if let Some(shape) = shape_system.store.get_mut_component(entity) {
                            shape.action = ShapeAction::Update {
                                sprite: sprite.0,
                                flipped: shape.flipped,
                            };
                        }
                        if let Some(damage_point) = sprite.1 {
                            if let Some(damage) = damage_system.store.get_mut_component(entity) {
                                if let DamageAction::None = damage.action {
                                    damage.action = DamageAction::Inflict {
                                        point: damage_point,
                                    };
                                }
                            }
                        }
                    }
                }
                MovementState::StrongKicking {
                    frame: (sprite, frame),
                } => {
                    if frame == 0 {
                        if let Some(velocity) = velocity_system.store.get_mut_component(entity) {
                            velocity.velocity.0 = 0.0;
                        }
                        let sprite = movement.sprites.strong_kick[sprite];
                        if let Some(shape) = shape_system.store.get_mut_component(entity) {
                            shape.action = ShapeAction::Update {
                                sprite: sprite.0,
                                flipped: shape.flipped,
                            };
                        }
                        if let Some(damage_point) = sprite.1 {
                            if let Some(damage) = damage_system.store.get_mut_component(entity) {
                                if let DamageAction::None = damage.action {
                                    damage.action = DamageAction::Inflict {
                                        point: damage_point,
                                    };
                                }
                            }
                        }
                    }
                }
                MovementState::StrongPunching {
                    frame: (sprite, frame),
                } => {
                    if frame == 0 {
                        if let Some(velocity) = velocity_system.store.get_mut_component(entity) {
                            velocity.velocity.0 = 0.0;
                        }
                        let sprite = movement.sprites.strong_punch[sprite];
                        if let Some(shape) = shape_system.store.get_mut_component(entity) {
                            shape.action = ShapeAction::Update {
                                sprite: sprite.0,
                                flipped: shape.flipped,
                            };
                        }
                        if let Some(damage_point) = sprite.1 {
                            if let Some(damage) = damage_system.store.get_mut_component(entity) {
                                if let DamageAction::None = damage.action {
                                    damage.action = DamageAction::Inflict {
                                        point: damage_point,
                                    };
                                }
                            }
                        }
                    }
                }
                MovementState::JumpingStraight { starting } => {
                    if starting {
                        if let Some(velocity) = velocity_system.store.get_mut_component(entity) {
                            velocity.velocity.1 = -10.0;
                        }
                    }
                }
            }
        }
    }
}

fn calculate_transition(state: MovementState, action: Option<MovementAction>) -> MovementState {
    match state {
        MovementState::Standing { frame } => match action {
            None => MovementState::Standing {
                frame: next_infinite_frame(frame, STANDING_FRAMES),
            },
            Some(action) => match action {
                MovementAction::Hit => MovementState::Stunt { frame: 0 },
                MovementAction::Land => MovementState::Standing {
                    frame: next_infinite_frame(frame, STANDING_FRAMES),
                },
                MovementAction::LightKick => MovementState::LightKicking { frame: (0, 0) },
                MovementAction::LightPunch => MovementState::LightPunching { frame: (0, 0) },
                MovementAction::StrongKick => MovementState::StrongKicking { frame: (0, 0) },
                MovementAction::StrongPunch => MovementState::StrongPunching { frame: (0, 0) },
                MovementAction::JumpStraight => MovementState::JumpingStraight { starting: true },
                MovementAction::WalkLeft => MovementState::Walking {
                    frame: (0, 0),
                    direction: WalkingDirection::Left,
                },
                MovementAction::WalkRight => MovementState::Walking {
                    frame: (0, 0),
                    direction: WalkingDirection::Right,
                },
            },
        },
        MovementState::Stunt { frame } => {
            if frame < STUNT_FRAMES.1 {
                MovementState::Stunt { frame: frame + 1 }
            } else {
                match action {
                    None => MovementState::Standing { frame: (0, 0) },
                    Some(action) => match action {
                        MovementAction::Hit => MovementState::Stunt { frame: 0 },
                        MovementAction::Land => MovementState::Standing { frame: (0, 0) },
                        MovementAction::LightKick => MovementState::LightKicking { frame: (0, 0) },
                        MovementAction::LightPunch => {
                            MovementState::LightPunching { frame: (0, 0) }
                        }
                        MovementAction::StrongKick => {
                            MovementState::StrongKicking { frame: (0, 0) }
                        }
                        MovementAction::StrongPunch => {
                            MovementState::StrongPunching { frame: (0, 0) }
                        }
                        MovementAction::JumpStraight => {
                            MovementState::JumpingStraight { starting: true }
                        }
                        MovementAction::WalkLeft => MovementState::Walking {
                            frame: (0, 0),
                            direction: WalkingDirection::Left,
                        },
                        MovementAction::WalkRight => MovementState::Walking {
                            frame: (0, 0),
                            direction: WalkingDirection::Right,
                        },
                    },
                }
            }
        }
        MovementState::Walking { frame, direction } => match action {
            None => MovementState::Standing { frame: (0, 0) },
            Some(action) => match action {
                MovementAction::Hit => MovementState::Stunt { frame: 0 },
                MovementAction::Land => MovementState::Standing { frame: (0, 0) },
                MovementAction::LightKick => MovementState::LightKicking { frame: (0, 0) },
                MovementAction::LightPunch => MovementState::LightPunching { frame: (0, 0) },
                MovementAction::StrongKick => MovementState::StrongKicking { frame: (0, 0) },
                MovementAction::StrongPunch => MovementState::StrongPunching { frame: (0, 0) },
                MovementAction::JumpStraight => MovementState::JumpingStraight { starting: true },
                MovementAction::WalkLeft => match direction {
                    WalkingDirection::Left => MovementState::Walking {
                        frame: next_infinite_frame(frame, WALKING_FRAMES),
                        direction,
                    },
                    WalkingDirection::Right => MovementState::Walking {
                        frame: (0, 0),
                        direction: WalkingDirection::Left,
                    },
                },
                MovementAction::WalkRight => match direction {
                    WalkingDirection::Left => MovementState::Walking {
                        frame: (0, 0),
                        direction: WalkingDirection::Right,
                    },
                    WalkingDirection::Right => MovementState::Walking {
                        frame: next_infinite_frame(frame, WALKING_FRAMES),
                        direction,
                    },
                },
            },
        },
        MovementState::JumpingStraight { .. } => match action {
            None => MovementState::JumpingStraight { starting: false },
            Some(action) => match action {
                MovementAction::Hit => MovementState::Stunt { frame: 0 },
                MovementAction::Land => MovementState::Standing { frame: (0, 0) },
                MovementAction::LightKick => MovementState::JumpingStraight { starting: false },
                MovementAction::LightPunch => MovementState::JumpingStraight { starting: false },
                MovementAction::StrongKick => MovementState::JumpingStraight { starting: false },
                MovementAction::StrongPunch => MovementState::JumpingStraight { starting: false },
                MovementAction::JumpStraight => MovementState::JumpingStraight { starting: false },
                MovementAction::WalkLeft => MovementState::JumpingStraight { starting: false },
                MovementAction::WalkRight => MovementState::JumpingStraight { starting: false },
            },
        },
        MovementState::LightKicking { frame } => match next_frame(frame, LIGHT_KICK_FRAMES) {
            Some(frame) => MovementState::LightKicking { frame },
            None => match action {
                None => MovementState::Standing { frame: (0, 0) },
                Some(action) => match action {
                    MovementAction::Hit => MovementState::Stunt { frame: 0 },
                    MovementAction::Land => MovementState::Standing { frame: (0, 0) },
                    MovementAction::LightKick => MovementState::LightKicking { frame: (0, 0) },
                    MovementAction::LightPunch => MovementState::LightPunching { frame: (0, 0) },
                    MovementAction::StrongKick => MovementState::StrongKicking { frame: (0, 0) },
                    MovementAction::StrongPunch => MovementState::StrongPunching { frame: (0, 0) },
                    MovementAction::JumpStraight => {
                        MovementState::JumpingStraight { starting: true }
                    }
                    MovementAction::WalkLeft => MovementState::Walking {
                        frame: (0, 0),
                        direction: WalkingDirection::Left,
                    },
                    MovementAction::WalkRight => MovementState::Walking {
                        frame: (0, 0),
                        direction: WalkingDirection::Right,
                    },
                },
            },
        },
        MovementState::LightPunching { frame } => match next_frame(frame, LIGHT_PUNCH_FRAMES) {
            Some(frame) => MovementState::LightPunching { frame },
            None => match action {
                None => MovementState::Standing { frame: (0, 0) },
                Some(action) => match action {
                    MovementAction::Hit => MovementState::Stunt { frame: 0 },
                    MovementAction::Land => MovementState::Standing { frame: (0, 0) },
                    MovementAction::LightKick => MovementState::LightKicking { frame: (0, 0) },
                    MovementAction::LightPunch => MovementState::LightPunching { frame: (0, 0) },
                    MovementAction::StrongKick => MovementState::StrongKicking { frame: (0, 0) },
                    MovementAction::StrongPunch => MovementState::StrongPunching { frame: (0, 0) },
                    MovementAction::JumpStraight => {
                        MovementState::JumpingStraight { starting: true }
                    }
                    MovementAction::WalkLeft => MovementState::Walking {
                        frame: (0, 0),
                        direction: WalkingDirection::Left,
                    },
                    MovementAction::WalkRight => MovementState::Walking {
                        frame: (0, 0),
                        direction: WalkingDirection::Right,
                    },
                },
            },
        },
        MovementState::StrongKicking { frame } => match next_frame(frame, STRONG_KICK_FRAMES) {
            Some(frame) => MovementState::StrongKicking { frame },
            None => match action {
                None => MovementState::Standing { frame: (0, 0) },
                Some(action) => match action {
                    MovementAction::Hit => MovementState::Stunt { frame: 0 },
                    MovementAction::Land => MovementState::Standing { frame: (0, 0) },
                    MovementAction::LightKick => MovementState::LightKicking { frame: (0, 0) },
                    MovementAction::LightPunch => MovementState::LightPunching { frame: (0, 0) },
                    MovementAction::StrongKick => MovementState::StrongKicking { frame: (0, 0) },
                    MovementAction::StrongPunch => MovementState::StrongPunching { frame: (0, 0) },
                    MovementAction::JumpStraight => {
                        MovementState::JumpingStraight { starting: true }
                    }
                    MovementAction::WalkLeft => MovementState::Walking {
                        frame: (0, 0),
                        direction: WalkingDirection::Left,
                    },
                    MovementAction::WalkRight => MovementState::Walking {
                        frame: (0, 0),
                        direction: WalkingDirection::Right,
                    },
                },
            },
        },
        MovementState::StrongPunching { frame } => match next_frame(frame, STRONG_PUNCH_FRAMES) {
            Some(frame) => MovementState::StrongPunching { frame },
            None => match action {
                None => MovementState::Standing { frame: (0, 0) },
                Some(action) => match action {
                    MovementAction::Hit => MovementState::Stunt { frame: 0 },
                    MovementAction::Land => MovementState::Standing { frame: (0, 0) },
                    MovementAction::LightKick => MovementState::LightKicking { frame: (0, 0) },
                    MovementAction::LightPunch => MovementState::LightPunching { frame: (0, 0) },
                    MovementAction::StrongKick => MovementState::StrongKicking { frame: (0, 0) },
                    MovementAction::StrongPunch => MovementState::StrongPunching { frame: (0, 0) },
                    MovementAction::JumpStraight => {
                        MovementState::JumpingStraight { starting: true }
                    }
                    MovementAction::WalkLeft => MovementState::Walking {
                        frame: (0, 0),
                        direction: WalkingDirection::Left,
                    },
                    MovementAction::WalkRight => MovementState::Walking {
                        frame: (0, 0),
                        direction: WalkingDirection::Right,
                    },
                },
            },
        },
    }
}

fn next_frame(frame: (usize, u8), frames: (usize, u8)) -> Option<(usize, u8)> {
    let next_frame = frame.1 + 1;
    if next_frame < frames.1 {
        Some((frame.0, next_frame))
    } else {
        let next_sprite = frame.0 + 1;
        if next_sprite < frames.0 {
            Some((next_sprite, 0))
        } else {
            None
        }
    }
}
fn next_infinite_frame(frame: (usize, u8), frames: (usize, u8)) -> (usize, u8) {
    let next_frame = frame.1 + 1;
    if next_frame < frames.1 {
        (frame.0, next_frame)
    } else {
        let next_sprite = frame.0 + 1;
        if next_sprite < frames.0 {
            (next_sprite, 0)
        } else {
            (0, 0)
        }
    }
}
