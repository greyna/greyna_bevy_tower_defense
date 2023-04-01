use crate::game::schedule::GameSet;
use bevy::prelude::*;
use depiction_systems::*;

pub mod components;
mod depiction_systems;

pub struct BlinkingPlugin;

impl Plugin for BlinkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (first_blink.before(blink), handle_blink_requests, blink).in_set(GameSet::Depiction),
        );
    }
}
