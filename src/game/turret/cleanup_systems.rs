use bevy::prelude::*;

use super::components::*;

pub fn despawn_turrets(mut commands: Commands, turrets: Query<Entity, With<Turret>>) {
    for entity in turrets.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
