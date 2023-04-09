use super::{enemies::resources::Lives, schedule::GameSet};
use crate::{ui::my_window, AppState};
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::{egui::Align2, EguiContexts};

pub struct GoldPlugin;

impl Plugin for GoldPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(gain_gold.in_set(GameSet::Logic))
            .add_system(ui.in_set(GameSet::Depiction))
            .add_event::<EnemyKilled>()
            .add_system(init_gold.in_schedule(OnEnter(AppState::Game)));
    }
}

pub fn ui(
    gold: Res<Gold>,
    lives: Res<Lives>,
    mut contexts: EguiContexts,
    main_window: Query<&Window, With<PrimaryWindow>>,
) {
    const LATERAL_OFFSET: f32 = 115.0;

    my_window("Gold", Vec2::default(), &main_window)
        .anchor(Align2::CENTER_TOP, [-LATERAL_OFFSET, 10.0])
        .show(contexts.ctx_mut(), |ui| {
            ui.heading(format!("Gold: {}", gold.0));
        });

    my_window("Lives", Vec2::default(), &main_window)
        .anchor(Align2::CENTER_TOP, [-7.5, 10.0])
        .show(contexts.ctx_mut(), |ui| {
            ui.heading(format!("Lives: {}", lives.0));
        });

    my_window("Score", Vec2::default(), &main_window)
        .anchor(Align2::CENTER_TOP, [LATERAL_OFFSET, 10.0])
        .show(contexts.ctx_mut(), |ui| {
            ui.heading(format!("Score: TODO{}", 0));
        });
}

pub fn gain_gold(mut enemy_killed_receiver: EventReader<EnemyKilled>, mut gold: ResMut<Gold>) {
    const GOLD_PER_ENEMY_KILL: u32 = 20;
    for _ in enemy_killed_receiver.iter() {
        gold.0 += GOLD_PER_ENEMY_KILL;
    }
}

pub fn init_gold(mut commands: Commands) {
    let gold = Gold(2500);
    println!("You have {} gold.", gold.0);
    commands.insert_resource(gold);
}

#[derive(Resource)]
pub struct Gold(pub u32);

pub struct EnemyKilled;
