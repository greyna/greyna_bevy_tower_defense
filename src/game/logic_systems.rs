use super::components::*;
use super::grid::components::Grid;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

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

pub fn _move_player(
    window_q: Query<&Window, With<PrimaryWindow>>,
    mut player_q: Query<&mut Transform, With<Player>>,
    target: Res<Target>,
    grid: Res<Grid>,
) {
    let window = window_q.single();

    let target = match target.pos {
        Some(pos) => pos,
        None => Vec2::new(window.width() / 2.0, window.height() / 2.0),
    };

    let target = grid.snap_to_cell_center(target);

    for mut player_transform in &mut player_q {
        player_transform.translation.x = target.x;
        player_transform.translation.y = target.y;
    }
}
