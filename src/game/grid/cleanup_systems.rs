use bevy::prelude::*;

use super::components::*;

pub fn despawn_terrain(mut commands: Commands, terrain: Query<Entity, With<Terrain>>) {
    for entity in terrain.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
