use super::{
    components::Enemy,
    resources::{EnemiesSpawnerTimings, Lives},
};
use crate::{
    game::{
        damages::Health, grid::components::Grid, shooting::components::Shootable, utils::Cooldown,
    },
    AppState,
};
use bevy::{prelude::*, time::Stopwatch};
use rand::Rng;

pub fn set_enemies_resources(mut commands: Commands) {
    commands.insert_resource(Lives(10));
    commands.insert_resource(EnemiesSpawnerTimings {
        spawn_cooldown: Cooldown::new(3.0),
        time_since_start: Stopwatch::new(),
    });
}

pub fn spawn_enemies(
    mut commands: Commands,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    grid: Res<Grid>,
    mut timings: ResMut<EnemiesSpawnerTimings>,
) {
    timings.spawn_cooldown.tick(&time);
    timings.time_since_start.tick(time.delta());

    if timings.spawn_cooldown.ready() {
        let mut rng = rand::thread_rng();

        static ENEMIES_SPRITES: [&str; 3] = [
            "sprites/enemy_green.png",
            "sprites/enemy_grey.png",
            "sprites/enemy_orange.png",
        ];

        const ENEMIES_INCREASE_PER_SECOND: f32 = 0.15;
        let nb_enemies_to_spawn: u32 = {
            1 + ((timings.time_since_start.elapsed_secs() * ENEMIES_INCREASE_PER_SECOND).floor()
                as u32)
        };

        for _ in 0..nb_enemies_to_spawn {
            let random_height = rng.gen_range(200.0..(grid.height() - 40.0));
            let random_sprite = ENEMIES_SPRITES[rng.gen_range(0..3)];

            commands.spawn((
                Enemy {},
                SpriteBundle {
                    transform: Transform::from_xyz(-64.0, random_height, 0.0),
                    texture: asset_server.load(random_sprite),
                    ..default()
                },
                Shootable::default(),
                Health(3.0),
            ));
        }

        timings.spawn_cooldown.start();
    }
}

pub fn move_enemies(time: Res<Time>, mut enemies: Query<&mut Transform, With<Enemy>>) {
    for mut enemy_pos in enemies.iter_mut() {
        let pos = &mut enemy_pos.translation;
        const ENEMIES_SPEED: f32 = 200.0;
        pos.x += ENEMIES_SPEED * time.delta_seconds();
    }
}

pub fn enemies_out(
    mut commands: Commands,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
    mut lives: ResMut<Lives>,
    grid: Res<Grid>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (enemy_entity, enemy_transform) in enemies.iter() {
        if enemy_transform.translation.x >= grid.width() {
            commands.entity(enemy_entity).despawn();

            lives.0 -= 1;
            println!("Enemy out, life lost! Lives = {}", lives.0);

            if lives.0 == 0 {
                println!("Game Over !");
                next_state.set(AppState::Menu);
                break;
            }
        }
    }
}

pub fn clean_enemies(mut commands: Commands, enemies: Query<Entity, With<Enemy>>) {
    for enemy_entity in enemies.iter() {
        commands.entity(enemy_entity).despawn_recursive();
    }
}
