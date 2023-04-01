mod blinking;
mod collisions;
mod schedule;
mod turret;
mod utils;

use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use blinking::BlinkingPlugin;
use collisions::components::Collidable;
use collisions::CollisionsPlugin;
use schedule::GameplaySet;
use turret::TurretPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(TurretPlugin)
        .add_plugin(BlinkingPlugin)
        .add_plugin(CollisionsPlugin)
        .add_startup_systems((spawn_player, spawn_camera, spawn_target))
        .add_system(exit_game)
        .add_system(target_cursor.in_set(GameplaySet::Input))
        .add_system(move_player.in_set(GameplaySet::LogicMovement))
        .run();
}

#[derive(Component)]
pub struct Player {}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.single();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/player.png"),
            ..default()
        },
        Player {},
        Collidable {},
    ));
}

#[derive(Component)]
pub struct MainCamera;

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

#[derive(Resource, Default)]
pub struct Target {
    pos: Option<Vec2>,
}

pub fn spawn_target(mut commands: Commands) {
    commands.init_resource::<Target>();
}

pub fn target_cursor(
    window_q: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut target: ResMut<Target>,
) {
    let (camera, camera_transform) = camera_q.single();
    let window = window_q.single();

    target.pos = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate());
}

pub fn move_player(
    window_q: Query<&Window, With<PrimaryWindow>>,
    mut player_q: Query<&mut Transform, With<Player>>,
    target: Res<Target>,
) {
    let window = window_q.single();

    let target = match target.pos {
        Some(pos) => pos,
        None => Vec2::new(window.width() / 2.0, window.height() / 2.0),
    };

    for mut player_transform in &mut player_q {
        player_transform.translation.x = target.x;
        player_transform.translation.y = target.y;
    }
}

pub fn exit_game(input: Res<Input<KeyCode>>, mut app_exit_sender: EventWriter<AppExit>) {
    if input.just_pressed(KeyCode::Escape) {
        app_exit_sender.send(AppExit);
    }
}
