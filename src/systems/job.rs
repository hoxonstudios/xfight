use super::{aim::AimDirection, health::Player, input::Controller};

#[derive(Copy, Clone)]
pub struct FightJob {
    pub script: usize,
    pub parameters: FightJobParameters,
}
#[derive(Copy, Clone)]
pub enum FightJobParameters {
    None,
    DestroyEntity {
        entity: usize,
    },
    SpawnFighter {
        position: (f32, f32),
        controller: Controller,
        player: Player,
    },
    SpawnSpell {
        position: (f32, f32),
        direction: AimDirection,
        player: Player,
    },
}
