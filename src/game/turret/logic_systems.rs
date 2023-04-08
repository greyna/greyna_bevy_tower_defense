use bevy::prelude::*;

use crate::game::{
    collisions::components::*, components::*, gold::Gold, grid::components::*,
    shooting::components::*,
};

use super::components::*;

const BASE_TURRET_ATTACK_COOLDOWN: f32 = 0.7;

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
                let turret_free = grid.get_turret(target).is_none();
                if turret_free {
                    if gold.0 >= TURRET_GOLD_COST {
                        gold.0 -= TURRET_GOLD_COST;
                        println!(
                            "Turret cost you {} gold. You have {} gold left.",
                            TURRET_GOLD_COST, gold.0
                        );

                        let turret_entity = commands
                            .spawn((
                                Transform::from_xyz(target.x, target.y, 0.0),
                                Turret::default(),
                                Collidable {},
                                Shooter::new(BASE_TURRET_ATTACK_COOLDOWN),
                            ))
                            .id();
                        grid.put_turret(target, turret_entity);
                    } else {
                        println!(
                            "Turret cost is {} gold. You don't have enough.",
                            TURRET_GOLD_COST
                        );
                    }
                }
            } else {
                println!("Zone disallowed for building.");
            }
        }
    }
}

pub fn upgrade_turret(
    mut turrets: Query<(&mut Turret, &mut Shooter)>,
    target: Res<Target>,
    input: Res<Input<MouseButton>>,
    grid: ResMut<Grid>,
    mut gold: ResMut<Gold>,
) {
    const UPGRADE_GOLD_COST: u32 = 300;

    if let Some(target) = target.pos {
        if input.just_pressed(MouseButton::Left) {
            let target = grid.snap_to_cell_center(target);

            let turret_entity = grid.get_turret(target);
            if let Some(turret_entity) = turret_entity {
                if gold.0 >= UPGRADE_GOLD_COST {
                    gold.0 -= UPGRADE_GOLD_COST;
                    let (mut turret, mut shooter) = turrets.get_mut(turret_entity).unwrap();
                    turret.level += 1;
                    shooter.set_attack_cooldown(BASE_TURRET_ATTACK_COOLDOWN / turret.level as f32);

                    println!(
                        "Upgraded turret from level {} to {} for {} gold. You have {} gold left.",
                        turret.level - 1,
                        turret.level,
                        UPGRADE_GOLD_COST,
                        gold.0
                    );
                } else {
                    println!(
                        "Upgrade cost is {} gold. You don't have enough.",
                        UPGRADE_GOLD_COST
                    );
                }
            }
        }
    }
}
