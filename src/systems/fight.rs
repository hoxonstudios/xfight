use sdl2::keyboard::Keycode;

use crate::components::{
    animation::AnimationComponent,
    fighter::{Direction, FighterComponent, FighterState, Player},
    physics::PhysicsComponent,
};

pub struct FightSystem;
impl FightSystem {
    pub fn run<'a>(
        pressed_keys: &Vec<Keycode>,
        entities: impl Iterator<
            Item = (
                (&'a mut FighterComponent, &'a mut AnimationComponent),
                &'a mut PhysicsComponent,
            ),
        >,
    ) {
        for ((fighter, animation), physics) in entities {
            let keys = get_pressed_keys(&fighter.player, pressed_keys);
            let action = handle_keys(&fighter.direction, &keys);
            let state = reduce(&fighter.state, &action);

            match state {
                FighterState::Standing => match fighter.state {
                    FighterState::Standing => {}
                    _ => {
                        println!("Start: Standing");
                        animation.frame = 0;
                        animation.sprite = 0;
                        animation.infinite = true;
                        animation.state = FighterState::Standing.key()
                    }
                },
                FighterState::MovingBackward => match fighter.state {
                    FighterState::MovingBackward => {}
                    _ => {
                        println!("Start: MovingBackward");
                        animation.frame = 0;
                        animation.sprite = 0;
                        animation.infinite = true;
                        animation.state = FighterState::MovingBackward.key()
                    }
                },
                FighterState::MovingForward => match fighter.state {
                    FighterState::MovingForward => {}
                    _ => {
                        println!("Start: MovingForward");
                        animation.frame = 0;
                        animation.sprite = 0;
                        animation.infinite = true;
                        animation.state = FighterState::MovingForward.key()
                    }
                },
            }

            fighter.state = state;
        }
    }
}

fn reduce(state: &FighterState, action: &FighterAction) -> FighterState {
    match state {
        FighterState::Standing => match action {
            FighterAction::MoveForward => FighterState::MovingForward,
            FighterAction::MoveBackward => FighterState::MovingBackward,
            FighterAction::None => FighterState::Standing,
        },
        FighterState::MovingBackward => match action {
            FighterAction::MoveForward => FighterState::MovingForward,
            FighterAction::MoveBackward => FighterState::MovingBackward,
            FighterAction::None => FighterState::Standing,
        },
        FighterState::MovingForward => match action {
            FighterAction::MoveForward => FighterState::MovingForward,
            FighterAction::MoveBackward => FighterState::MovingBackward,
            FighterAction::None => FighterState::Standing,
        },
    }
}

fn handle_keys(direction: &Direction, keys: &PressedKeys) -> FighterAction {
    match keys {
        PressedKeys { left: true, .. } => match direction {
            Direction::Left => FighterAction::MoveForward,
            Direction::Right => FighterAction::MoveBackward,
        },
        PressedKeys { right: true, .. } => match direction {
            Direction::Left => FighterAction::MoveBackward,
            Direction::Right => FighterAction::MoveForward,
        },
        PressedKeys {
            right: false,
            left: false,
        } => FighterAction::None,
    }
}

fn get_pressed_keys(player: &Player, keys: &Vec<Keycode>) -> PressedKeys {
    match player {
        Player::One => PressedKeys {
            left: keys.iter().any(|&k| k == Keycode::A),
            right: keys.iter().any(|&k| k == Keycode::D),
        },
        Player::Two => PressedKeys {
            left: keys.iter().any(|&k| k == Keycode::Left),
            right: keys.iter().any(|&k| k == Keycode::Right),
        },
    }
}

#[derive(Debug)]
struct PressedKeys {
    left: bool,
    right: bool,
}

#[derive(Debug)]
enum FighterAction {
    None,
    MoveForward,
    MoveBackward,
}
