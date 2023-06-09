use bevy::prelude::*;

use super::components::Turret;

pub fn turret_spawn_sprite(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    new_turrets: Query<(Entity, &Transform, &Turret), Added<Turret>>,
) {
    for (turret_entity, turret_transform, turret) in &new_turrets {
        commands
            .entity(turret_entity)
            .insert(SpatialBundle::from_transform(*turret_transform));

        let sprite_type = match turret.main_type {
            super::ColorType::Green => "sprites/turret_type_green.png",
            super::ColorType::Orange => "sprites/turret_type_orange.png",
            super::ColorType::Grey => "sprites/turret_type_grey.png",
        };

        commands
            .spawn(SpriteBundle {
                transform: Transform::from_xyz(0.0, 0.0, -1.0),
                texture: asset_server.load(sprite_type),
                ..default()
            })
            .set_parent(turret_entity);

        commands
            .spawn(SpriteBundle {
                transform: Transform::from_xyz(0.0, 40.0, 0.0),
                texture: asset_server.load("sprites/turret.png"),
                ..default()
            })
            .set_parent(turret_entity);
    }
}

pub fn turret_spawn_audio(audio: Res<Audio>, asset_server: Res<AssetServer>) {
    let sfx = asset_server.load("audio/turret_creation.ogg");
    audio.play(sfx);
}
