use bevy::prelude::*;

use crate::GameplaySet;

use core_systems::build_turret_to_system;

use crate::utils::*;
use components::*;
use depiction_systems::*;

mod components;
mod core_systems;
mod depiction_systems;
mod events;
mod resources;
mod startup_systems;

pub struct TurretPlugin;

impl Plugin for TurretPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(build_turret_to_system(1.0).in_set(GameplaySet::LogicAction))
            .add_system(
                turret_spawn_audio
                    .run_if(any_added_component_condition::<Turret>())
                    .in_set(GameplaySet::Depiction),
            )
            .add_system(
                turret_spawn_sprite
                    .run_if(any_added_component_condition::<Turret>())
                    .in_set(GameplaySet::Depiction),
            );
    }
}
