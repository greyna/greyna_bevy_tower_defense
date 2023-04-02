use bevy::prelude::*;

use super::components::*;

pub fn despawn_player(mut commands: Commands, players: Query<Entity, With<Player>>) {
    for entity in players.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn despawn_camera(mut commands: Commands, camera: Query<Entity, With<MainCamera>>) {
    commands.entity(camera.single()).despawn();
}

pub fn despawn_target(mut commands: Commands) {
    commands.remove_resource::<Target>();
}
