use super::schedule::GameSet;
use bevy::prelude::*;
use systems::*;

pub mod components;
mod systems;

pub struct ShootingPlugin;

impl Plugin for ShootingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (update_attack_power, shoot.after(update_attack_power)).in_set(GameSet::Logic),
        );
    }
}
