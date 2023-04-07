pub mod blinking;
mod cleanup_systems;
pub mod collisions;
pub mod components;
mod damages;
mod enemies;
mod gold;
pub mod grid;
mod logic_systems;
pub mod schedule;
mod shooting;
mod startup_systems;
mod turret;
pub mod utils;

use schedule::*;

use crate::AppState;
use bevy::prelude::*;

use blinking::BlinkingPlugin;
use collisions::CollisionsPlugin;
use damages::DamagesPlugin;
use enemies::EnemiesPlugin;
use gold::GoldPlugin;
use grid::GridPlugin;
use shooting::ShootingPlugin;
use turret::TurretPlugin;

use cleanup_systems::*;
use logic_systems::*;
use startup_systems::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(SchedulePlugin)
            .add_plugin(GridPlugin)
            .add_plugin(TurretPlugin)
            .add_plugin(BlinkingPlugin)
            .add_plugin(CollisionsPlugin)
            .add_plugin(EnemiesPlugin)
            .add_plugin(ShootingPlugin)
            .add_plugin(DamagesPlugin)
            .add_plugin(GoldPlugin)
            .add_systems(
                (spawn_player, spawn_camera, spawn_target).in_schedule(OnEnter(AppState::Game)),
            )
            .add_systems(
                (despawn_player, despawn_camera, despawn_target)
                    .in_schedule(OnExit(AppState::Game)),
            )
            .add_system(target_cursor.in_set(GameSet::Input))
            .add_system(move_player.in_set(GameSet::LogicMovement));
    }
}
