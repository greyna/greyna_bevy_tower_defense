use super::components::*;
use crate::game::{blinking::components::BlinkRequest, grid::components::Grid, turret::*};
use bevy::prelude::*;

pub fn shoot(
    mut commands: Commands,
    mut shooters: Query<(&mut Transform, &mut Shooter), Without<Shootable>>,
    mut targets: Query<(Entity, &Transform, &mut Shootable), Without<Shooter>>,
    time: Res<Time>,
    grid: Res<Grid>,
) {
    for (_, _, mut target) in targets.iter_mut() {
        target.received_shot_power = 0.0;
    }

    for (mut shooter_transform, mut shooter) in shooters.iter_mut() {
        let attack_cooldown = &mut shooter.attack_cooldown;
        attack_cooldown.tick(&time);

        if attack_cooldown.ready() {
            if let Some((target_entity, target_pos, mut target_shootable, _)) = targets
                .iter_mut()
                .filter(|target| {
                    target.1.translation.x > 20.0 && target.1.translation.x < grid.width()
                })
                .map(|target| {
                    (
                        target.0,
                        target.1.translation,
                        target.2,
                        target.1.translation.distance(shooter_transform.translation),
                    )
                })
                .min_by(|a, b| a.3.total_cmp(&b.3))
            {
                attack_cooldown.start();

                target_shootable.received_shot_power +=
                    receive_shot_power(&shooter, &target_shootable);

                commands.entity(target_entity).insert(BlinkRequest {});

                let mut shooting_direction = target_pos - shooter_transform.translation;
                shooting_direction.z = 0.0;
                if shooting_direction != Vec3::ZERO {
                    let angle = Vec3::Y.angle_between(shooting_direction);
                    shooter_transform.rotation = Quat::from_rotation_z(angle);
                }
            }
        }
    }
}

fn receive_shot_power(shooter: &Shooter, shootable: &Shootable) -> f32 {
    const RIGHT_TYPE_MODIFIER: f32 = 2.0;
    const WRONG_TYPE_MODIFIER: f32 = 0.5;

    let received_green = shooter.attack_power_green
        * if shootable.typpe == ColorType::Green {
            RIGHT_TYPE_MODIFIER
        } else {
            WRONG_TYPE_MODIFIER
        };

    let received_orange = shooter.attack_power_orange
        * if shootable.typpe == ColorType::Orange {
            RIGHT_TYPE_MODIFIER
        } else {
            WRONG_TYPE_MODIFIER
        };

    let received_grey = shooter.attack_power_grey
        * if shootable.typpe == ColorType::Grey {
            RIGHT_TYPE_MODIFIER
        } else {
            WRONG_TYPE_MODIFIER
        };

    received_green + received_orange + received_grey
}

// Bevy Jam #3 -> SIDE EFFECT APPLICATION :)
pub fn update_attack_power(
    mut shooters: Query<(Entity, &mut Shooter)>,
    turrets: Query<&Turret>,
    grid: Res<Grid>,
) {
    for (_, mut shooter) in shooters.iter_mut() {
        shooter.attack_power_green = 0.0;
        shooter.attack_power_orange = 0.0;
        shooter.attack_power_grey = 0.0;
    }

    for (entity, mut shooter) in shooters.iter_mut() {
        let turret = turrets.get(entity).unwrap();
        let typpe = turret.main_type;
        let power = turret.attack_power;

        shooter.add_attack_power(power, typpe);

        let (left_turret, right_turret) = grid.get_side_turrets(entity);

        if let Some(left_turret) = left_turret {
            if let Ok(left_turret) = turrets.get(left_turret) {
                shooter.add_attack_power(left_turret.attack_power, left_turret.main_type);
            }
        }

        if let Some(right_turret) = right_turret {
            if let Ok(right_turret) = turrets.get(right_turret) {
                shooter.add_attack_power(right_turret.attack_power, right_turret.main_type);
            }
        }
    }
}
