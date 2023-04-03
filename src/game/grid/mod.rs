use bevy::prelude::*;

use components::*;

pub mod components;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Grid::new(128.0));
    }
}
