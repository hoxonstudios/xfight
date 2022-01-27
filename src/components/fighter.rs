#[derive(Copy, Clone)]
pub struct FighterComponent {
    pub player: Player,
    pub direction: Direction,
    pub state: FighterState,
}

#[derive(Copy, Clone)]
pub enum Player {
    One,
    Two,
}

#[derive(Copy, Clone)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Copy, Clone, Debug)]
pub enum FighterState {
    Standing,
    MovingBackward,
    MovingForward,
}
impl FighterState {
    pub const fn key(&self) -> usize {
        match self {
            FighterState::Standing => 0,
            FighterState::MovingForward => 1,
            FighterState::MovingBackward => 2,
        }
    }
}
