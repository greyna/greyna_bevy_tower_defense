use crate::AppState;
use bevy::{prelude::*, window::PrimaryWindow};

use cleanup_systems::*;
use components::*;
use depiction_systems::*;

mod cleanup_systems;
pub mod components;
mod depiction_systems;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                create_grid,
                apply_system_buffers.after(create_grid),
                spawn_terrain.after(apply_system_buffers),
            )
                .in_schedule(OnEnter(AppState::Game)),
        )
        .add_system(despawn_terrain.in_schedule(OnExit(AppState::Game)));
    }
}

pub fn create_grid(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.single();
    let grid = Grid::new(128.0, window.height(), window.width());

    println!(
        "Grid created with size ({}, {}) in window of size ({}, {})",
        grid.height(),
        grid.width(),
        window.height(),
        window.width()
    );

    commands.insert_resource(grid);
}
