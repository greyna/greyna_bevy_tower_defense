use bevy::prelude::*;

use crate::{collisions::components::Collidable, game::components::Target, utils::*};

use super::components::Turret;

pub fn build_turret_to_system(
    cooldown: f32,
) -> impl FnMut(Commands, Res<Target>, Res<Input<MouseButton>>, Res<Time>) {
    let mut cooldown = Cooldown::new(cooldown);
    move |commands, target, input, time| {
        build_turret(commands, &target, &input, &time, &mut cooldown)
    }
}

fn build_turret(
    mut commands: Commands,
    target: &Target,
    input: &Input<MouseButton>,
    time: &Time,
    cooldown: &mut Cooldown,
) {
    cooldown.tick(time);

    if cooldown.ready() {
        if let Some(target) = target.pos {
            if input.just_pressed(MouseButton::Left) {
                commands.spawn((
                    Transform::from_xyz(target.x + 70.0, target.y, 0.0),
                    Turret {},
                    Collidable {},
                ));
                cooldown.start();
            }
        }
    }
}
