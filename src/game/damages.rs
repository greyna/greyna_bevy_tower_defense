use bevy::prelude::*;

use super::{schedule::GameSet, shooting::components::Shootable};

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
        const DAMAGE_PER_SHOT: f32 = 2.0;
        health.0 -= shootable.shot as f32 * DAMAGE_PER_SHOT;
    }
}

pub fn unspawn_dead(mut commands: Commands, healths: Query<(Entity, &Health)>) {
    for (entity, health) in healths.iter() {
        if health.0 <= 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}
