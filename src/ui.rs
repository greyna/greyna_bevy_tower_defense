use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::egui::{self, Align2};

pub fn my_window<'a>(
    id: &'a str,
    pos: Vec2,
    main_window: &Query<&Window, With<PrimaryWindow>>,
) -> egui::Window<'a> {
    let main_window = main_window.single();

    let offset = [
        pos.x - (main_window.width() / 2.0),
        -pos.y + (main_window.height() / 2.0),
    ];

    egui::Window::new(id)
        .title_bar(false)
        .collapsible(false)
        .resizable(false)
        .hscroll(false)
        .vscroll(false)
        .movable(false)
        .anchor(Align2::CENTER_CENTER, offset)
}
