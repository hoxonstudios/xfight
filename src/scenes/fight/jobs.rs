use crate::systems::job::FightJobParameters;

use super::{
    fighters::ryu::{
        job_spawn_ryu,
        spells::adoken::{job_destroy_ryu_adoken, job_spawn_ryu_adoken},
    },
    floor::job_spawn_floor,
    FightScene,
};

pub type FightJobScript = fn(&mut FightScene, FightJobParameters);

pub const JOB_SPAWN_FLOOR_INDEX: usize = 0;
pub const JOB_SPAWN_RYU_INDEX: usize = 1;
pub const JOB_SPAWN_RYU_ADOKEN: usize = 2;
pub const JOB_DESTROY_RYU_ADOKEN: usize = 3;

pub const FIGHT_JOBS: &'static [FightJobScript] = &[
    job_spawn_floor,
    job_spawn_ryu,
    job_spawn_ryu_adoken,
    job_destroy_ryu_adoken,
];
