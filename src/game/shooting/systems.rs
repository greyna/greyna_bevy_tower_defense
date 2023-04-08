use super::components::*;
use crate::game::{blinking::components::BlinkRequest, grid::components::Grid, turret::ColorType};
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
