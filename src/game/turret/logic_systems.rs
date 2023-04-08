use bevy::prelude::*;

use crate::game::{
    collisions::components::*, components::*, gold::Gold, grid::components::*,
    shooting::components::*,
};

use super::components::*;

pub fn build_turret(
    mut commands: Commands,
    target: Res<Target>,
    key_input: Res<Input<KeyCode>>,
    mut grid: ResMut<Grid>,
    mut gold: ResMut<Gold>,
) {
    const TURRET_GOLD_COST: u32 = 300;

    let a_key = key_input.just_pressed(KeyCode::A);
    let z_key = key_input.just_pressed(KeyCode::Z);
    let e_key = key_input.just_pressed(KeyCode::E);

    if let Some(target) = target.pos {
        if a_key || z_key || e_key {
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

                        let mut attack_power_green = 0.0;
                        let mut attack_power_orange = 0.0;
                        let mut attack_power_grey = 0.0;

                        let color_type = if a_key {
                            attack_power_green = BASE_POWER;
                            ColorType::Green
                        } else if z_key {
                            attack_power_orange = BASE_POWER;
                            ColorType::Orange
                        } else {
                            attack_power_grey = BASE_POWER;
                            ColorType::Grey
                        };

                        let turret_entity = commands
                            .spawn((
                                Transform::from_xyz(target.x, target.y, 0.0),
                                Turret::new(BASE_POWER, color_type),
                                Collidable {},
                                Shooter::new(
                                    BASE_TURRET_ATTACK_COOLDOWN,
                                    attack_power_green,
                                    attack_power_orange,
                                    attack_power_grey,
                                ),
                            ))
                            .id();
                        grid.put_turret(target, turret_entity);
                    } else {
                        println!(
                            "Turret cost is {} gold. You only have {}.",
                            TURRET_GOLD_COST, gold.0
                        );
                    }
                }
            } else {
                println!("Zone disallowed for building.");
            }
        }
    }
}

const BASE_UPGRADE_GOLD_COST: u32 = 400;
const BASE_TURRET_ATTACK_COOLDOWN: f32 = 0.4;
const BASE_POWER: f32 = 20.0;
const MAX_LEVEL: u32 = 9;

pub fn upgrade_turret(
    mut turrets: Query<(&mut Turret, &mut Shooter)>,
    target: Res<Target>,
    input: Res<Input<MouseButton>>,
    grid: ResMut<Grid>,
    mut gold: ResMut<Gold>,
) {
    if let Some(target) = target.pos {
        if input.just_pressed(MouseButton::Left) {
            let target = grid.snap_to_cell_center(target);

            let turret_entity = grid.get_turret(target);
            if let Some(turret_entity) = turret_entity {
                let (mut turret, mut shooter) = turrets.get_mut(turret_entity).unwrap();

                if turret.level < MAX_LEVEL {
                    let cost = upgrade_gold_cost(turret.level);

                    if gold.0 >= cost {
                        gold.0 -= cost;

                        turret.level += 1;

                        let old_power = turret.attack_power;
                        turret.attack_power = upgrade_power(turret.level);

                        match turret.main_type {
                            ColorType::Green => shooter.attack_power_green = turret.attack_power,
                            ColorType::Orange => shooter.attack_power_orange = turret.attack_power,
                            ColorType::Grey => shooter.attack_power_grey = turret.attack_power,
                        }

                        println!(
                        "Upgraded turret from level {} to {} (power from {} to {}) for {} gold. You have {} gold left.",
                        turret.level - 1,
                        turret.level,
                        old_power,
                        turret.attack_power,
                        cost,
                        gold.0
                    );
                    } else {
                        println!("Upgrade cost is {} gold. You only have {}.", cost, gold.0);
                    }
                } else {
                    println!(
                        "Turret is already at level {} on max {}.",
                        turret.level, MAX_LEVEL
                    );
                }
            }
        }
    }
}

fn upgrade_gold_cost(level: u32) -> u32 {
    match level {
        0 | 1 => BASE_UPGRADE_GOLD_COST,
        // cost increase of 50% per level
        _ => upgrade_gold_cost(level - 1) * 3 / 2,
    }
}

fn upgrade_power_per_gold(level: u32) -> f32 {
    match level {
        0 | 1 => BASE_POWER / BASE_UPGRADE_GOLD_COST as f32,
        // power per cost increase of 10% per level
        _ => upgrade_power_per_gold(level - 1) * 1.1,
    }
}

fn upgrade_power(level: u32) -> f32 {
    match level {
        0 | 1 => BASE_POWER,
        _ => (upgrade_power(level - 1)
            + (upgrade_gold_cost(level) as f32 * upgrade_power_per_gold(level)))
        .round(),
    }
}
