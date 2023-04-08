use bevy::prelude::*;

use crate::game::utils::Cooldown;

#[derive(Component, Default)]
pub struct Shootable {
    pub received_shot_power: f32,
}

#[derive(Component)]
pub struct Shooter {
    pub attack_cooldown: Cooldown,
    pub attack_power: f32,
}

impl Shooter {
    pub fn new(attack_cooldown: f32, attack_power: f32) -> Self {
        Self {
            attack_cooldown: Cooldown::new(attack_cooldown),
            attack_power,
        }
    }
}
