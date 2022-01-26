use xfight_ecs::components::{MovementComponent, PhysicsComponent, ShapeTexture};

use super::{
    movements::{
        RYU_STATE_JUMPING_STRAIGHT, RYU_STATE_MOVING_BACKWARD, RYU_STATE_MOVING_FORWARD,
        RYU_STATE_STAND_UP,
    },
    sprites::{
        RYU_JUMPING_1, RYU_JUMPING_2, RYU_JUMPING_3, RYU_JUMPING_4, RYU_JUMPING_5, RYU_JUMPING_6,
        RYU_STAND_1, RYU_STAND_2, RYU_STAND_3, RYU_STAND_4, RYU_WALKING_1, RYU_WALKING_2,
        RYU_WALKING_3, RYU_WALKING_4, RYU_WALKING_5, RYU_WALKING_6,
    },
};

pub fn ryu_animation(
    movement: &Option<MovementComponent>,
    _: &Option<PhysicsComponent>,
) -> Option<&'static ShapeTexture> {
    let MovementComponent { state, frame, .. } = movement.unwrap();

    match state {
        RYU_STATE_STAND_UP => {
            if frame < 3 {
                Some(&RYU_STAND_1)
            } else if frame < 6 {
                Some(&RYU_STAND_2)
            } else if frame < 9 {
                Some(&RYU_STAND_3)
            } else {
                Some(&RYU_STAND_4)
            }
        }
        RYU_STATE_MOVING_FORWARD => {
            if frame < 3 {
                Some(&RYU_WALKING_1)
            } else if frame < 6 {
                Some(&RYU_WALKING_2)
            } else if frame < 9 {
                Some(&RYU_WALKING_3)
            } else if frame < 12 {
                Some(&RYU_WALKING_4)
            } else if frame < 15 {
                Some(&RYU_WALKING_5)
            } else {
                Some(&RYU_WALKING_6)
            }
        }
        RYU_STATE_MOVING_BACKWARD => {
            if frame < 3 {
                Some(&RYU_WALKING_6)
            } else if frame < 6 {
                Some(&RYU_WALKING_5)
            } else if frame < 9 {
                Some(&RYU_WALKING_4)
            } else if frame < 12 {
                Some(&RYU_WALKING_3)
            } else if frame < 15 {
                Some(&RYU_WALKING_2)
            } else {
                Some(&RYU_WALKING_1)
            }
        }
        RYU_STATE_JUMPING_STRAIGHT => {
            if frame < 3 {
                Some(&RYU_WALKING_6)
            } else if frame < 6 {
                Some(&RYU_JUMPING_1)
            } else if frame < 9 {
                Some(&RYU_JUMPING_2)
            } else if frame < 12 {
                Some(&RYU_JUMPING_3)
            } else if frame < 15 {
                Some(&RYU_JUMPING_4)
            } else if frame < 18 {
                Some(&RYU_JUMPING_5)
            } else {
                Some(&RYU_JUMPING_6)
            }
        }
        _ => None,
    }
}
