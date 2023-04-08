use bevy::prelude::*;

use super::{gold::EnemyKilled, schedule::GameSet, shooting::components::Shootable};

pub struct DamagesPlugin;

impl Plugin for DamagesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (handle_damages, unspawn_dead.after(handle_damages)).in_set(GameSet::Logic),
        );
    }
}

#[derive(Component)]
pub struct Health(pub f32);

pub fn handle_damages(mut damaged: Query<(&mut Health, &Shootable)>) {
    for (mut health, shootable) in damaged.iter_mut() {
        health.0 -= shootable.received_shot_power;
    }
}

pub fn unspawn_dead(
    mut commands: Commands,
    healths: Query<(Entity, &Health)>,
    mut enemy_killed_sender: EventWriter<EnemyKilled>,
) {
    for (entity, health) in healths.iter() {
        if health.0 <= 0.0 {
            enemy_killed_sender.send(EnemyKilled);
            commands.entity(entity).despawn_recursive();
        }
    }
}
