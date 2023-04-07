mod game;

use bevy::app::AppExit;
use bevy::prelude::*;

use game::GamePlugin;

fn main() {
    println!("Game started");

    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_plugin(GamePlugin)
        .add_systems((toggle_menu, exit_game))
        .run();
}

pub fn exit_game(input: Res<Input<KeyCode>>, mut app_exit_sender: EventWriter<AppExit>) {
    if input.just_pressed(KeyCode::Escape) {
        app_exit_sender.send(AppExit);
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    Menu,
    #[default]
    Game,
}

pub fn toggle_menu(
    input: Res<Input<KeyCode>>,
    current_state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if input.just_pressed(KeyCode::Space) {
        next_state.set(if current_state.0 == AppState::Menu {
            println!("Game started");
            AppState::Game
        } else {
            println!("In menu");
            AppState::Menu
        });
    }
}
