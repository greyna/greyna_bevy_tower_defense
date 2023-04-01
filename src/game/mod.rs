use bevy::prelude::*;

use schedule::GameSet;

use self::logic_systems::*;
use self::startup_systems::*;

pub mod components;
mod depiction_systems;
mod events;
mod logic_systems;
mod resources;
pub mod schedule;
mod startup_systems;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_systems((spawn_player, spawn_camera, spawn_target))
            .add_system(target_cursor.in_set(GameSet::Input))
            .add_system(move_player.in_set(GameSet::LogicMovement));
    }
}
