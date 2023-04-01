use super::events::*;
use crate::game::blinking::components::BlinkRequest;
use bevy::prelude::*;

pub fn handle_collisions(mut commands: Commands, mut collisions_receiver: EventReader<Collision>) {
    for collision in collisions_receiver.iter() {
        commands.entity(collision.entity_a).insert(BlinkRequest {});
        commands.entity(collision.entity_b).insert(BlinkRequest {});
    }
}
