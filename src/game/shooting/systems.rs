use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;

use super::components::*;

pub fn shoot(
    mut shooters: Query<(&mut Transform, &mut Shooter), Without<Shootable>>,
    mut targets: Query<(&Transform, &mut Shootable), Without<Shooter>>,
    time: Res<Time>,
) {
    for mut shooter in shooters.iter_mut() {
        let cd = &mut shooter.1.cooldown;
        cd.tick(&time);

        if cd.ready() {
            if let Some((best_target_pos, mut best_target_shootable, _)) = targets
                .iter_mut()
                .map(|target| {
                    (
                        target.0.translation,
                        target.1,
                        target.0.translation.distance(shooter.0.translation),
                    )
                })
                .min_by(|a, b| a.2.total_cmp(&b.2))
            {
                cd.start();

                best_target_shootable.shot = true;

                let shooter_pos = shooter.0.translation;
                let angle = (best_target_pos - shooter_pos).angle_between(shooter_pos);
                shooter.0.rotation = Quat::from_rotation_z(angle - FRAC_PI_2);
            }
        }
    }
}
