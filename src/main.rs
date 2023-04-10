mod game;
mod menu;
mod ui;

use crate::menu::MenuPlugin;
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use game::GamePlugin;

fn main() {
    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    println!("Game started");

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_state::<AppState>()
        .add_plugin(GamePlugin)
        .add_plugin(MenuPlugin)
        .add_systems((toggle_menu, exit_game))
        .run();
}

fn exit_game(input: Res<Input<KeyCode>>, mut app_exit_sender: EventWriter<AppExit>) {
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

fn toggle_menu(
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
