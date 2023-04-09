use super::collisions::components::Collidable;
use super::components::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn _spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.single();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, -10.0),
            texture: asset_server.load("sprites/player.png"),
            ..default()
        },
        Player {},
        Collidable {},
    ));
}

pub fn spawn_camera(mut commands: Commands, window_q: Query<&Window, With<PrimaryWindow>>) {
    let window = window_q.single();

    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        },
        MainCamera,
    ));
}

pub fn spawn_target(mut commands: Commands) {
    commands.init_resource::<Target>();
}
