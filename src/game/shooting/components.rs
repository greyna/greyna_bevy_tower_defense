use bevy::prelude::*;

use crate::game::utils::Cooldown;

#[derive(Component, Default)]
pub struct Shootable {
    pub shot: u8,
}

#[derive(Component)]
pub struct Shooter {
    pub cooldown: Cooldown,
}

impl Shooter {
    pub fn new(attack_cooldown: f32) -> Self {
        Self {
            cooldown: Cooldown::new(attack_cooldown),
        }
    }
}
