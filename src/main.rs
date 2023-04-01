mod game;

use bevy::app::AppExit;
use bevy::prelude::*;

use game::GamePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .add_system(exit_game)
        .run();
}

pub fn exit_game(input: Res<Input<KeyCode>>, mut app_exit_sender: EventWriter<AppExit>) {
    if input.just_pressed(KeyCode::Escape) {
        app_exit_sender.send(AppExit);
    }
}
