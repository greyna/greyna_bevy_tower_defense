use super::components::*;
use crate::game::blinking::components::BlinkRequest;
use bevy::prelude::*;
use std::f32::consts::FRAC_PI_2;

pub fn shoot(
    mut commands: Commands,
    mut shooters: Query<(&mut Transform, &mut Shooter), Without<Shootable>>,
    mut targets: Query<(Entity, &Transform, &mut Shootable), Without<Shooter>>,
    time: Res<Time>,
) {
    for (_, _, mut target) in targets.iter_mut() {
        target.shot = 0;
    }

    for mut shooter in shooters.iter_mut() {
        let cd = &mut shooter.1.cooldown;
        cd.tick(&time);

        if cd.ready() {
            if let Some((target_entity, target_pos, mut target_shootable, _)) = targets
                .iter_mut()
                .map(|target| {
                    (
                        target.0,
                        target.1.translation,
                        target.2,
                        target.1.translation.distance(shooter.0.translation),
                    )
                })
                .min_by(|a, b| a.3.total_cmp(&b.3))
            {
                cd.start();

                target_shootable.shot += 1;
                commands.entity(target_entity).insert(BlinkRequest {});

                let shooter_pos = shooter.0.translation;
                let angle = (target_pos - shooter_pos).angle_between(shooter_pos);
                shooter.0.rotation = Quat::from_rotation_z(angle - FRAC_PI_2);
            }
        }
    }
}
