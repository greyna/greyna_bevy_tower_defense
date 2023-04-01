use bevy::prelude::*;

use super::components::*;

pub fn handle_blink_requests(
    mut commands: Commands,
    mut requests: Query<(Entity, Option<&mut Blinking>), With<BlinkRequest>>,
) {
    for (entity, blinking) in requests.iter_mut() {
        match blinking {
            Some(mut b) => b.reset_expiration(),
            None => drop(commands.entity(entity).insert(Blinking::default())),
        };
        commands.entity(entity).remove::<BlinkRequest>();
    }
}

pub fn first_blink(mut blinkers: Query<(&mut Visibility, &mut Blinking), Added<Blinking>>) {
    for (mut vis, mut blink) in blinkers.iter_mut() {
        blink.init_first_visibility(*vis);
        *vis = blink.opposite_first_visibility();
    }
}

pub fn blink(
    mut commands: Commands,
    time: Res<Time>,
    mut blinkers: Query<(Entity, &mut Visibility, &mut Blinking)>,
) {
    for (entity, mut vis, mut blink) in blinkers.iter_mut() {
        blink.tick(time.delta());

        if blink.expired() {
            *vis = blink.first_visibility();
            commands.entity(entity).remove::<Blinking>();
        } else if blink.toggling_visibility() {
            *vis = if *vis == blink.first_visibility() {
                blink.opposite_first_visibility()
            } else {
                blink.first_visibility()
            };
        }
    }
}
