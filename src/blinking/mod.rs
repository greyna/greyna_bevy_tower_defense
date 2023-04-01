use bevy::prelude::*;

use crate::GameplaySet;
use depiction_systems::*;

pub mod components;
mod depiction_systems;
mod events;
mod logic_systems;
mod resources;
mod startup_systems;

pub struct BlinkingPlugin;

impl Plugin for BlinkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (first_blink.before(blink), handle_blink_requests, blink)
                .in_set(GameplaySet::Depiction),
        );
    }
}
