use super::schedule::GameSet;
use bevy::prelude::*;
use systems::*;

pub mod components;
mod systems;

pub struct ShootingPlugin;

impl Plugin for ShootingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(shoot.in_set(GameSet::Logic));
    }
}
