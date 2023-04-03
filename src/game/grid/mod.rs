use crate::AppState;
use bevy::prelude::*;

use cleanup_systems::*;
use components::*;
use depiction_systems::*;

mod cleanup_systems;
pub mod components;
mod depiction_systems;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Grid::new(128.0))
            .add_system(spawn_terrain.in_schedule(OnEnter(AppState::Game)))
            .add_system(despawn_terrain.in_schedule(OnExit(AppState::Game)));
    }
}
