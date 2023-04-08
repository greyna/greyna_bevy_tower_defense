use super::components::*;
use crate::game::{blinking::components::BlinkRequest, grid::components::Grid};
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

                target_shootable.received_shot_power += shooter.attack_power;
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
