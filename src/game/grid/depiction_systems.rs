use bevy::{prelude::*, window::PrimaryWindow};

use super::components::*;

pub fn spawn_terrain(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    grid: Res<Grid>,
) {
    let window = window_query.single();
    let cell_size = grid.cell_size();

    let lines_count = (window.height() / cell_size).ceil() as usize;
    let rows_count = (window.width() / cell_size).ceil() as usize;

    let mut batch = vec![
        (
            SpriteBundle {
                texture: asset_server.load("sprites/desert.png"),
                ..default()
            },
            Terrain {},
        );
        lines_count * rows_count
    ];

    for line in 0..lines_count {
        for row in 0..rows_count {
            let index = row + (line * rows_count);
            let (SpriteBundle { transform, .. }, ..) = &mut batch[index];
            let Vec2 { x, y } =
                grid.snap(Vec2::new(cell_size * row as f32, cell_size * line as f32));
            *transform = Transform::from_xyz(x, y, -100.0);
        }
    }

    commands.spawn_batch(batch);
}
