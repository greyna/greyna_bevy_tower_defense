use std::time::Duration;

use bevy::prelude::*;

pub struct Cooldown {
    timer: Timer,
}

impl Cooldown {
    pub fn new(duration: f32) -> Self {
        let mut timer = Timer::from_seconds(duration, TimerMode::Once);
        timer.tick(Duration::from_secs_f32(duration + 1.0));
        Self { timer }
    }

    pub fn tick(&mut self, time: &Time) {
        self.timer.tick(time.delta());
    }

    pub fn ready(&self) -> bool {
        self.timer.finished()
    }

    pub fn start(&mut self) {
        self.timer.reset();
    }

    pub fn set_duration(&mut self, duration: f32) {
        self.timer = Timer::from_seconds(duration, TimerMode::Once);
        self.timer.tick(Duration::from_secs_f32(duration + 1.0));
    }
}

pub fn any_added_component_condition<T: Component>(
) -> impl FnMut(Query<(), Added<T>>) -> bool + Clone {
    move |query: Query<(), Added<T>>| !query.is_empty()
}
