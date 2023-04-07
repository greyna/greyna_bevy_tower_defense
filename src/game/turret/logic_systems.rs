use bevy::prelude::*;

use crate::game::{
    collisions::components::*, components::*, gold::Gold, grid::components::*,
    shooting::components::*,
};

use super::components::*;

pub fn build_turret(
    mut commands: Commands,
    target: Res<Target>,
    input: Res<Input<MouseButton>>,
    grid: Res<Grid>,
    mut gold: ResMut<Gold>,
) {
    const TURRET_GOLD_COST: u32 = 300;

    if gold.0 >= TURRET_GOLD_COST && input.just_pressed(MouseButton::Left) {
        if let Some(target) = target.pos {
            gold.0 -= TURRET_GOLD_COST;
            println!(
                "Turret cost you {} gold. You have {} gold left.",
                TURRET_GOLD_COST, gold.0
            );

            let target = grid.snap(target);
            commands.spawn((
                Transform::from_xyz(target.x, target.y, 0.0),
                Turret {},
                Collidable {},
                Shooter::new(0.7),
            ));
        }
    }
}
