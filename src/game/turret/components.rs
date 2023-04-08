use bevy::prelude::*;

#[derive(Component)]
pub struct Turret {
    pub level: u8,
}

impl Default for Turret {
    fn default() -> Self {
        Self { level: 1 }
    }
}
