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
    mut grid: ResMut<Grid>,
    mut gold: ResMut<Gold>,
) {
    const TURRET_GOLD_COST: u32 = 300;

    if let Some(target) = target.pos {
        if input.just_pressed(MouseButton::Left) {
            let target = grid.snap_to_cell_center(target);

            let zone_allowed_for_turret =
                target.y == grid.bot_row_y() || target.y == grid.top_row_y();
            if zone_allowed_for_turret {
                let turret_free = grid.get_turret(target);
                if !turret_free {
                    if gold.0 >= TURRET_GOLD_COST {
                        gold.0 -= TURRET_GOLD_COST;
                        println!(
                            "Turret cost you {} gold. You have {} gold left.",
                            TURRET_GOLD_COST, gold.0
                        );

                        grid.put_turret(target);
                        commands.spawn((
                            Transform::from_xyz(target.x, target.y, 0.0),
                            Turret {},
                            Collidable {},
                            Shooter::new(0.7),
                        ));
                    } else {
                        println!(
                            "Turret cost is {} gold. You don't have enough.",
                            TURRET_GOLD_COST
                        );
                    }
                } else {
                    println!("Cannot build one turret on another.");
                }
            } else {
                println!("Zone disallowed for building.");
            }
        }
    }
}
