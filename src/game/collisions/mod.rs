use crate::game::schedule::GameSet;
use bevy::prelude::*;

use depiction_systems::*;
use events::*;
use logic_systems::*;

pub mod components;
mod depiction_systems;
mod events;
mod logic_systems;

pub struct CollisionsPlugin;

impl Plugin for CollisionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(check_collisions.in_set(GameSet::LogicCollisions))
            .add_system(handle_collisions.after(check_collisions))
            .add_event::<Collision>();
    }
}
