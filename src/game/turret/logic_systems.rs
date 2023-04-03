use bevy::prelude::*;

use crate::game::{
    collisions::components::Collidable, components::Target, grid::components::*, utils::*,
};

use super::components::*;

pub fn build_turret_to_system(
    cooldown: f32,
) -> impl FnMut(Commands, Res<Target>, Res<Input<MouseButton>>, Res<Time>, Res<Grid>) {
    let mut cooldown = Cooldown::new(cooldown);
    move |commands, target, input, time, grid| {
        build_turret(commands, &target, &input, &time, &mut cooldown, &grid)
    }
}

fn build_turret(
    mut commands: Commands,
    target: &Target,
    input: &Input<MouseButton>,
    time: &Time,
    cooldown: &mut Cooldown,
    grid: &Grid,
) {
    cooldown.tick(time);

    if cooldown.ready() {
        if let Some(target) = target.pos {
            if input.just_pressed(MouseButton::Left) {
                let target = grid.snap(target);
                commands.spawn((
                    Transform::from_xyz(target.x, target.y, 0.0),
                    Turret {},
                    Collidable {},
                ));
                cooldown.start();
            }
        }
    }
}
