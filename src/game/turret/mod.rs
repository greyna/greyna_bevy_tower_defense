mod cleanup_systems;
mod components;
mod depiction_systems;
mod logic_systems;

use bevy::prelude::*;

use crate::game::schedule::GameSet;
use crate::game::utils::*;
use crate::AppState;

use cleanup_systems::*;
pub use components::*;
use depiction_systems::*;
use logic_systems::*;

pub struct TurretPlugin;

impl Plugin for TurretPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (build_turret, upgrade_turret.before(build_turret)).in_set(GameSet::LogicAction),
        )
        .add_system(despawn_turrets.in_schedule(OnExit(AppState::Game)))
        .add_system(ui.in_set(GameSet::Depiction))
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
