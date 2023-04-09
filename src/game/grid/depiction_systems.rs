use bevy::prelude::*;

use super::components::*;

pub fn spawn_terrain(mut commands: Commands, asset_server: Res<AssetServer>, grid: Res<Grid>) {
    let cell_size = grid.cell_size();

    let mut batch = vec![
        (
            SpriteBundle {
                texture: asset_server.load("sprites/desert.png"),
                ..default()
            },
            Terrain {},
        );
        grid.rows_len() * grid.columns_len()
    ];

    for row in 0..grid.rows_len() {
        for column in 0..grid.columns_len() {
            let index = column + (row * grid.columns_len());

            let (SpriteBundle { transform, .. }, ..) = &mut batch[index];
            let Vec2 { x, y } = grid
                .snap_to_cell_center(Vec2::new(cell_size * column as f32, cell_size * row as f32));
            *transform = Transform::from_xyz(x, y, -100.0);

            if row == 0 || row == (grid.rows_len() - 1) {
                let (SpriteBundle { texture, .. }, ..) = &mut batch[index];
                *texture = asset_server.load("sprites/turret_line.png");
            }
        }
    }

    commands.spawn_batch(batch);
}
