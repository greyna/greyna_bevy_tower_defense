use bevy::prelude::*;

use crate::game::grid::components::*;

use super::{components::*, events::*};

pub fn check_collisions(
    collidables: Query<(Entity, &Transform), With<Collidable>>,
    mut collision_sender: EventWriter<Collision>,
    grid: Res<Grid>,
) {
    let mut collidables_couples = collidables.iter_combinations();
    while let Some([a, b]) = collidables_couples.fetch_next() {
        if a.1.translation.distance(b.1.translation) < grid.cell_size() {
            collision_sender.send(Collision {
                entity_a: a.0,
                entity_b: b.0,
            });
        }
    }
}
