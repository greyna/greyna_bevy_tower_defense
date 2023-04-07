use bevy::{prelude::*, time::Stopwatch};

use crate::game::utils::Cooldown;

#[derive(Resource)]
pub struct Lives(pub u8);

#[derive(Resource)]
pub struct EnemiesSpawnerTimings {
    pub spawn_cooldown: Cooldown,
    pub time_since_start: Stopwatch,
}
