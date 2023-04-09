use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::{egui::Align2, EguiContexts};

use crate::{ui::my_window, AppState};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(display_menu.in_set(OnUpdate(AppState::Menu)));
    }
}

fn display_menu(mut contexts: EguiContexts, main_window: Query<&Window, With<PrimaryWindow>>) {
    my_window(
        "Side Tower Defense for Bevy Game Jam #3",
        Vec2::default(),
        &main_window,
    )
    .title_bar(true)
    .anchor(Align2::CENTER_TOP, [0.0, 100.0])
    .show(contexts.ctx_mut(), |ui| {
        ui.label("Made by greyna for Bevy Game Jam #3");
    });

    my_window("Menu", Vec2::default(), &main_window)
        .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
        .show(contexts.ctx_mut(), |ui| {
            ui.heading("Press space to restart.");
        });
}
