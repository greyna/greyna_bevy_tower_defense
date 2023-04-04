use super::schedule::*;
use bevy::prelude::*;
use systems::*;

mod components;
mod resources;
mod systems;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((spawn_enemies_to_system(0.3), move_enemies).in_set(GameSet::Logic));
    }
}
