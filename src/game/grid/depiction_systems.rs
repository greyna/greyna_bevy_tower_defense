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
        grid.nb_rows() * grid.nb_columns()
    ];

    for row in 0..grid.nb_rows() {
        for column in 0..grid.nb_columns() {
            let index = column + (row * grid.nb_columns());
            let (SpriteBundle { transform, .. }, ..) = &mut batch[index];
            let Vec2 { x, y } = grid
                .snap_to_cell_center(Vec2::new(cell_size * column as f32, cell_size * row as f32));
            *transform = Transform::from_xyz(x, y, -100.0);
        }
    }

    commands.spawn_batch(batch);
}
