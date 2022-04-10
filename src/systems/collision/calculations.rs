use super::{Point, Rectangle};

fn check_collision(source: &Rectangle, target: &Rectangle) -> bool {
    source.x0 <= target.x1
        && source.x1 >= target.x0
        && source.y0 <= target.y1
        && source.y1 >= target.y0
}
pub(super) fn get_collision_time(
    source: &Rectangle,
    velocity: &Point,
    target: &Rectangle,
) -> Option<(Option<f32>, Option<f32>, Option<f32>, Option<f32>)> {
    if check_collision(source, target) {
        let left = if target.x0 < source.x0 && source.x1 >= target.x1 {
            Some(0.0f32)
        } else {
            None
        };
        let right = if target.x1 > source.x1 && source.x0 <= target.x0 {
            Some(0.0f32)
        } else {
            None
        };
        let top = if target.y0 < source.y0 && source.y1 >= target.y1 {
            Some(0.0f32)
        } else {
            None
        };
        let bottom = if target.y1 > source.y1 && source.y0 <= target.y0 {
            Some(0.0f32)
        } else {
            None
        };

        Some((left, top, right, bottom))
    } else {
        let moved_source = Rectangle {
            x0: source.x0 + velocity.x,
            x1: source.x1 + velocity.x,
            y0: source.y0 + velocity.y,
            y1: source.y1 + velocity.y,
        };
        if check_collision(&moved_source, target) {
            let left = if velocity.x == 0.0 {
                None
            } else if velocity.x < 0.0 {
                let delta = (target.x1 - source.x0) / velocity.x;
                if delta < 0.0 {
                    None
                } else {
                    Some(delta)
                }
            } else {
                None
            };
            let right = if velocity.x == 0.0 {
                None
            } else if velocity.x > 0.0 {
                let delta = (target.x0 - source.x1) / velocity.x;
                if delta < 0.0 {
                    None
                } else {
                    Some(delta)
                }
            } else {
                None
            };
            let top = if velocity.y == 0.0 {
                None
            } else if velocity.y < 0.0 {
                let delta = (target.y1 - source.y0) / velocity.y;
                if delta < 0.0 {
                    None
                } else {
                    Some(delta)
                }
            } else {
                None
            };
            let bottom = if velocity.y == 0.0 {
                None
            } else if velocity.y > 0.0 {
                let delta = (target.y0 - source.y1) / velocity.y;
                if delta < 0.0 {
                    None
                } else {
                    Some(delta)
                }
            } else {
                None
            };

            let horizontal = left.or(right);
            let vertical = top.or(bottom);

            match (vertical, horizontal) {
                (Some(vertical), Some(horizontal)) => {
                    if horizontal > vertical {
                        Some((left, None, right, None))
                    } else if horizontal < vertical {
                        Some((None, top, None, bottom))
                    } else {
                        Some((left, top, right, bottom))
                    }
                }
                _ => Some((left, top, right, bottom)),
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    mod check_collision {
        use crate::systems::collision::{calculations::check_collision, Rectangle};

        #[test]
        fn it_returns_true_when_there_is_collision() {
            let source = Rectangle {
                x0: 0.0,
                x1: 10.0,
                y0: 0.0,
                y1: 10.0,
            };
            let target = Rectangle {
                x0: 5.0,
                x1: 15.0,
                y0: 0.0,
                y1: 10.0,
            };

            let collide = check_collision(&source, &target);

            assert!(collide);
        }
        #[test]
        fn it_returns_false_when_there_is_no_collision() {
            let source = Rectangle {
                x0: 0.0,
                x1: 10.0,
                y0: 0.0,
                y1: 10.0,
            };
            let target = Rectangle {
                x0: 5.0,
                x1: 15.0,
                y0: 20.0,
                y1: 25.0,
            };

            let collide = check_collision(&source, &target);

            assert!(!collide);
        }
    }
    mod get_collision_time_with_velocity {
        use crate::systems::collision::{calculations::get_collision_time, Point, Rectangle};

        #[test]
        fn it_returns_no_collisions() {
            let source = Rectangle {
                x0: 0.0,
                x1: 10.0,
                y0: 0.0,
                y1: 10.0,
            };
            let velocity = Point { x: 1.0, y: 0.0 };
            let target = Rectangle {
                x0: 20.0,
                x1: 25.0,
                y0: 0.0,
                y1: 5.0,
            };

            let result = get_collision_time(&source, &velocity, &target);

            assert_eq!(result, None);
        }
        #[test]
        fn it_returns_right_collision() {
            let source = Rectangle {
                x0: 0.0,
                x1: 10.0,
                y0: 0.0,
                y1: 10.0,
            };
            let velocity = Point { x: 20.0, y: 0.0 };
            let target = Rectangle {
                x0: 20.0,
                x1: 25.0,
                y0: 0.0,
                y1: 5.0,
            };

            let result = get_collision_time(&source, &velocity, &target);

            assert_eq!(result, Some((None, None, Some(0.5), None)));
        }
        #[test]
        fn it_returns_left_collision() {
            let source = Rectangle {
                x0: 20.0,
                x1: 25.0,
                y0: 0.0,
                y1: 5.0,
            };
            let velocity = Point { x: -20.0, y: 0.0 };
            let target = Rectangle {
                x0: 0.0,
                x1: 10.0,
                y0: 0.0,
                y1: 10.0,
            };

            let result = get_collision_time(&source, &velocity, &target);

            assert_eq!(result, Some((Some(0.5), None, None, None)));
        }
        #[test]
        fn it_returns_top_collision() {
            let source = Rectangle {
                x0: 0.0,
                x1: 10.0,
                y0: 15.0,
                y1: 20.0,
            };
            let velocity = Point { x: 0.0, y: -10.0 };
            let target = Rectangle {
                x0: 0.0,
                x1: 10.0,
                y0: 0.0,
                y1: 10.0,
            };

            let result = get_collision_time(&source, &velocity, &target);

            assert_eq!(result, Some((None, Some(0.5), None, None)));
        }
        #[test]
        fn it_returns_bottom_collision() {
            let source = Rectangle {
                x0: 0.0,
                x1: 10.0,
                y0: 0.0,
                y1: 10.0,
            };
            let velocity = Point { x: 0.0, y: 10.0 };
            let target = Rectangle {
                x0: 0.0,
                x1: 10.0,
                y0: 15.0,
                y1: 20.0,
            };

            let result = get_collision_time(&source, &velocity, &target);

            assert_eq!(result, Some((None, None, None, Some(0.5))));
        }
        #[test]
        fn it_returns_right_collision_when_already_colliding() {
            let source = Rectangle {
                x0: 0.0,
                x1: 10.0,
                y0: 0.0,
                y1: 10.0,
            };
            let velocity = Point { x: 15.0, y: 0.0 };
            let target = Rectangle {
                x0: 5.0,
                x1: 15.0,
                y0: 0.0,
                y1: 5.0,
            };

            let result = get_collision_time(&source, &velocity, &target);

            assert_eq!(result, Some((None, None, Some(0.0), None)));
        }
        #[test]
        fn it_returns_left_collision_when_already_colliding() {
            let source = Rectangle {
                x0: 5.0,
                x1: 15.0,
                y0: 5.0,
                y1: 10.0,
            };
            let velocity = Point { x: -15.0, y: 0.0 };
            let target = Rectangle {
                x0: 0.0,
                x1: 10.0,
                y0: 0.0,
                y1: 15.0,
            };

            let result = get_collision_time(&source, &velocity, &target);

            assert_eq!(result, Some((Some(0.0), None, None, None)));
        }
        #[test]
        fn it_returns_top_collision_when_already_colliding() {
            let source = Rectangle {
                x0: 0.0,
                x1: 10.0,
                y0: 5.0,
                y1: 15.0,
            };
            let velocity = Point { x: 0.0, y: -10.0 };
            let target = Rectangle {
                x0: 0.0,
                x1: 10.0,
                y0: 0.0,
                y1: 10.0,
            };

            let result = get_collision_time(&source, &velocity, &target);

            assert_eq!(result, Some((None, Some(0.0), None, None)));
        }
        #[test]
        fn it_returns_bottom_collision_when_already_colliding() {
            let source = Rectangle {
                x0: 0.0,
                x1: 10.0,
                y0: 0.0,
                y1: 10.0,
            };
            let velocity = Point { x: 0.0, y: 10.0 };
            let target = Rectangle {
                x0: 0.0,
                x1: 10.0,
                y0: 5.0,
                y1: 15.0,
            };

            let result = get_collision_time(&source, &velocity, &target);

            assert_eq!(result, Some((None, None, None, Some(0.0))));
        }
        #[test]
        fn it_returns_right_and_top_collision_when_already_colliding_without_velocity() {
            let source = Rectangle {
                x0: 0.0,
                x1: 10.0,
                y0: 5.0,
                y1: 10.0,
            };
            let velocity = Point { x: 0.0, y: 0.0 };
            let target = Rectangle {
                x0: 5.0,
                x1: 15.0,
                y0: 0.0,
                y1: 10.0,
            };

            let result = get_collision_time(&source, &velocity, &target);

            assert_eq!(result, Some((None, Some(0.0), Some(0.0), None)));
        }
        #[test]
        fn it_returns_left_and_bottom_collision_when_already_colliding_without_velocity() {
            let source = Rectangle {
                x0: 5.0,
                x1: 15.0,
                y0: 0.0,
                y1: 5.0,
            };
            let velocity = Point { x: 0.0, y: 0.0 };
            let target = Rectangle {
                x0: 0.0,
                x1: 10.0,
                y0: 0.0,
                y1: 10.0,
            };

            let result = get_collision_time(&source, &velocity, &target);

            assert_eq!(result, Some((Some(0.0), None, None, Some(0.0))));
        }
        #[test]
        fn it_returns_top_collision_when_already_colliding_without_velocity() {
            let source = Rectangle {
                x0: 0.0,
                x1: 10.0,
                y0: 5.0,
                y1: 15.0,
            };
            let velocity = Point { x: 0.0, y: -10.0 };
            let target = Rectangle {
                x0: 0.0,
                x1: 10.0,
                y0: 0.0,
                y1: 10.0,
            };

            let result = get_collision_time(&source, &velocity, &target);

            assert_eq!(result, Some((None, Some(0.0), None, None)));
        }
        #[test]
        fn it_returns_bottom_collision_when_already_colliding_without_velocity() {
            let source = Rectangle {
                x0: 0.0,
                x1: 10.0,
                y0: 0.0,
                y1: 10.0,
            };
            let velocity = Point { x: 0.0, y: 10.0 };
            let target = Rectangle {
                x0: 0.0,
                x1: 10.0,
                y0: 5.0,
                y1: 15.0,
            };

            let result = get_collision_time(&source, &velocity, &target);

            assert_eq!(result, Some((None, None, None, Some(0.0))));
        }
        #[test]
        fn it_returns_collision_without_side_when_there_is_no_velocity_and_is_inside_the_target() {
            let source = Rectangle {
                x0: 5.0,
                x1: 15.0,
                y0: 5.0,
                y1: 15.0,
            };
            let velocity = Point { x: 0.0, y: 10.0 };
            let target = Rectangle {
                x0: 0.0,
                x1: 20.0,
                y0: 0.0,
                y1: 20.0,
            };

            let result = get_collision_time(&source, &velocity, &target);

            assert_eq!(result, Some((None, None, None, None)));
        }
        #[test]
        fn it_returns_only_first_edge_collision() {
            let source = Rectangle {
                x0: 0.0,
                x1: 5.0,
                y0: 10.0,
                y1: 15.0,
            };
            let velocity = Point { x: 10.0, y: -10.0 };
            let target = Rectangle {
                x0: 10.0,
                x1: 20.0,
                y0: 0.0,
                y1: 20.0,
            };

            let result = get_collision_time(&source, &velocity, &target);

            assert_eq!(result, Some((None, None, Some(0.5), None)));
        }
    }
}
