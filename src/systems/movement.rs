use super::{
    damage::{DamageAction, DamagePoint, DamageSystem},
    health::{HealthSystem, Shield},
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

const CROUCH_LIGHT_PUNCH_FRAMES: (usize, u8) = (3, 3);
const CROUCH_STRONG_PUNCH_FRAMES: (usize, u8) = (5, 3);
const CROUCH_LIGHT_KICK_FRAMES: (usize, u8) = (3, 3);
const CROUCH_STRONG_KICK_FRAMES: (usize, u8) = (3, 3);

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
    pub block: (Sprite, Shield),
    pub crouch_block: (Sprite, Shield),
    pub light_punch: [(Sprite, Option<DamagePoint>); LIGHT_PUNCH_FRAMES.0],
    pub strong_punch: [(Sprite, Option<DamagePoint>); STRONG_PUNCH_FRAMES.0],
    pub light_kick: [(Sprite, Option<DamagePoint>); LIGHT_KICK_FRAMES.0],
    pub strong_kick: [(Sprite, Option<DamagePoint>); STRONG_KICK_FRAMES.0],
    pub crouching: Sprite,
    pub crouch_light_punch: [(Sprite, Option<DamagePoint>); CROUCH_LIGHT_PUNCH_FRAMES.0],
    pub crouch_strong_punch: [(Sprite, Option<DamagePoint>); CROUCH_STRONG_PUNCH_FRAMES.0],
    pub crouch_light_kick: [(Sprite, Option<DamagePoint>); CROUCH_LIGHT_KICK_FRAMES.0],
    pub crouch_strong_kick: [(Sprite, Option<DamagePoint>); CROUCH_STRONG_KICK_FRAMES.0],
}
#[derive(Copy, Clone, Debug)]
pub enum MovementAction {
    WalkRight,
    WalkLeft,
    Crouch,

    Block,
    CrouchBlock,

    LightPunch,
    StrongPunch,
    LightKick,
    StrongKick,

    CrouchLightPunch,
    CrouchStrongPunch,
    CrouchLightKick,
    CrouchStrongKick,

    JumpStraight,

    Land,

    BlockedHit,
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
    Blocking,
    CrouchBlocking,

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

