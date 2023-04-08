use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum ColorType {
    Green,
    Orange,
    Grey,
}

#[derive(Component)]
pub struct Turret {
    pub level: u32,
    pub attack_power: f32,
    pub main_type: ColorType,
}

impl Turret {
    pub fn new(attack_power: f32, main_type: ColorType) -> Self {
        Self {
            level: 1,
            attack_power,
            main_type,
        }
    }
}
