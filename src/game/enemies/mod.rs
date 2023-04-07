use super::schedule::*;
use crate::AppState;
use bevy::prelude::*;
use systems::*;

mod components;
mod resources;
mod systems;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_enemies_to_system(0.3).in_set(GameSet::Logic))
            .add_system(move_enemies.in_set(GameSet::LogicMovement))
            .add_system(enemies_out.in_set(GameSet::Depiction))
            .add_system(set_lives.in_schedule(OnEnter(AppState::Game)))
            .add_system(clean_enemies.in_schedule(OnExit(AppState::Game)));
    }
}
