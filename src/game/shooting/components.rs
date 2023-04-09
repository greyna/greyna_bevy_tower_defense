use bevy::prelude::*;

use crate::game::{turret::ColorType, utils::Cooldown};

#[derive(Component)]
pub struct Shootable {
    pub received_shot_power: f32,
    pub typpe: ColorType,
}

impl Shootable {
    pub fn new(typpe: ColorType) -> Self {
        Self {
            received_shot_power: 0.0,
            typpe,
        }
    }
}

#[derive(Component)]
pub struct Shooter {
    pub attack_cooldown: Cooldown,
    pub attack_power_green: f32,
    pub attack_power_orange: f32,
    pub attack_power_grey: f32,
}

impl Shooter {
    pub fn new(
        attack_cooldown: f32,
        attack_power_green: f32,
        attack_power_orange: f32,
        attack_power_grey: f32,
    ) -> Self {
        Self {
            attack_cooldown: Cooldown::new(attack_cooldown),
            attack_power_green,
            attack_power_orange,
            attack_power_grey,
        }
    }

    pub fn add_attack_power(&mut self, power: f32, typpe: ColorType) {
        match typpe {
            ColorType::Green => self.attack_power_green += power,
            ColorType::Orange => self.attack_power_orange += power,
            ColorType::Grey => self.attack_power_grey += power,
        }
    }
}
