use xfight_ecs::components::{MovementComponent, PhysicsComponent};

// ACTIONS
pub const RYU_ACTION_NONE: u32 = 0;
pub const RYU_ACTION_MOVE_FORWARD: u32 = 1;
pub const RYU_ACTION_MOVE_BACKWARD: u32 = 2;
pub const RYU_ACTION_JUMP_STRAIGHT: u32 = 3;
// STATES
pub const RYU_STATE_STAND_UP: u32 = 0;
pub const RYU_STATE_MOVING_FORWARD: u32 = 1;
pub const RYU_STATE_MOVING_BACKWARD: u32 = 2;
pub const RYU_STATE_JUMPING_STRAIGHT: u32 = 3;

pub fn movement_logic(state: u32, action: u32, frame: u32) -> (u32, bool) {
    match state {
        RYU_STATE_STAND_UP => match action {
            RYU_ACTION_NONE => (RYU_STATE_STAND_UP, frame > 11),
            RYU_ACTION_MOVE_FORWARD => (RYU_STATE_MOVING_FORWARD, frame > 18),
            RYU_ACTION_MOVE_BACKWARD => (RYU_STATE_MOVING_BACKWARD, frame > 18),
            RYU_ACTION_JUMP_STRAIGHT => (RYU_STATE_JUMPING_STRAIGHT, frame > 21),
            _ => (RYU_STATE_STAND_UP, frame > 11),
        },
        RYU_STATE_MOVING_FORWARD => match action {
            RYU_ACTION_NONE => (RYU_STATE_STAND_UP, frame > 11),
            RYU_ACTION_MOVE_FORWARD => (RYU_STATE_MOVING_FORWARD, frame > 18),
            RYU_ACTION_MOVE_BACKWARD => (RYU_STATE_MOVING_BACKWARD, frame > 18),
            _ => (RYU_STATE_STAND_UP, frame > 11),
        },
        RYU_STATE_MOVING_BACKWARD => match action {
            RYU_ACTION_NONE => (RYU_STATE_STAND_UP, frame > 11),
            RYU_ACTION_MOVE_FORWARD => (RYU_STATE_MOVING_FORWARD, frame > 18),
            RYU_ACTION_MOVE_BACKWARD => (RYU_STATE_MOVING_BACKWARD, frame > 18),
            _ => (RYU_STATE_STAND_UP, frame > 11),
        },
        RYU_STATE_JUMPING_STRAIGHT => match action {
            RYU_ACTION_NONE => (RYU_STATE_JUMPING_STRAIGHT, frame > 21),
            _ => (RYU_STATE_JUMPING_STRAIGHT, frame > 21),
        },
        _ => (RYU_STATE_STAND_UP, frame > 11),
    }
}

pub fn physics_logic(movement: &Option<MovementComponent>, physics: &mut PhysicsComponent) {
    if let Some(movement) = movement {
        match movement.state {
            RYU_STATE_STAND_UP => physics.speed = (0.0, 0.0),
            RYU_STATE_MOVING_FORWARD => physics.speed = (2.0, 0.0),
            RYU_STATE_MOVING_BACKWARD => physics.speed = (-2.0, 0.0),
            RYU_STATE_JUMPING_STRAIGHT => {
                physics.speed = (0.0, -5.0);
                physics.acceleration = (0.0, 10.0)
            }
            _ => {}
        }
    }
}
