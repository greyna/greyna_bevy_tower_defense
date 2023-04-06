use super::schedule::*;
use bevy::prelude::*;
use resources::*;
use systems::*;

mod components;
mod resources;
mod systems;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Lives(10))
            .add_system(spawn_enemies_to_system(0.3).in_set(GameSet::Logic))
            .add_system(move_enemies.in_set(GameSet::LogicMovement))
            .add_system(enemies_out.in_set(GameSet::Depiction));
    }
}
