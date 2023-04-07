use super::{components::Enemy, resources::*};
use crate::{
    game::{
        damages::Health, enemies::components::*, grid::components::Grid,
        shooting::components::Shootable, utils::Cooldown,
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

struct EnemyType {
    sprite: &'static str,
    speed: f32,
    health: f32,
}

impl EnemyType {
    const fn new(sprite: &'static str, speed: f32, health: f32) -> Self {
        Self {
            sprite,
            speed,
            health,
        }
    }
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

        static ENEMIES_TYPES: [EnemyType; 3] = [
            EnemyType::new("sprites/enemy_green.png", 300.0, 4.0),
            EnemyType::new("sprites/enemy_grey.png", 100.0, 12.0),
            EnemyType::new("sprites/enemy_orange.png", 200.0, 6.0),
        ];

        const ENEMIES_INCREASE_PER_SECOND: f32 = 0.15;
        let nb_enemies_to_spawn: u32 = {
            1 + ((timings.time_since_start.elapsed_secs() * ENEMIES_INCREASE_PER_SECOND).floor()
                as u32)
        };

        for _ in 0..nb_enemies_to_spawn {
            let random_height = rng.gen_range(208.0..(grid.height() - 208.0));
            let random_type = &ENEMIES_TYPES[rng.gen_range(0..3)];
            let random_speed_modifier = rng.gen_range(0.95..1.05);

            commands.spawn((
                Enemy {},
                Movement(random_type.speed * random_speed_modifier),
                SpriteBundle {
                    transform: Transform::from_xyz(-32.0, random_height, 0.0),
                    texture: asset_server.load(random_type.sprite),
                    ..default()
                },
                Shootable::default(),
                Health(random_type.health),
            ));
        }

        timings.spawn_cooldown.start();
    }
}

pub fn move_enemies(time: Res<Time>, mut enemies: Query<(&mut Transform, &Movement), With<Enemy>>) {
    for (mut enemy_pos, Movement(enemy_speed)) in enemies.iter_mut() {
        let pos = &mut enemy_pos.translation;
        pos.x += enemy_speed * time.delta_seconds();
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
        if enemy_transform.translation.x >= (grid.width() + 32.0) {
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
