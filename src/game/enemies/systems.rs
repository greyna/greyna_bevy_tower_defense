use super::{components::Enemy, resources::*};
use crate::{
    game::{
        damages::Health, enemies::components::*, grid::components::Grid,
        shooting::components::Shootable, turret::ColorType, utils::Cooldown,
    },
    AppState,
};
use bevy::{prelude::*, time::Stopwatch};
use rand::Rng;

pub fn set_enemies_resources(mut commands: Commands) {
    commands.insert_resource(Lives(20));
    commands.insert_resource(EnemiesSpawnerTimings {
        spawn_cooldown: Cooldown::new(3.0),
        time_since_start: Stopwatch::new(),
    });
}

struct EnemyType {
    sprite: &'static str,
    speed: f32,
    health: f32,
    typpe: ColorType,
}

impl EnemyType {
    const fn new(sprite: &'static str, speed: f32, health: f32, typpe: ColorType) -> Self {
        Self {
            sprite,
            speed,
            health,
            typpe,
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

        const ENEMIES_TYPES: [EnemyType; 3] = [
            EnemyType::new("sprites/enemy_green.png", 255.0, 280.0, ColorType::Green),
            EnemyType::new("sprites/enemy_orange.png", 170.0, 560.0, ColorType::Orange),
            EnemyType::new("sprites/enemy_grey.png", 85.0, 840.0, ColorType::Grey),
        ];

        const ENEMIES_INCREASE_PER_SECOND: f32 = 0.075;
        let nb_enemies_to_spawn: u32 = {
            1 + ((timings.time_since_start.elapsed_secs() * ENEMIES_INCREASE_PER_SECOND).floor()
                as u32)
        };

        for _ in 0..nb_enemies_to_spawn {
            let random_height = rng.gen_range(208.0..(grid.height() - 208.0));
            let random_type = &ENEMIES_TYPES[rng.gen_range(0..3)];
            let unstack_pos_modifier = rng.gen_range(0.8..1.20);
            let random_z = rng.gen_range(-0.99..-0.01);

            commands.spawn((
                Enemy {},
                Movement(random_type.speed * unstack_pos_modifier),
                SpriteBundle {
                    transform: Transform::from_xyz(
                        -32.0 * (2.0 - unstack_pos_modifier),
                        random_height,
                        random_z,
                    ),
                    texture: asset_server.load(random_type.sprite),
                    ..default()
                },
                Shootable::new(random_type.typpe),
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
