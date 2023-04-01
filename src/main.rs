mod blinking;
mod collisions;
mod game;
mod turret;
mod utils;

use bevy::app::AppExit;
use bevy::prelude::*;

use blinking::BlinkingPlugin;
use collisions::CollisionsPlugin;
use game::GamePlugin;
use turret::TurretPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(TurretPlugin)
        .add_plugin(BlinkingPlugin)
        .add_plugin(CollisionsPlugin)
        .add_plugin(GamePlugin)
        .add_system(exit_game)
        .run();
}

pub fn exit_game(input: Res<Input<KeyCode>>, mut app_exit_sender: EventWriter<AppExit>) {
    if input.just_pressed(KeyCode::Escape) {
        app_exit_sender.send(AppExit);
    }
}
