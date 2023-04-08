use bevy::prelude::*;

#[derive(Component)]
pub struct Turret {
    pub level: u32,
}

impl Default for Turret {
    fn default() -> Self {
        Self { level: 1 }
    }
}
