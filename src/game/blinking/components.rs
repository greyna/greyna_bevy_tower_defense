use bevy::prelude::*;
use std::time::Duration;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct BlinkRequest {}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Blinking {
    blink_timer: Timer,
    expiration_timer: Timer,
    first_visibility: Visibility,
}

impl Default for Blinking {
    fn default() -> Self {
        Self {
            blink_timer: Timer::from_seconds(0.08, TimerMode::Repeating),
            expiration_timer: Timer::from_seconds(0.4, TimerMode::Once),
            first_visibility: Default::default(),
        }
    }
}

impl Blinking {
    pub fn init_first_visibility(&mut self, visibility: Visibility) {
        self.first_visibility = visibility;
    }

    pub fn tick(&mut self, delta: Duration) {
        self.blink_timer.tick(delta);
        self.expiration_timer.tick(delta);
    }

    pub fn toggling_visibility(&self) -> bool {
        self.blink_timer.finished()
    }

    pub fn expired(&self) -> bool {
        self.expiration_timer.finished()
    }

    pub fn reset_expiration(&mut self) {
        self.expiration_timer.reset();
    }

    pub fn opposite_first_visibility(&self) -> Visibility {
        match self.first_visibility {
            Visibility::Inherited => Visibility::Hidden,
            Visibility::Hidden => Visibility::Inherited,
            Visibility::Visible => Visibility::Hidden,
        }
    }

    pub fn first_visibility(&self) -> Visibility {
        self.first_visibility
    }
}
