use bevy::prelude::*;

use crate::game::schedule::GameSet;

use logic_systems::build_turret_to_system;

use crate::utils::*;
use components::*;
use depiction_systems::*;

mod components;
mod depiction_systems;
mod events;
mod logic_systems;
mod resources;
mod startup_systems;

pub struct TurretPlugin;

impl Plugin for TurretPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(build_turret_to_system(1.0).in_set(GameSet::LogicAction))
            .add_system(
                turret_spawn_audio
                    .run_if(any_added_component_condition::<Turret>())
                    .in_set(GameSet::Depiction),
            )
            .add_system(
                turret_spawn_sprite
                    .run_if(any_added_component_condition::<Turret>())
                    .in_set(GameSet::Depiction),
            );
    }
}
