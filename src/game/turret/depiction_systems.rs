use bevy::prelude::*;

use super::components::Turret;

pub fn turret_spawn_sprite(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    new_turrets: Query<(Entity, &Transform), Added<Turret>>,
) {
    for (turret_entity, turret_transform) in &new_turrets {
        commands.entity(turret_entity).insert(SpriteBundle {
            transform: *turret_transform,
            texture: asset_server.load("sprites/turret.png"),
            ..default()
        });
    }
}

pub fn turret_spawn_audio(audio: Res<Audio>, asset_server: Res<AssetServer>) {
    let sfx = asset_server.load("audio/turret_creation.ogg");
    audio.play(sfx);
}