    Crouching,
    CrouchLightPunching {
        frame: (usize, u8),
    },
    CrouchStrongPunching {
        frame: (usize, u8),
    },
    CrouchLightKicking {
        frame: (usize, u8),
    },
    CrouchStrongKicking {
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
        health_system: &mut HealthSystem,
    ) {
        for movement in self.store.data_mut() {
            let entity = movement.entity;
            let transition = calculate_transition(movement.state, movement.action);

            if movement.state != transition {
                match movement.state {
                    MovementState::Blocking => {
                        if let Some(health) = health_system.store.get_mut_component(entity) {
                            health.shield = None;
                        }
                    }
                    MovementState::CrouchBlocking => {
                        if let Some(health) = health_system.store.get_mut_component(entity) {
                            health.shield = None;
                        }
                    }
                    _ => {}
                }
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
                MovementState::Crouching => {
                    if let Some(shape) = shape_system.store.get_mut_component(entity) {
                        shape.action = ShapeAction::Update {
                            sprite: movement.sprites.crouching,
                            flipped: shape.flipped,
                        };
                    }
                    if let Some(velocity) = velocity_system.store.get_mut_component(entity) {
                        velocity.velocity.0 = 0.0;
                    }
                }
                MovementState::Blocking => {
                    if let Some(shape) = shape_system.store.get_mut_component(entity) {
                        shape.action = ShapeAction::Update {
                            sprite: movement.sprites.block.0,
                            flipped: shape.flipped,
                        };
                    }
                    if let Some(velocity) = velocity_system.store.get_mut_component(entity) {
                        velocity.velocity.0 = 0.0;
                    }
                    if let Some(health) = health_system.store.get_mut_component(entity) {
                        health.shield = Some(movement.sprites.block.1);
                    }
                }
                MovementState::CrouchBlocking => {
                    if let Some(shape) = shape_system.store.get_mut_component(entity) {
                        shape.action = ShapeAction::Update {
                            sprite: movement.sprites.crouch_block.0,
                            flipped: shape.flipped,
                        };
                    }
                    if let Some(velocity) = velocity_system.store.get_mut_component(entity) {
                        velocity.velocity.0 = 0.0;
                    }
                    if let Some(health) = health_system.store.get_mut_component(entity) {
                        health.shield = Some(movement.sprites.crouch_block.1);
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
                MovementState::CrouchLightPunching {
                    frame: (sprite, frame),
                } => {
                    if frame == 0 {
                        if let Some(velocity) = velocity_system.store.get_mut_component(entity) {
                            velocity.velocity.0 = 0.0;
                        }
                        let sprite = movement.sprites.crouch_light_punch[sprite];
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
                MovementState::CrouchStrongPunching {
                    frame: (sprite, frame),
                } => {
                    if frame == 0 {
                        if let Some(velocity) = velocity_system.store.get_mut_component(entity) {
                            velocity.velocity.0 = 0.0;
                        }
                        let sprite = movement.sprites.crouch_strong_punch[sprite];
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
                MovementState::CrouchLightKicking {
                    frame: (sprite, frame),
                } => {
                    if frame == 0 {
                        if let Some(velocity) = velocity_system.store.get_mut_component(entity) {
                            velocity.velocity.0 = 0.0;
                        }
                        let sprite = movement.sprites.crouch_light_kick[sprite];
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
                MovementState::CrouchStrongKicking {
                    frame: (sprite, frame),
                } => {
                    if frame == 0 {
                        if let Some(velocity) = velocity_system.store.get_mut_component(entity) {
                            velocity.velocity.0 = 0.0;
                        }
                        let sprite = movement.sprites.crouch_strong_kick[sprite];
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
                MovementAction::BlockedHit => MovementState::Blocking,
                MovementAction::Land => MovementState::Standing {
                    frame: next_infinite_frame(frame, STANDING_FRAMES),
                },
                MovementAction::Crouch => MovementState::Crouching,
                MovementAction::Block => MovementState::Blocking,
                MovementAction::CrouchBlock => MovementState::CrouchBlocking,
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
                MovementAction::CrouchLightPunch => {
                    MovementState::CrouchLightPunching { frame: (0, 0) }
                }
                MovementAction::CrouchStrongPunch => {
                    MovementState::CrouchStrongPunching { frame: (0, 0) }
                }
                MovementAction::CrouchLightKick => {
                    MovementState::CrouchLightKicking { frame: (0, 0) }
                }
                MovementAction::CrouchStrongKick => {
                    MovementState::CrouchStrongKicking { frame: (0, 0) }
                }
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
                        MovementAction::BlockedHit => MovementState::Blocking,
                        MovementAction::Land => MovementState::Standing { frame: (0, 0) },
                        MovementAction::Crouch => MovementState::Crouching,
                        MovementAction::Block => MovementState::Blocking,
                        MovementAction::CrouchBlock => MovementState::CrouchBlocking,
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
                        MovementAction::CrouchLightPunch => {
                            MovementState::CrouchLightPunching { frame: (0, 0) }
                        }
                        MovementAction::CrouchStrongPunch => {
                            MovementState::CrouchStrongPunching { frame: (0, 0) }
                        }
                        MovementAction::CrouchLightKick => {
                            MovementState::CrouchLightKicking { frame: (0, 0) }
                        }
                        MovementAction::CrouchStrongKick => {
                            MovementState::CrouchStrongKicking { frame: (0, 0) }
                        }
                    },
                }
            }
        }
        MovementState::Crouching => match action {
            None => MovementState::Standing { frame: (0, 0) },
            Some(action) => match action {
                MovementAction::Hit => MovementState::Stunt { frame: 0 },
                MovementAction::BlockedHit => MovementState::CrouchBlocking,
                MovementAction::Land => MovementState::Standing { frame: (0, 0) },
                MovementAction::Crouch => MovementState::Crouching,
                MovementAction::Block => MovementState::Blocking,
                MovementAction::CrouchBlock => MovementState::CrouchBlocking,
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
                MovementAction::CrouchLightPunch => {
                    MovementState::CrouchLightPunching { frame: (0, 0) }
                }
                MovementAction::CrouchStrongPunch => {
                    MovementState::CrouchStrongPunching { frame: (0, 0) }
                }
                MovementAction::CrouchLightKick => {
                    MovementState::CrouchLightKicking { frame: (0, 0) }
                }
                MovementAction::CrouchStrongKick => {
                    MovementState::CrouchStrongKicking { frame: (0, 0) }
                }
            },
        },
        MovementState::Blocking => match action {
            None => MovementState::Standing { frame: (0, 0) },
            Some(action) => match action {
                MovementAction::Hit => MovementState::Stunt { frame: 0 },
                MovementAction::BlockedHit => MovementState::Blocking,
                MovementAction::Land => MovementState::Standing { frame: (0, 0) },
                MovementAction::Crouch => MovementState::Crouching,
                MovementAction::Block => MovementState::Blocking,
                MovementAction::CrouchBlock => MovementState::CrouchBlocking,
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
                MovementAction::CrouchLightPunch => {
                    MovementState::CrouchLightPunching { frame: (0, 0) }
                }
                MovementAction::CrouchStrongPunch => {
                    MovementState::CrouchStrongPunching { frame: (0, 0) }
                }
                MovementAction::CrouchLightKick => {
                    MovementState::CrouchLightKicking { frame: (0, 0) }
                }
                MovementAction::CrouchStrongKick => {
                    MovementState::CrouchStrongKicking { frame: (0, 0) }
                }
            },
        },
        MovementState::CrouchBlocking => match action {
            None => MovementState::Standing { frame: (0, 0) },
            Some(action) => match action {
                MovementAction::Hit => MovementState::Stunt { frame: 0 },
                MovementAction::BlockedHit => MovementState::CrouchBlocking,
                MovementAction::Land => MovementState::Standing { frame: (0, 0) },
                MovementAction::Crouch => MovementState::Crouching,
                MovementAction::Block => MovementState::Blocking,
                MovementAction::CrouchBlock => MovementState::CrouchBlocking,
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
                MovementAction::CrouchLightPunch => {
                    MovementState::CrouchLightPunching { frame: (0, 0) }
                }
                MovementAction::CrouchStrongPunch => {
                    MovementState::CrouchStrongPunching { frame: (0, 0) }
                }
                MovementAction::CrouchLightKick => {
                    MovementState::CrouchLightKicking { frame: (0, 0) }
                }
                MovementAction::CrouchStrongKick => {
                    MovementState::CrouchStrongKicking { frame: (0, 0) }
                }
            },
        },
        MovementState::Walking { frame, direction } => match action {
            None => MovementState::Standing { frame: (0, 0) },
            Some(action) => match action {
                MovementAction::Hit => MovementState::Stunt { frame: 0 },
                MovementAction::BlockedHit => MovementState::Blocking,
                MovementAction::Land => MovementState::Standing { frame: (0, 0) },
                MovementAction::Crouch => MovementState::Crouching,
                MovementAction::Block => MovementState::Blocking,
                MovementAction::CrouchBlock => MovementState::CrouchBlocking,
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
                MovementAction::CrouchLightPunch => {
                    MovementState::CrouchLightPunching { frame: (0, 0) }
                }
                MovementAction::CrouchStrongPunch => {
                    MovementState::CrouchStrongPunching { frame: (0, 0) }
                }
                MovementAction::CrouchLightKick => {
                    MovementState::CrouchLightKicking { frame: (0, 0) }
                }
                MovementAction::CrouchStrongKick => {
                    MovementState::CrouchStrongKicking { frame: (0, 0) }
                }
            },
        },
        MovementState::JumpingStraight { .. } => match action {
            None => MovementState::JumpingStraight { starting: false },
            Some(action) => match action {
                MovementAction::Hit => MovementState::Stunt { frame: 0 },
                MovementAction::BlockedHit => MovementState::Blocking,
                MovementAction::Land => MovementState::Standing { frame: (0, 0) },
                MovementAction::Crouch => MovementState::JumpingStraight { starting: false },
                MovementAction::Block => MovementState::JumpingStraight { starting: false },
                MovementAction::CrouchBlock => MovementState::JumpingStraight { starting: false },
                MovementAction::LightKick => MovementState::JumpingStraight { starting: false },
                MovementAction::LightPunch => MovementState::JumpingStraight { starting: false },
                MovementAction::StrongKick => MovementState::JumpingStraight { starting: false },
                MovementAction::StrongPunch => MovementState::JumpingStraight { starting: false },
                MovementAction::JumpStraight => MovementState::JumpingStraight { starting: false },
                MovementAction::WalkLeft => MovementState::JumpingStraight { starting: false },
                MovementAction::WalkRight => MovementState::JumpingStraight { starting: false },
                MovementAction::CrouchLightPunch => {
                    MovementState::JumpingStraight { starting: false }
                }
                MovementAction::CrouchStrongPunch => {
                    MovementState::JumpingStraight { starting: false }
                }
                MovementAction::CrouchLightKick => {
                    MovementState::JumpingStraight { starting: false }
                }
                MovementAction::CrouchStrongKick => {
                    MovementState::JumpingStraight { starting: false }
                }
            },
        },
        MovementState::LightKicking { frame } => match next_frame(frame, LIGHT_KICK_FRAMES) {
            Some(frame) => MovementState::LightKicking { frame },
            None => match action {
                None => MovementState::Standing { frame: (0, 0) },
                Some(action) => match action {
                    MovementAction::Hit => MovementState::Stunt { frame: 0 },
                    MovementAction::BlockedHit => MovementState::Blocking,
                    MovementAction::Land => MovementState::Standing { frame: (0, 0) },
                    MovementAction::Crouch => MovementState::Crouching,
                    MovementAction::Block => MovementState::Blocking,
                    MovementAction::CrouchBlock => MovementState::CrouchBlocking,
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
                    MovementAction::CrouchLightPunch => {
                        MovementState::CrouchLightPunching { frame: (0, 0) }
                    }
                    MovementAction::CrouchStrongPunch => {
                        MovementState::CrouchStrongPunching { frame: (0, 0) }
                    }
                    MovementAction::CrouchLightKick => {
                        MovementState::CrouchLightKicking { frame: (0, 0) }
                    }
                    MovementAction::CrouchStrongKick => {
                        MovementState::CrouchStrongKicking { frame: (0, 0) }
                    }
                },
            },
        },
        MovementState::LightPunching { frame } => match next_frame(frame, LIGHT_PUNCH_FRAMES) {
            Some(frame) => MovementState::LightPunching { frame },
            None => match action {
                None => MovementState::Standing { frame: (0, 0) },
                Some(action) => match action {
                    MovementAction::Hit => MovementState::Stunt { frame: 0 },
                    MovementAction::BlockedHit => MovementState::Blocking,
                    MovementAction::Land => MovementState::Standing { frame: (0, 0) },
                    MovementAction::Crouch => MovementState::Crouching,
                    MovementAction::Block => MovementState::Blocking,
                    MovementAction::CrouchBlock => MovementState::CrouchBlocking,
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
                    MovementAction::CrouchLightPunch => {
                        MovementState::CrouchLightPunching { frame: (0, 0) }
                    }
                    MovementAction::CrouchStrongPunch => {
                        MovementState::CrouchStrongPunching { frame: (0, 0) }
                    }
                    MovementAction::CrouchLightKick => {
                        MovementState::CrouchLightKicking { frame: (0, 0) }
                    }
                    MovementAction::CrouchStrongKick => {
                        MovementState::CrouchStrongKicking { frame: (0, 0) }
                    }
                },
            },
        },
        MovementState::StrongKicking { frame } => match next_frame(frame, STRONG_KICK_FRAMES) {
            Some(frame) => MovementState::StrongKicking { frame },
            None => match action {
                None => MovementState::Standing { frame: (0, 0) },
                Some(action) => match action {
                    MovementAction::Hit => MovementState::Stunt { frame: 0 },
                    MovementAction::BlockedHit => MovementState::Blocking,
                    MovementAction::Land => MovementState::Standing { frame: (0, 0) },
                    MovementAction::Crouch => MovementState::Crouching,
                    MovementAction::Block => MovementState::Blocking,
                    MovementAction::CrouchBlock => MovementState::CrouchBlocking,
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
                    MovementAction::CrouchLightPunch => {
                        MovementState::CrouchLightPunching { frame: (0, 0) }
                    }
                    MovementAction::CrouchStrongPunch => {
                        MovementState::CrouchStrongPunching { frame: (0, 0) }
                    }
                    MovementAction::CrouchLightKick => {
                        MovementState::CrouchLightKicking { frame: (0, 0) }
                    }
                    MovementAction::CrouchStrongKick => {
                        MovementState::CrouchStrongKicking { frame: (0, 0) }
                    }
                },
            },
        },
        MovementState::StrongPunching { frame } => match next_frame(frame, STRONG_PUNCH_FRAMES) {
            Some(frame) => MovementState::StrongPunching { frame },
            None => match action {
                None => MovementState::Standing { frame: (0, 0) },
                Some(action) => match action {
                    MovementAction::Hit => MovementState::Stunt { frame: 0 },
                    MovementAction::BlockedHit => MovementState::Blocking,
                    MovementAction::Land => MovementState::Standing { frame: (0, 0) },
                    MovementAction::Crouch => MovementState::Crouching,
                    MovementAction::Block => MovementState::Blocking,
                    MovementAction::CrouchBlock => MovementState::CrouchBlocking,
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
                    MovementAction::CrouchLightPunch => {
                        MovementState::CrouchLightPunching { frame: (0, 0) }
                    }
                    MovementAction::CrouchStrongPunch => {
                        MovementState::CrouchStrongPunching { frame: (0, 0) }
                    }
                    MovementAction::CrouchLightKick => {
                        MovementState::CrouchLightKicking { frame: (0, 0) }
                    }
                    MovementAction::CrouchStrongKick => {
                        MovementState::CrouchStrongKicking { frame: (0, 0) }
                    }
                },
            },
        },
        MovementState::CrouchLightPunching { frame } => {
            match next_frame(frame, CROUCH_LIGHT_PUNCH_FRAMES) {
                Some(frame) => MovementState::CrouchLightPunching { frame },
                None => match action {
                    None => MovementState::Standing { frame: (0, 0) },
                    Some(action) => match action {
                        MovementAction::Hit => MovementState::Stunt { frame: 0 },
                        MovementAction::BlockedHit => MovementState::Blocking,
                        MovementAction::Land => MovementState::Standing { frame: (0, 0) },
                        MovementAction::Crouch => MovementState::Crouching,
                        MovementAction::Block => MovementState::Blocking,
                        MovementAction::CrouchBlock => MovementState::CrouchBlocking,
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
                        MovementAction::CrouchLightPunch => {
                            MovementState::CrouchLightPunching { frame: (0, 0) }
                        }
                        MovementAction::CrouchStrongPunch => {
                            MovementState::CrouchStrongPunching { frame: (0, 0) }
                        }
                        MovementAction::CrouchLightKick => {
                            MovementState::CrouchLightKicking { frame: (0, 0) }
                        }
                        MovementAction::CrouchStrongKick => {
                            MovementState::CrouchStrongKicking { frame: (0, 0) }
                        }
                    },
                },
            }
        }
        MovementState::CrouchStrongPunching { frame } => {
            match next_frame(frame, CROUCH_STRONG_PUNCH_FRAMES) {
                Some(frame) => MovementState::CrouchStrongPunching { frame },
                None => match action {
                    None => MovementState::Standing { frame: (0, 0) },
                    Some(action) => match action {
                        MovementAction::Hit => MovementState::Stunt { frame: 0 },
                        MovementAction::BlockedHit => MovementState::Blocking,
                        MovementAction::Land => MovementState::Standing { frame: (0, 0) },
                        MovementAction::Crouch => MovementState::Crouching,
                        MovementAction::Block => MovementState::Blocking,
                        MovementAction::CrouchBlock => MovementState::CrouchBlocking,
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
                        MovementAction::CrouchLightPunch => {
                            MovementState::CrouchLightPunching { frame: (0, 0) }
                        }
                        MovementAction::CrouchStrongPunch => {
                            MovementState::CrouchStrongPunching { frame: (0, 0) }
                        }
                        MovementAction::CrouchLightKick => {
                            MovementState::CrouchLightKicking { frame: (0, 0) }
                        }
                        MovementAction::CrouchStrongKick => {
                            MovementState::CrouchStrongKicking { frame: (0, 0) }
                        }
                    },
                },
            }
        }
        MovementState::CrouchLightKicking { frame } => {
            match next_frame(frame, CROUCH_LIGHT_KICK_FRAMES) {
                Some(frame) => MovementState::CrouchLightKicking { frame },
                None => match action {
                    None => MovementState::Standing { frame: (0, 0) },
                    Some(action) => match action {
                        MovementAction::Hit => MovementState::Stunt { frame: 0 },
                        MovementAction::BlockedHit => MovementState::Blocking,
                        MovementAction::Land => MovementState::Standing { frame: (0, 0) },
                        MovementAction::Crouch => MovementState::Crouching,
                        MovementAction::Block => MovementState::Blocking,
                        MovementAction::CrouchBlock => MovementState::CrouchBlocking,
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
                        MovementAction::CrouchLightPunch => {
                            MovementState::CrouchLightPunching { frame: (0, 0) }
                        }
                        MovementAction::CrouchStrongPunch => {
                            MovementState::CrouchStrongPunching { frame: (0, 0) }
                        }
                        MovementAction::CrouchLightKick => {
                            MovementState::CrouchLightKicking { frame: (0, 0) }
                        }
                        MovementAction::CrouchStrongKick => {
                            MovementState::CrouchStrongKicking { frame: (0, 0) }
                        }
                    },
                },
            }
        }
        MovementState::CrouchStrongKicking { frame } => {
            match next_frame(frame, CROUCH_STRONG_KICK_FRAMES) {
                Some(frame) => MovementState::CrouchStrongKicking { frame },
                None => match action {
                    None => MovementState::Standing { frame: (0, 0) },
                    Some(action) => match action {
                        MovementAction::Hit => MovementState::Stunt { frame: 0 },
                        MovementAction::BlockedHit => MovementState::Blocking,
                        MovementAction::Land => MovementState::Standing { frame: (0, 0) },
                        MovementAction::Crouch => MovementState::Crouching,
                        MovementAction::Block => MovementState::Blocking,
                        MovementAction::CrouchBlock => MovementState::CrouchBlocking,
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
                        MovementAction::CrouchLightPunch => {
                            MovementState::CrouchLightPunching { frame: (0, 0) }
                        }
                        MovementAction::CrouchStrongPunch => {
                            MovementState::CrouchStrongPunching { frame: (0, 0) }
                        }
                        MovementAction::CrouchLightKick => {
                            MovementState::CrouchLightKicking { frame: (0, 0) }
                        }
                        MovementAction::CrouchStrongKick => {
                            MovementState::CrouchStrongKicking { frame: (0, 0) }
                        }
                    },
                },
            }
        }
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
