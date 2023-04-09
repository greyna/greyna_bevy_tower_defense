use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::{egui::Align2, EguiContexts};

use crate::{
    game::{
        collisions::components::*, components::*, gold::Gold, grid::components::*,
        shooting::components::*,
    },
    ui::my_window,
};

use super::components::*;

const TURRET_GOLD_COST: u32 = 300;

pub fn build_turret(
    mut commands: Commands,
    target: Res<Target>,
    key_input: Res<Input<KeyCode>>,
    mut grid: ResMut<Grid>,
    mut gold: ResMut<Gold>,
) {
    let e_key = key_input.just_pressed(KeyCode::E);
    let r_key = key_input.just_pressed(KeyCode::R);
    let t_key = key_input.just_pressed(KeyCode::T);

    if let Some(target) = target.pos {
        if e_key || r_key || t_key {
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

                        let color_type = if e_key {
                            attack_power_green = BASE_POWER;
                            ColorType::Green
                        } else if r_key {
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
                } else {
                    println!("Turret already built here.");
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

            let zone_allowed_for_turret =
                target.y == grid.bot_row_y() || target.y == grid.top_row_y();
            if zone_allowed_for_turret {
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
                                ColorType::Green => {
                                    shooter.attack_power_green = turret.attack_power
                                }
                                ColorType::Orange => {
                                    shooter.attack_power_orange = turret.attack_power
                                }
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
                } else {
                    println!("No turret to upgrade here.");
                }
            } else {
                println!("Zone disallowed for upgrading.");
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

pub fn ui(
    target: Res<Target>,
    turrets: Query<(&Transform, &Turret, &Shooter)>,
    grid: Res<Grid>,
    mut contexts: EguiContexts,
    main_window: Query<&Window, With<PrimaryWindow>>,
) {
    my_window(
        &format!("Build Turrets ({} gold)", TURRET_GOLD_COST),
        Vec2::default(),
        &main_window,
    )
    .title_bar(true)
    .anchor(Align2::RIGHT_TOP, [-10.0, 10.0])
    .collapsible(true)
    .show(contexts.ctx_mut(), |ui| {
        ui.label("Hover your mouse over top/bottom row then:");
        ui.label(" - Press 'e' for a green one, strong against green enemies");
        ui.label(" - Press 'r' for an orange one, strong against orange enemies");
        ui.label(" - Press 't' for a grey one, strong against grey enemies");
    });

    my_window("Side Effect", Vec2::default(), &main_window)
        .anchor(Align2::LEFT_TOP, [10.0, 10.0])
        .title_bar(true)
        .collapsible(true)
        .show(contexts.ctx_mut(), |ui| {
            ui.label("Turrets share their power!");
            ui.label("Left and right side turrets benefit.");
            ui.label("The power share keeps its source color.");
        });

    if let Some(target) = target.pos {
        let target = grid.snap_to_cell_center(target);
        if let Some((_, turret, shooter)) = turrets
            .iter()
            .filter(|(transform, ..)| {
                target == Vec2::new(transform.translation.x, transform.translation.y)
            })
            .next()
        {
            my_window("Turret tooltip", target, &main_window).show(contexts.ctx_mut(), |ui| {
                ui.heading(format!("LEVEL {}", turret.level));
                ui.label(format!("Upgrade: {}g", upgrade_gold_cost(turret.level)));
                ui.heading("POWER");
                ui.label(format!("Green {}", shooter.attack_power_green));
                ui.label(format!("Orange {}", shooter.attack_power_orange));
                ui.label(format!("Grey {}", shooter.attack_power_grey));
            });
        }
    }
}
